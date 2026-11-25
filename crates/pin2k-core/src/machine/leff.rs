//! Authentic `Leff*` lamp and flasher choreography engine lifted from `swe1_0200.elf`
//! (`LeffRampMade`, `LeffMainMultiballStart`, `LeffMainMultiballJackpot`, `LeffJediMultiballClash`,
//! `LeffJediMultiballFinish`, `LeffSkillShotAward`, `LeffSuperJetsStart`, `LeffGenericShieldHit`,
//! `LeffLaserFired`, `Class*LeffHit`, `LeffC3P0Complete`, `LeffExtraBallCollected`, `LeffBonus`).

use pin2k_hal::{AudioBackend, LampState, PinballHardware, VideoBackend};
use serde::{Deserialize, Serialize};

use super::{MachineState, Pin2kMachine};
use crate::rfm_rules::GameTitle;
use crate::swe1_tables::{drive, lamp};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LeffKind {
    RampMade { right: bool },
    MainMultiballStart,
    MainMultiballJackpot,
    JediMultiballClash,
    JediMultiballFinish,
    SkillShotAward,
    SuperJetsStart,
    ShieldHit,
    LaserFired { right: bool },
    SceneHit,
    SceneComplete,
    C3P0Complete,
    ExtraBallCollected,
    BonusShow,
}

impl LeffKind {
    pub const fn duration_ms(self) -> u32 {
        match self {
            LeffKind::RampMade { .. } => 900,
            LeffKind::MainMultiballStart => 2_400,
            LeffKind::MainMultiballJackpot => 1_600,
            LeffKind::JediMultiballClash => 1_000,
            LeffKind::JediMultiballFinish => 2_200,
            LeffKind::SkillShotAward => 1_200,
            LeffKind::SuperJetsStart => 1_400,
            LeffKind::ShieldHit => 450,
            LeffKind::LaserFired { .. } => 350,
            LeffKind::SceneHit => 500,
            LeffKind::SceneComplete => 1_800,
            LeffKind::C3P0Complete => 1_600,
            LeffKind::ExtraBallCollected => 1_600,
            LeffKind::BonusShow => 2_000,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActiveLeff {
    pub kind: LeffKind,
    pub elapsed_ms: u32,
    pub duration_ms: u32,
}

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    /// Starts a timed ROM lamp/flasher choreography (`Leff*`) and pulses its initial flasher beat.
    pub fn trigger_leff(&mut self, kind: LeffKind) {
        self.active_leff = Some(ActiveLeff {
            kind,
            elapsed_ms: 0,
            duration_ms: kind.duration_ms(),
        });
        if !self.coin_door_closed {
            return;
        }
        match kind {
            LeffKind::RampMade { right } => {
                self.hw.fire_coil(drive::UPPER_HOTDOG_FL, 24);
                let fl = if right {
                    drive::BACK_RIGHT_UPPER_FL
                } else {
                    drive::BACK_LEFT_UPPER_FL
                };
                self.hw.fire_coil(fl, 24);
            }
            LeffKind::MainMultiballStart | LeffKind::MainMultiballJackpot => {
                self.hw.fire_coil(drive::BACK_MIDDLE_FL, 32);
                self.hw.fire_coil(drive::UPPER_HOTDOG_FL, 32);
                self.hw.fire_coil(drive::LIGHTSABER_NEON, 96);
            }
            LeffKind::JediMultiballClash | LeffKind::JediMultiballFinish => {
                self.hw.fire_coil(drive::LEFT_LASER_FL, 32);
                self.hw.fire_coil(drive::RIGHT_LASER_FL, 32);
                self.hw.fire_coil(drive::BACK_MIDDLE_FL, 32);
            }
            LeffKind::SkillShotAward | LeffKind::SuperJetsStart => {
                self.hw.fire_coil(drive::JET_FLASHER, 32);
                self.hw.fire_coil(drive::UPPER_HOTDOG_FL, 32);
            }
            LeffKind::ShieldHit | LeffKind::SceneHit => {
                self.hw.fire_coil(drive::BACK_MIDDLE_FL, 24);
            }
            LeffKind::LaserFired { right } => {
                self.hw.fire_coil(
                    if right { drive::RIGHT_LASER_FL } else { drive::LEFT_LASER_FL },
                    32,
                );
                self.hw.fire_coil(
                    if right { drive::RIGHT_INLANE_FL } else { drive::LEFT_INLANE_FL },
                    24,
                );
            }
            LeffKind::SceneComplete | LeffKind::C3P0Complete | LeffKind::ExtraBallCollected => {
                self.hw.fire_coil(drive::BACK_LEFT_MID_FL, 24);
                self.hw.fire_coil(drive::BACK_RIGHT_MID_FL, 24);
                self.hw.fire_coil(drive::LIGHTSABER_NEON, 128);
            }
            LeffKind::BonusShow => {
                self.hw.fire_coil(drive::BACK_MIDDLE_FL, 24);
            }
        }
    }

    pub(super) fn tick_leff(&mut self, dt_ms: u32) {
        let Some(mut leff) = self.active_leff else {
            return;
        };
        let prev_beat = leff.elapsed_ms / 160;
        leff.elapsed_ms = leff.elapsed_ms.saturating_add(dt_ms);
        let new_beat = leff.elapsed_ms / 160;

        if self.coin_door_closed && new_beat != prev_beat {
            match leff.kind {
                LeffKind::MainMultiballStart | LeffKind::MainMultiballJackpot => {
                    let fl = if new_beat % 2 == 0 {
                        drive::BACK_LEFT_UPPER_FL
                    } else {
                        drive::BACK_RIGHT_UPPER_FL
                    };
                    self.hw.fire_coil(fl, 24);
                }
                LeffKind::JediMultiballClash => {
                    let fl = if new_beat % 2 == 0 {
                        drive::LEFT_LASER_FL
                    } else {
                        drive::RIGHT_LASER_FL
                    };
                    self.hw.fire_coil(fl, 24);
                }
                LeffKind::SuperJetsStart | LeffKind::SkillShotAward => {
                    self.hw.fire_coil(drive::JET_FLASHER, 24);
                }
                _ => {}
            }
        }

        if leff.elapsed_ms >= leff.duration_ms {
            self.active_leff = None;
        } else {
            self.active_leff = Some(leff);
        }
    }

    /// Overlays active `Leff*` choreography patterns onto the PDB dual 8×8 lamp matrix.
    pub(super) fn apply_leff_lamps(&mut self) {
        if self.game_title != GameTitle::Swe1 || self.state != MachineState::InGame {
            return;
        }
        let Some(leff) = self.active_leff else {
            return;
        };
        let step = (leff.elapsed_ms / 80) as usize;

        match leff.kind {
            LeffKind::RampMade { .. } => {
                for (i, &lid) in lamp::SHIELD_LAMPS.iter().enumerate() {
                    if i % 3 == step % 3 {
                        self.hw.set_lamp(lid, LampState::On);
                    }
                }
                let jedi_idx = step % lamp::JEDI_LETTERS.len();
                self.hw.set_lamp(lamp::JEDI_LETTERS[jedi_idx], LampState::On);
            }
            LeffKind::MainMultiballStart | LeffKind::MainMultiballJackpot => {
                let even = step % 2 == 0;
                for (i, &lid) in lamp::SHIELD_LAMPS.iter().enumerate() {
                    self.hw.set_lamp(
                        lid,
                        if (i % 2 == 0) == even { LampState::On } else { LampState::Off },
                    );
                }
                for (i, &lid) in lamp::NABOO_SHIP_BODY.iter().enumerate() {
                    if (i + step) % 2 == 0 {
                        self.hw.set_lamp(lid, LampState::On);
                    }
                }
                self.hw.set_lamp(lamp::LEFT_SAUCER_INSERT, LampState::On);
                self.hw.set_lamp(lamp::RIGHT_SAUCER_INSERT, LampState::On);
            }
            LeffKind::JediMultiballClash | LeffKind::JediMultiballFinish => {
                for &lid in &lamp::JEDI_LETTERS {
                    self.hw.set_lamp(
                        lid,
                        if step % 2 == 0 { LampState::On } else { LampState::Off },
                    );
                }
                let jedi_rank = [
                    lamp::JEDI_YOUTH,
                    lamp::JEDI_KNIGHT,
                    lamp::JEDI_MASTER,
                    lamp::JEDI_SPIRIT,
                ];
                self.hw.set_lamp(jedi_rank[step % jedi_rank.len()], LampState::On);
            }
            LeffKind::SkillShotAward => {
                self.hw.set_lamp(lamp::JETS_ROLLOVER, LampState::On);
                self.hw.set_lamp(lamp::MIDDLE_JET, LampState::On);
                let idx = step % lamp::NABOO_SHIP_BODY.len();
                self.hw.set_lamp(lamp::NABOO_SHIP_BODY[idx], LampState::On);
            }
            LeffKind::SuperJetsStart => {
                self.hw.set_lamp(lamp::MIDDLE_JET, LampState::On);
                self.hw.set_lamp(lamp::JETS_ROLLOVER, LampState::On);
                let idx = step % lamp::SHIELD_LAMPS.len();
                self.hw.set_lamp(lamp::SHIELD_LAMPS[idx], LampState::On);
            }
            LeffKind::ShieldHit | LeffKind::SceneHit => {
                for &lid in &lamp::SHIELD_LAMPS[0..6] {
                    if step % 2 == 0 {
                        self.hw.set_lamp(lid, LampState::On);
                    }
                }
            }
            LeffKind::LaserFired { right } => {
                let bank = if right {
                    &lamp::RIGHT_STANDUPS
                } else {
                    &lamp::LEFT_STANDUPS
                };
                for &lid in bank {
                    self.hw.set_lamp(lid, LampState::On);
                }
            }
            LeffKind::SceneComplete | LeffKind::BonusShow => {
                let fill = (step % lamp::NABOO_SHIP_BODY.len()) + 1;
                for &lid in &lamp::NABOO_SHIP_BODY[..fill] {
                    self.hw.set_lamp(lid, LampState::On);
                }
            }
            LeffKind::C3P0Complete | LeffKind::ExtraBallCollected => {
                self.hw.set_lamp(
                    lamp::EXTRA_BALL,
                    if step % 2 == 0 { LampState::On } else { LampState::Off },
                );
                self.hw.set_lamp(
                    lamp::SHOOT_AGAIN,
                    if step % 2 == 1 { LampState::On } else { LampState::Off },
                );
            }
        }
    }
}
