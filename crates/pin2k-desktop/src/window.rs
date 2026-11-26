//! Native 60 Hz desktop window (`--window` / `--fullscreen`) using `winit` + `softbuffer`.
//! Preserves 4:3 Pinball 2000 aspect ratio with integer/letterboxed scaling and `F11` / `Alt+Enter`
//! fullscreen toggle, with zero external C library dependencies.

use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::{Duration, Instant};

use anyhow::Result;
use softbuffer::{Context, Surface};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, ModifiersState, PhysicalKey};
use winit::window::{Fullscreen, Window, WindowAttributes, WindowId};

use crate::cli::DisplayFilter;

pub(crate) fn run_windowed<F>(
    title: &str,
    src_w: u32,
    src_h: u32,
    start_fullscreen: bool,
    display_filter: DisplayFilter,
    mut tick_frame: F,
) -> Result<()>
where
    F: FnMut(&[(u8, bool)]) -> Option<Vec<u8>>,
{
    let event_loop = EventLoop::new()?;
    let mut app = Pin2kWindowApp {
        title: title.to_string(),
        src_w: src_w.max(1),
        src_h: src_h.max(1),
        start_fullscreen,
        display_filter,
        window: None,
        surface: None,
        modifiers: ModifiersState::empty(),
        next_frame_at: Instant::now(),
        last_rgba: vec![0u8; (src_w.max(1) * src_h.max(1) * 4) as usize],
        pending_switches: Vec::new(),
        tick_frame: &mut tick_frame,
    };
    event_loop.run_app(&mut app)?;
    Ok(())
}

struct Pin2kWindowApp<'a, F> {
    title: String,
    src_w: u32,
    src_h: u32,
    start_fullscreen: bool,
    display_filter: DisplayFilter,
    window: Option<Rc<Window>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    modifiers: ModifiersState,
    next_frame_at: Instant,
    last_rgba: Vec<u8>,
    pending_switches: Vec<(u8, bool)>,
    tick_frame: &'a mut F,
}

impl<F> ApplicationHandler for Pin2kWindowApp<'_, F>
where
    F: FnMut(&[(u8, bool)]) -> Option<Vec<u8>>,
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let mut attrs = WindowAttributes::default()
            .with_title(&self.title)
            .with_inner_size(LogicalSize::new(
                self.src_w.clamp(960, 1280),
                self.src_h.clamp(720, 960),
            ))
            .with_min_inner_size(LogicalSize::new(640, 480));
        if self.start_fullscreen {
            attrs = attrs.with_fullscreen(Some(Fullscreen::Borderless(None)));
        }
        let win = Rc::new(
            event_loop
                .create_window(attrs)
                .expect("failed to create native Pinball 2000 window"),
        );
        let ctx = Context::new(Rc::clone(&win)).expect("failed to init softbuffer context");
        let surf = Surface::new(&ctx, Rc::clone(&win)).expect("failed to init softbuffer surface");
        self.window = Some(win);
        self.surface = Some(surf);
        self.next_frame_at = Instant::now();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::ModifiersChanged(m) => self.modifiers = m.state(),
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    physical_key: PhysicalKey::Code(code),
                    state,
                    repeat: false,
                    ..
                },
                ..
            } => {
                let pressed = state == ElementState::Pressed;
                if pressed
                    && (code == KeyCode::F11
                        || (code == KeyCode::Enter && self.modifiers.alt_key()))
                {
                    if let Some(ref win) = self.window {
                        let fs = if win.fullscreen().is_some() {
                            None
                        } else {
                            Some(Fullscreen::Borderless(None))
                        };
                        win.set_fullscreen(fs);
                    }
                    return;
                }
                if let Some(sw_id) = map_keycode_to_switch(code) {
                    self.pending_switches.push((sw_id, pressed));
                }
            }
            WindowEvent::RedrawRequested => {
                self.present_rgba();
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let now = Instant::now();
        if now >= self.next_frame_at {
            self.next_frame_at = now + Duration::from_nanos(16_666_667);
            let switches = std::mem::take(&mut self.pending_switches);
            match (self.tick_frame)(&switches) {
                Some(rgba) => {
                    self.last_rgba = rgba;
                    if let Some(ref win) = self.window {
                        win.request_redraw();
                    }
                }
                None => {
                    event_loop.exit();
                    return;
                }
            }
        }
        event_loop.set_control_flow(ControlFlow::WaitUntil(self.next_frame_at));
    }
}

impl<F> Pin2kWindowApp<'_, F> {
    fn present_rgba(&mut self) {
        let (Some(win), Some(surf)) = (self.window.as_ref(), self.surface.as_mut()) else {
            return;
        };
        let size = win.inner_size();
        let (Some(w), Some(h)) = (NonZeroU32::new(size.width), NonZeroU32::new(size.height)) else {
            return;
        };
        if surf.resize(w, h).is_err() {
            return;
        }
        let Ok(mut buf) = surf.buffer_mut() else {
            return;
        };
        let dst_w = size.width as usize;
        let dst_h = size.height as usize;
        buf.fill(0);

        // Aspect-preserved 4:3 letterboxed nearest-neighbor blit
        let (vp_w, vp_h) = if dst_w * 3 >= dst_h * 4 {
            ((dst_h * 4) / 3, dst_h)
        } else {
            (dst_w, (dst_w * 3) / 4)
        };
        let ox = (dst_w.saturating_sub(vp_w)) / 2;
        let oy = (dst_h.saturating_sub(vp_h)) / 2;

        let sw = self.src_w as usize;
        let sh = self.src_h as usize;
        if self.display_filter == DisplayFilter::Nearest {
            for dy in 0..vp_h {
                let sy = (dy * sh) / vp_h.max(1);
                let dst_row = (oy + dy) * dst_w + ox;
                let src_row = sy * sw;
                for dx in 0..vp_w {
                    let sx = (dx * sw) / vp_w.max(1);
                    let p = (src_row + sx) * 4;
                    if p + 2 < self.last_rgba.len() {
                        let r = self.last_rgba[p] as u32;
                        let g = self.last_rgba[p + 1] as u32;
                        let b = self.last_rgba[p + 2] as u32;
                        buf[dst_row + dx] = (r << 16) | (g << 8) | b;
                    }
                }
            }
        } else {
            let crt = self.display_filter == DisplayFilter::Crt;
            let sw_f = (sw.saturating_sub(1)) as f32;
            let sh_f = (sh.saturating_sub(1)) as f32;
            for dy in 0..vp_h {
                let fy = (dy as f32 * sh_f) / vp_h.saturating_sub(1).max(1) as f32;
                let y0 = (fy as usize).min(sh.saturating_sub(1));
                let y1 = (y0 + 1).min(sh.saturating_sub(1));
                let wy = ((fy - y0 as f32) * 256.0) as u32;
                let inv_wy = 256 - wy;
                let scan_mul = if crt && (dy & 1) == 1 { 205u32 } else { 256u32 };
                let dst_row = (oy + dy) * dst_w + ox;
                for dx in 0..vp_w {
                    let fx = (dx as f32 * sw_f) / vp_w.saturating_sub(1).max(1) as f32;
                    let x0 = (fx as usize).min(sw.saturating_sub(1));
                    let x1 = (x0 + 1).min(sw.saturating_sub(1));
                    let wx = ((fx - x0 as f32) * 256.0) as u32;
                    let inv_wx = 256 - wx;
                    let p00 = (y0 * sw + x0) * 4;
                    let p10 = (y0 * sw + x1) * 4;
                    let p01 = (y1 * sw + x0) * 4;
                    let p11 = (y1 * sw + x1) * 4;
                    if p11 + 2 < self.last_rgba.len() {
                        let w00 = inv_wx * inv_wy;
                        let w10 = wx * inv_wy;
                        let w01 = inv_wx * wy;
                        let w11 = wx * wy;
                        let sample = |c: usize| -> u32 {
                            let v = (self.last_rgba[p00 + c] as u32 * w00
                                + self.last_rgba[p10 + c] as u32 * w10
                                + self.last_rgba[p01 + c] as u32 * w01
                                + self.last_rgba[p11 + c] as u32 * w11)
                                >> 16;
                            (v * scan_mul) >> 8
                        };
                        buf[dst_row + dx] = (sample(0) << 16) | (sample(1) << 8) | sample(2);
                    }
                }
            }
        }
        let _ = buf.present();
    }
}

fn map_keycode_to_switch(code: KeyCode) -> Option<u8> {
    match code {
        KeyCode::KeyC => Some(64),      // Left Coin
        KeyCode::KeyS => Some(2),       // Start Button
        KeyCode::KeyZ => Some(85),      // Left Flipper
        KeyCode::KeyM => Some(84),      // Right Flipper
        KeyCode::KeyX => Some(87),      // Left Laser Action
        KeyCode::Period => Some(86),    // Right Laser Action
        KeyCode::KeyL => Some(36),      // Left Ramp Made
        KeyCode::KeyR => Some(37),      // Right Ramp Made
        KeyCode::KeyH => Some(34),      // Deflector Shield Target
        KeyCode::KeyP => Some(32),      // Center Shield Popper
        KeyCode::KeyK => Some(22),      // Left Saucer
        KeyCode::KeyJ => Some(45),      // Jets Rollover
        KeyCode::Digit1 => Some(72),    // Diag Escape
        KeyCode::Digit2 => Some(73),    // Diag Down
        KeyCode::Digit3 => Some(74),    // Diag Up
        KeyCode::Digit4 => Some(75),    // Diag Enter
        _ => None,
    }
}
