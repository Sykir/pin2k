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

pub(crate) fn run_windowed<F>(
    title: &str,
    src_w: u32,
    src_h: u32,
    start_fullscreen: bool,
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
            .with_inner_size(LogicalSize::new(self.src_w.max(960), self.src_h.max(720)))
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

        for dy in 0..vp_h {
            let sy = (dy * self.src_h as usize) / vp_h.max(1);
            let dst_row = (oy + dy) * dst_w + ox;
            let src_row = sy * self.src_w as usize;
            for dx in 0..vp_w {
                let sx = (dx * self.src_w as usize) / vp_w.max(1);
                let p = (src_row + sx) * 4;
                if p + 2 < self.last_rgba.len() {
                    let r = self.last_rgba[p] as u32;
                    let g = self.last_rgba[p + 1] as u32;
                    let b = self.last_rgba[p + 2] as u32;
                    buf[dst_row + dx] = (r << 16) | (g << 8) | b;
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
