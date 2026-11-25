use pin2k_core::swe1_tables::{lamp, SWE1_DRIVES};
use pin2k_hal::LampState;

use crate::compositor::SoftwareCompositor;
use crate::raster::{draw_rect_outline, draw_string, fill_rect};

impl SoftwareCompositor {
    pub fn draw_playfield_telemetry(
        &mut self,
        lamps: &[LampState; 128],
        drive_mask: u64,
        pdb_present: bool,
        coin_door_closed: bool,
        credits: u8,
        ball: u8,
        jedi_rank: &str,
        lasers: u8,
        elapsed_ms: u64,
    ) {
        // Playfield frame box (`x: 20..360, y: 252..470`)
        draw_rect_outline(&mut self.framebuffer, 20, 252, 340, 218, [45, 95, 160, 255]);
        draw_string(&mut self.framebuffer, "SWE1 PLAYFIELD LAMPS & COILS", 28, 258, [130, 200, 255, 255], 1);

        // Plot real SWE1 lamps using normalized coords
        let lamp_coords: &[(u8, u8, u8, [u8; 4])] = &[
            (lamp::JEDI_J, 46, 36, [80, 255, 120, 255]),
            (lamp::JEDI_E, 46, 41, [80, 255, 120, 255]),
            (lamp::JEDI_D, 46, 46, [80, 255, 120, 255]),
            (lamp::JEDI_I, 46, 51, [80, 255, 120, 255]),
            (lamp::JEDI_YOUTH, 33, 62, [100, 220, 255, 255]),
            (lamp::JEDI_KNIGHT, 40, 62, [100, 220, 255, 255]),
            (lamp::JEDI_MASTER, 53, 63, [100, 220, 255, 255]),
            (lamp::JEDI_SPIRIT, 58, 61, [255, 240, 90, 255]),
            (lamp::SHOOT_AGAIN, 46, 90, [255, 120, 60, 255]),
            (lamp::EXTRA_BALL, 78, 44, [255, 180, 50, 255]),
            (lamp::FIRE_LASERS_LEFT, 38, 56, [255, 70, 70, 255]),
            (lamp::FIRE_LASERS_RIGHT, 56, 56, [255, 70, 70, 255]),
        ];

        // Shield bank lamps
        for (i, &lid) in lamp::SHIELD_LAMPS.iter().enumerate() {
            let lx = 38 + ((i % 4) as i32) * 6;
            let ly = 22 + ((i / 4) as i32) * 4;
            let lit = is_lamp_visible(lamps[lid as usize], elapsed_ms);
            let col = if lit { [0, 230, 255, 255] } else { [25, 40, 55, 255] };
            let px = 25 + (lx * 320 / 100);
            let py = 270 + (ly * 190 / 100);
            fill_rect(&mut self.framebuffer, px, py, 5, 5, col);
        }

        for &(lid, lx, ly, active_col) in lamp_coords {
            let lit = is_lamp_visible(lamps[lid as usize], elapsed_ms);
            let col = if lit { active_col } else { [30, 35, 50, 255] };
            let px = 25 + ((lx as i32) * 320 / 100);
            let py = 270 + ((ly as i32) * 190 / 100);
            fill_rect(&mut self.framebuffer, px, py, 6, 6, col);
        }

        // Plot all 33 wired SWE1 Solenoid / Flasher / Flipper / Neon Drives!
        for d in &SWE1_DRIVES {
            let active = ((drive_mask >> d.id) & 1) != 0;
            let px = 25 + ((d.x as i32) * 320 / 100);
            let py = 270 + ((d.y as i32) * 190 / 100);
            let col = if active {
                if d.id == 40 {
                    [50, 255, 90, 255] // Lightsaber Neon green!
                } else if d.is_flasher {
                    [255, 250, 140, 255]
                } else {
                    [255, 65, 65, 255]
                }
            } else {
                [45, 25, 30, 255]
            };
            let sz = if active { 7 } else { 4 };
            fill_rect(&mut self.framebuffer, px, py, sz, sz, col);
        }

        // Right-side PDB Hardware & Rules Telemetry Panel (`x: 375..625, y: 252..470`)
        draw_rect_outline(&mut self.framebuffer, 375, 252, 250, 218, [70, 120, 90, 255]);
        draw_string(&mut self.framebuffer, "PINBALL 2000 PDB STATUS", 385, 260, [120, 255, 160, 255], 1);

        let lines = [
            format!("PDB LPT BUS : {}", if pdb_present { "ONLINE (0x87)" } else { "OPEN BUS" }),
            format!("COIN DOOR   : {}", if coin_door_closed { "CLOSED (HV ON)" } else { "OPEN (HV CUT)" }),
            format!("BALL / CRED : BALL {}  CRED {}", ball, credits),
            format!("JEDI RANK   : {}", jedi_rank),
            format!("LASER AMMO  : {}", lasers),
            format!("DRIVE MASK  : 0x{:012X}", drive_mask),
        ];
        for (i, line) in lines.iter().enumerate() {
            draw_string(
                &mut self.framebuffer,
                line,
                385,
                284 + (i as i32) * 18,
                [215, 230, 240, 255],
                1,
            );
        }

        draw_string(&mut self.framebuffer, "CONTROLS: [C]OIN [S]TART", 385, 408, [255, 215, 90, 255], 1);
        draw_string(&mut self.framebuffer, "[Z/M] FLIP  [X/.] LASERS", 385, 424, [255, 215, 90, 255], 1);
        draw_string(&mut self.framebuffer, "[L/R] RAMPS [P] POPPER", 385, 440, [255, 215, 90, 255], 1);
    }

}

fn is_lamp_visible(state: LampState, elapsed_ms: u64) -> bool {
    match state {
        LampState::Off => false,
        LampState::On => true,
        LampState::Blinking { period_ms } => {
            let p = period_ms.max(60) as u64;
            (elapsed_ms % p) < (p / 2)
        }
        LampState::Pwm(v) => v >= 64,
    }
}

