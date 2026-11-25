use pin2k_hal::{AudioBackend, PinballHardware, VideoBackend};

use super::anims::*;
use crate::machine::Pin2kMachine;
use crate::swe1_rules::{ActiveMultiball, Swe1Scene};

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    // ------------------------------------------------------------------
    // Scene hit / completion / timeout effects (`Deff*SceneBackground`)
    // ------------------------------------------------------------------

    pub(crate) fn scene_hit_fx(&mut self, scene: Swe1Scene, hits_left: u8, laser: bool) {
        let n = hits_left as usize;
        match scene {
            Swe1Scene::BattleDroid => {
                if laser {
                    self.spawn_fx_centered(BDROID_LASER_EXPLOSION, 320, 110, 8);
                } else if n % 3 == 0 {
                    self.spawn_fx(BDROID_STAP_EXPLOSION, 33, 14, 8);
                } else {
                    let cx = if n % 2 == 0 { 67 } else { 547 };
                    self.spawn_fx_centered(BDROID_EXPLOSION, cx, 125, 8);
                }
            }
            Swe1Scene::DestroyerDroid => {
                let (anim, cx, cy) = match n % 3 {
                    0 => (DESTROYER_S_EXPLOSION, 170, 130),
                    1 => (DESTROYER_M_EXPLOSION, 480, 125),
                    _ => (DESTROYER_L_EXPLOSION, 125, 128),
                };
                self.spawn_fx_centered(anim, cx, cy, 8);
                let blast = [SITH_PROBE_BLAST_1, SITH_PROBE_BLAST_2, SITH_PROBE_BLAST_3][n % 3];
                self.spawn_fx_ex(blast, 320 - blast.w / 2, 168, 8, 1, 0, 250);
            }
            Swe1Scene::FederationFighter => {
                let main = match (laser, n % 2) {
                    (true, 0) => FED_LASER_EXPLOSION,
                    (true, _) => FED_LASER_EXPLOSION_2,
                    (false, 0) => FED_EXPLOSION,
                    (false, _) => FED_EXPLOSION_2,
                };
                self.spawn_fx_centered(main, 291, 93, 8);
                let (pop, px) = match n % 3 {
                    0 => (EXPLOSION_4, 120),
                    1 => (EXPLOSION_5, 460),
                    _ => (EXPLOSION_6, 300),
                };
                self.spawn_fx_ex(pop, px, 150, 8, 1, 120, 0);
            }
            Swe1Scene::GunganBattle => {
                self.spawn_fx(GUNGAN_SUB_BUBBLES, 470, 12, 8);
            }
            Swe1Scene::HangarEscape => {
                let anim = if laser {
                    if n % 2 == 0 {
                        HANGAR_DROID_LASER_EXPLOSION
                    } else {
                        HANGAR_DROID_LASER_EXPLOSION_2
                    }
                } else {
                    [
                        HANGAR_DROID_EXPLOSION,
                        HANGAR_DROID_EXPLOSION_2,
                        HANGAR_DROID_EXPLOSION_3,
                        HANGAR_DROID_EXPLOSION_4,
                    ][n % 4]
                };
                self.spawn_fx_centered(anim, 317, 111, 8);
            }
            Swe1Scene::HoverTank => {
                let (anim, cx) = match n % 3 {
                    0 => (HOVER_TANK_L_EXPLODE, 150),
                    1 => (HOVER_TANK_R_EXPLODE, 500),
                    _ => (HOVER_TANK_C_EXPLODE, 320),
                };
                self.spawn_fx_centered(anim, cx, 110, 8);
            }
            Swe1Scene::Podrace => {
                self.spawn_fx_centered(PODRACE_3_EXPLOSION, 360 + (n as i32 % 3) * 60, 150, 8);
            }
            Swe1Scene::ProbeDroid => {
                let anim = if n % 2 == 0 { SITH_PROBE_EXPLOSION_1 } else { SITH_PROBE_EXPLOSION_2 };
                self.spawn_fx_centered(anim, 280, 90, 8);
                let blast = [SITH_PROBE_BLAST_1, SITH_PROBE_BLAST_2, SITH_PROBE_BLAST_3][n % 3];
                self.spawn_fx_ex(blast, 320 - blast.w / 2, 170, 8, 1, 0, 250);
            }
            Swe1Scene::QueenFashion => {
                self.spawn_fx_centered(FASHION_FLASH, 320, 110, 8);
            }
            Swe1Scene::JarJarJuggle => {
                if n % 2 == 0 {
                    self.spawn_fx_centered(JARJAR_JUMPUP, 320, 128, 8);
                } else {
                    self.spawn_fx_ex(JARJAR_SWITCH, 225, 20, 8, 1, 0, 400);
                }
            }
            Swe1Scene::WattosChance => {
                self.spawn_fx_centered(SITH_PROBE_MISS_EXPLOSION, 320, 150, 8);
            }
            Swe1Scene::MosEspaCantina | Swe1Scene::R2D2 => {
                self.spawn_fx_centered(SPARKS_1, 320, 140, 8);
            }
            Swe1Scene::QuestionMark => {
                self.spawn_fx_centered(BLUE_ICON_BOOM_SMALL, 320, 120, 8);
            }
        }
    }

    pub(crate) fn scene_complete_fx(&mut self, scene: Swe1Scene) {
        match scene {
            Swe1Scene::BattleDroid => self.spawn_fx(BDROID_TANK_EXPLOSION, 30, 10, 8),
            Swe1Scene::DestroyerDroid | Swe1Scene::ProbeDroid => self.spawn_fx(EXPLOSION_3, 0, 0, 8),
            Swe1Scene::FederationFighter => {
                self.spawn_fx_centered(FED_EXPLOSION_3, 320, 110, 8);
                self.spawn_fx_ex(EXPLOSION_3, 0, 0, 8, 1, 300, 0);
            }
            Swe1Scene::GunganBattle => self.spawn_fx(GUNGAN_SUB_SANDO, 120, 42, 8),
            Swe1Scene::HangarEscape => self.spawn_fx_ex(HANGAR_ESCAPE_EXIT_MOVIE, 0, 0, 8, 2, 0, 0),
            Swe1Scene::HoverTank => self.spawn_fx_centered(HOVER_TANK_C_EXPLODE, 320, 110, 8),
            Swe1Scene::Podrace => self.spawn_fx_centered(PODRACE_ANAKIN_ZOOM, 320, 120, 8),
            Swe1Scene::QueenFashion => self.spawn_fx_ex(QUEEN_ESCAPE_MISC_MOVIE, 0, 50, 8, 2, 0, 0),
            Swe1Scene::JarJarJuggle => {
                self.spawn_fx_centered(JARJAR_FALL_2, 320, 125, 8);
                self.spawn_fx_ex(JARJAR_THUMBS_UP, 262, 160, 9, 1, 300, 1_500);
            }
            Swe1Scene::WattosChance => self.spawn_fx_centered(WATTO_CUBE_EXPLODE, 320, 150, 8),
            Swe1Scene::MosEspaCantina => self.spawn_fx_ex(JEDI_STAND_MISC_MOVIE, 0, 50, 8, 2, 0, 0),
            Swe1Scene::R2D2 => self.spawn_fx_centered(FIREWORK_2, 320, 120, 8),
            Swe1Scene::QuestionMark => self.spawn_fx_centered(GREEN_ICON_BOOM_BIG, 320, 120, 8),
        }
    }

    pub(crate) fn scene_timeout_fx(&mut self, scene: Swe1Scene) {
        if scene == Swe1Scene::JarJarJuggle {
            self.spawn_fx_centered(JARJAR_FALL, 320, 120, 8);
        }
    }

    /// Hangar Multiball jackpot booms (`anim_jackpot_icon_boom`, blue / green super variants).
    pub(crate) fn jackpot_fx(&mut self, collected: u8, super_jackpot: bool) {
        if super_jackpot {
            self.spawn_fx_centered(GREEN_ICON_BOOM_BIG, 320, 110, 8);
            self.spawn_fx_ex(BLUE_ICON_BOOM_SMALL, 241, 140, 9, 1, 150, 0);
        } else if collected >= 2 {
            self.spawn_fx_centered(BLUE_ICON_BOOM_BIG, 320, 110, 8);
        } else {
            self.spawn_fx_centered(JACKPOT_ICON_BOOM, 320, 110, 8);
        }
    }

    /// `JediMultiball` lost (multiball drained before Maul was defeated).
    pub(crate) fn duel_lost_fx(&mut self) {
        self.spawn_fx_centered(JEDI_BATTLE_QUIGON_DEATH, 320, 120, 8);
        self.spawn_fx_ex(JEDI_BATTLE_WIN_ICON_MAUL, 135, 32, 9, 1, 900, 2_500);
    }

    pub(crate) fn duel_won_fx(&mut self, combo: bool) {
        if combo {
            self.spawn_fx_centered(JEDI_BATTLE_MAUL_DEATH_2, 320, 120, 8);
        }
    }

    /// `DeffBumperSparks::hit` from `JetBumpers::jet_bumper_hit`.
    pub(crate) fn bumper_sparks_fx(&mut self, jet_idx: u8) {
        if self.active_scene.is_some() || self.multiball != ActiveMultiball::None {
            return;
        }
        if jet_idx % 2 == 0 {
            self.spawn_fx(BUMPER_BUBBLES_1, 203, 100, 15);
        } else {
            self.spawn_fx(BUMPER_BUBBLES_2, 241, 100, 15);
        }
    }

    pub(crate) fn spinner_combo_fx(&mut self, right: bool) {
        let x = if right { 421 } else { 43 };
        self.spawn_fx(SPINNER_EXPLOSION, x - 40, 150, 23);
    }

    pub(crate) fn shield_fx(&mut self, raised: bool) {
        let anim = if raised { SHIELD_BLOCK } else { SHIELD_BLOCK_WIPE };
        self.spawn_fx(anim, 220, 198, 8);
    }

    /// First laser charge instruction (`Lasers` -> drop targets + action buttons).
    pub(crate) fn laser_instruction_fx(&mut self) {
        if self.active_scene.is_some() || self.multiball != ActiveMultiball::None {
            return;
        }
        self.spawn_fx_ex(DROP_TARGET, 20, 48, 8, 1, 0, 2_500);
        self.spawn_fx_ex(ACTION_BUTTON, 410, 92, 8, 1, 0, 2_500);
    }

    /// `DeffObiHolo` opening (`anim_obiwan_holo_activate` + `anim_holo_burst`).
    pub(crate) fn holo_open_fx(&mut self) {
        self.spawn_fx_centered(HOLO_BURST, 320, 120, 6);
        self.spawn_fx_centered(OBIWAN_HOLO_ACTIVATE, 320, 100, 7);
    }

    /// `DeffObiHolo` closing when a scene is chosen (`anim_obiwan_holo_flatten`).
    pub(crate) fn holo_close_fx(&mut self) {
        self.spawn_fx_centered(OBIWAN_HOLO_FLATTEN, 320, 120, 205);
    }

}
