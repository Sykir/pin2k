//! Authentic `BattleDroidScene` + `DeffBattleDroidSceneBackground` runtime & renderer (`swe1_0200.elf`):
//! - `SHIELD_HIT` (`0x27`): `400,000` pts, `-2` tank HP (8 total), `tank_hit()` / `tank_defeated()`.
//! - `LEFT_SHIELD_TARGET` (`0x22`) / `RIGHT_SHIELD_TARGET` (`0x23`): `200,000` pts, `-1` tank HP (down to 1).
//! - `RAMP_MADE_LEFT` (`0x1b`) / `RAMP_MADE_RIGHT` (`0x1c`): STAP ramp hit (`stap_ramp_hit`) + `400,000` pts.

use pin2k_hal::{AudioBackend, PinballHardware, VideoBackend};

use super::battle_droid_state::{
    BattleDroidState, DroidSideState, DROID_UNPACK_Y, LEFT_DROID_STAND_X, LEFT_DROID_STAND_Y,
    LEFT_DROID_UNPACK_X, MTT_DOORS_X, MTT_DOORS_Y, MTT_X, MTT_Y, RIGHT_DROID_STAND_X,
    RIGHT_DROID_STAND_Y, RIGHT_DROID_UNPACK_X, STAP_LEFT_X, STAP_LEFT_Y, STAP_RIGHT_X,
    STAP_RIGHT_Y, TANK_EXPLOSION_X, TANK_EXPLOSION_Y,
};
use crate::machine::{LeffKind, Pin2kMachine};
use crate::swe1_rules::Swe1Scene;
use crate::swe1_tables::sw;

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    pub(crate) fn start_battle_droid_state(&mut self) {
        self.battle_droid = Some(BattleDroidState::new());
    }

    pub(crate) fn tick_battle_droid(&mut self, dt_ms: u32) {
        let active_gameplay = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::BattleDroid && s.intro_remaining_ms == 0);
        if !active_gameplay {
            return;
        }
        let mut emit_ards: Vec<&'static str> = Vec::new();
        if let Some(ref mut bd) = self.battle_droid {
            bd.intro_elapsed_ms = bd.intro_elapsed_ms.saturating_add(dt_ms);
            bd.tank_hit_flash_ms = bd.tank_hit_flash_ms.saturating_sub(dt_ms);
            bd.tank_defeated_ms = bd.tank_defeated_ms.saturating_sub(dt_ms);
            bd.stap_exploding_ms = bd.stap_exploding_ms.saturating_sub(dt_ms);
            bd.stap_combo_window_ms = bd.stap_combo_window_ms.saturating_sub(dt_ms);
            bd.stap_zap_ms = bd.stap_zap_ms.saturating_add(dt_ms) % 1_600;

            for (side, ms, ard) in [
                (&mut bd.left_droid, &mut bd.left_droid_ms, "ard_bdroid_laser_l"),
                (&mut bd.right_droid, &mut bd.right_droid_ms, "ard_bdroid_laser_r"),
            ] {
                *ms = ms.saturating_add(dt_ms);
                match *side {
                    DroidSideState::Unpacking if *ms >= 250 => {
                        *side = DroidSideState::Standing;
                        *ms = 0;
                    }
                    DroidSideState::Standing if *ms >= 1_900 => {
                        *side = DroidSideState::Shooting;
                        *ms = 0;
                        emit_ards.push(ard);
                    }
                    DroidSideState::Shooting if *ms >= 500 => {
                        *side = DroidSideState::Standing;
                        *ms = 0;
                    }
                    DroidSideState::Exploding if *ms >= 350 => {
                        *side = DroidSideState::Standing;
                        *ms = 0;
                    }
                    _ => {}
                }
            }
        }
        for ard in emit_ards {
            self.emit_ard(ard);
        }
    }

    /// Handles authentic `BattleDroidScene` switches (`0x27`, `0x22`, `0x23`, `0x1b`, `0x1c`).
    pub(crate) fn handle_battle_droid_switch(&mut self, sw_id: u8) -> bool {
        let is_bd = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::BattleDroid);
        if !is_bd {
            return false;
        }
        if self.battle_droid.is_none() {
            self.start_battle_droid_state();
        }
        match sw_id {
            sw::SHIELD_HIT | sw::LEFT_SHIELD_TARGET | sw::RIGHT_SHIELD_TARGET => {
                let is_center = sw_id == sw::SHIELD_HIT;
                let pts = if is_center { 400_000 } else { 200_000 };
                self.add_score(pts);
                self.trigger_leff(LeffKind::ShieldHit);
                let mut completed = false;
                let mut speech = false;
                if let Some(ref mut bd) = self.battle_droid {
                    if is_center {
                        bd.tank_hits_remaining = bd.tank_hits_remaining.saturating_sub(2);
                    } else if bd.tank_hits_remaining > 1 {
                        bd.tank_hits_remaining -= 1;
                    }
                    bd.tank_hits_landed = 8u8.saturating_sub(bd.tank_hits_remaining);
                    if bd.tank_hits_remaining == 0 {
                        completed = true;
                        bd.tank_defeated_ms = 500;
                    } else {
                        bd.tank_hit_flash_ms = 500;
                        bd.speech_counter = bd.speech_counter.wrapping_add(1);
                        speech = (bd.speech_counter & 3) == 3;
                    }
                }
                if completed {
                    self.complete_battle_droid_scene();
                } else {
                    if self
                        .battle_droid
                        .as_ref()
                        .is_some_and(|b| b.tank_hits_remaining < 3)
                    {
                        self.set_shield_raised(true);
                    }
                    self.emit_ard("ard_battle_droid_scene_tank_hit");
                    if speech {
                        self.emit_ard("ard_hit_bdroids_speech");
                    }
                    if let Some(ref mut sc) = self.active_scene {
                        sc.hits_remaining = self
                            .battle_droid
                            .as_ref()
                            .map_or(1, |b| b.tank_hits_remaining.div_ceil(2).max(1));
                    }
                }
                true
            }
            sw::RAMP_MADE_LEFT | sw::RAMP_MADE_RIGHT => {
                let is_right = sw_id == sw::RAMP_MADE_RIGHT;
                self.trigger_stap_hit(is_right, false);
                true
            }
            _ => false,
        }
    }

    /// Handles `BattleDroidScene::hook_left_laser_fired` / `hook_right_laser_fired` (`stap_ramp_hit(side, 1)`).
    pub(crate) fn handle_battle_droid_laser(&mut self, right: bool) -> bool {
        let is_bd = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::BattleDroid);
        if !is_bd {
            return false;
        }
        if self.battle_droid.is_none() {
            self.start_battle_droid_state();
        }
        self.trigger_stap_hit(right, true);
        true
    }

    /// Handles `LEFT_DROP_TARGET` / `RIGHT_DROP_TARGET` during `BattleDroidScene` (100,000 pts, counts as miss).
    pub(crate) fn handle_battle_droid_drop_target(&mut self, right: bool) -> bool {
        if !self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::BattleDroid)
        {
            return false;
        }
        if self.battle_droid.is_none() {
            self.start_battle_droid_state();
        }
        if let Some(ref mut bd) = self.battle_droid {
            bd.staps_destroyed = bd.staps_destroyed.saturating_add(1);
            bd.explode_side_droid(right);
        }
        self.add_score(100_000);
        self.emit_ard("ard_battle_droid_scene_droid_boom");
        self.set_banner("DROID HIT! +100,000", 1_500);
        true
    }

    fn trigger_stap_hit(&mut self, is_right: bool, is_laser: bool) {
        let pts = self
            .battle_droid
            .as_mut()
            .map_or(200_000, |bd| bd.record_stap_hit(is_right, is_laser));
        self.add_score(pts);
        self.emit_ard("ard_battle_droid_scene_droid_boom");
        let prefix = if is_laser { "LASER STAP HIT" } else { "STAP DESTROYED" };
        self.set_banner(format!("{prefix}! +{pts}"), 1_800);
    }

    fn complete_battle_droid_scene(&mut self) {
        let perfect_mtt_only = self
            .battle_droid
            .as_ref()
            .is_some_and(|b| b.staps_destroyed == 0);
        self.emit_ard("ard_battle_droid_scene_tank_boom");
        self.complete_active_scene(Swe1Scene::BattleDroid);
        if perfect_mtt_only {
            // Rulebook: 3M when no non-MTT shot (droid/ramp/laser) was hit
            self.add_score(750_000);
        }
        self.battle_droid = None;
        self.set_banner("BATTLE DROID TANK DESTROYED!", 3_000);
    }

    pub(crate) fn render_battle_droid(&mut self, secs: u32) {
        let f60 = ((self.elapsed_ms * 60) / 1000) as usize;
        self.scene_sprite("anim_bdroid_background", 0, 0, 0, 1);
        let bd = self.battle_droid.clone().unwrap_or_default();

        // MTT Frame (`242x135`) + Doors (`138x152`) + Fires (`242x150`)
        self.scene_sprite("anim_mtt_frame", 0, MTT_X, MTT_Y, 2);
        self.scene_sprite(bd.mtt_doors_anim(), f60, MTT_DOORS_X, MTT_DOORS_Y, 4);
        if bd.tank_hits_landed >= 3 {
            self.scene_sprite("anim_mtt_fires", f60, MTT_X, MTT_Y, 3);
        }
        if bd.tank_hit_flash_ms > 0 {
            let fr = ((500 - bd.tank_hit_flash_ms) * 60 / 1000) as usize;
            self.scene_sprite("anim_explosion_2", fr, 210, 180, 7);
        }
        if bd.tank_defeated_ms > 0 {
            let fr = ((500 - bd.tank_defeated_ms) * 60 / 1000) as usize;
            self.scene_sprite(
                "anim_bdroid_tank_explosion",
                fr,
                TANK_EXPLOSION_X,
                TANK_EXPLOSION_Y,
                8,
            );
        }

        // Left & Right Battle Droids (exact ROM coordinates from `bdroid_process`, z=5 above MTT doors)
        for (is_right, side, ms) in [
            (false, bd.left_droid, bd.left_droid_ms),
            (true, bd.right_droid, bd.right_droid_ms),
        ] {
            let (unpack_anim, stand_anim, shoot_anim, ux, sx, sy, ex) = if is_right {
                (
                    "anim_right_bdroid_unpacking",
                    "anim_right_bdroid_standing",
                    "anim_right_bdroid_shooting",
                    RIGHT_DROID_UNPACK_X,
                    RIGHT_DROID_STAND_X,
                    RIGHT_DROID_STAND_Y,
                    RIGHT_DROID_STAND_X - 80,
                )
            } else {
                (
                    "anim_left_bdroid_unpacking",
                    "anim_left_bdroid_standing",
                    "anim_left_bdroid_shooting",
                    LEFT_DROID_UNPACK_X,
                    LEFT_DROID_STAND_X,
                    LEFT_DROID_STAND_Y,
                    LEFT_DROID_STAND_X - 40,
                )
            };
            match side {
                DroidSideState::Unpacking => {
                    let fr = ((ms * 60) / 1000) as usize;
                    self.scene_sprite(unpack_anim, fr.min(14), ux, DROID_UNPACK_Y, 5);
                }
                DroidSideState::Standing => {
                    self.scene_sprite(stand_anim, f60, sx, sy, 5);
                }
                DroidSideState::Shooting => {
                    self.scene_sprite(shoot_anim, f60, sx, sy, 5);
                    let dx = (ms / 4) as i32;
                    let zx = if is_right { sx - 24 - dx } else { sx + 52 + dx };
                    self.scene_sprite("anim_bdroid_zap", f60, zx, 148, 6);
                }
                DroidSideState::Exploding => {
                    let fr = ((ms * 60) / 1000) as usize;
                    self.scene_sprite("anim_bdroid_explosion", fr, ex, 96, 7);
                }
            }
        }

        // Left/Right STAP Patrol + Crosshair + Zap / Explosion (`stap_process` exact Y=84/80)
        if bd.stap_exploding_ms > 0 {
            let fr = ((350 - bd.stap_exploding_ms) * 60 / 1000) as usize;
            let ex = if bd.stap_side_right { 66 } else { 0 };
            self.scene_sprite("anim_bdroid_stap_explosion", fr, ex, 14, 7);
        } else {
            let sway = (((bd.intro_elapsed_ms / 32) % 40) as i32 - 20).abs();
            let (sx, sy) = if (bd.intro_elapsed_ms / 4_000) % 2 == 0 {
                (STAP_LEFT_X + sway, STAP_LEFT_Y)
            } else {
                (STAP_RIGHT_X - sway, STAP_RIGHT_Y)
            };
            self.scene_sprite("anim_bdroid_on_stap", f60, sx, sy, 4);
            self.scene_sprite("anim_stap_xhair", f60, sx + 62, sy + 18, 5);
            if bd.stap_zap_ms < 367 {
                self.scene_sprite("anim_bdroid_stap_zap", f60, sx + 55, sy + 42, 6);
            }
        }

        self.render_scene_footer(bd.tank_hits_remaining, secs, "DROIDS LEFT");
    }
}
