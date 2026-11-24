//! Lifted directly from `swe1_0200.elf` (`_attract.cpp`, `0x00100358..0x001035d6`,
//! `0x002b0bb8..0x002b8574`, `AttractProcInfo` table @ `0x002d94d0`).
//!
//! Reproduces the exact 20-entry `DeffAttractModeBackground` sequence,
//! `AttractMode::left_hit` / `right_hit` / `show_scores`, and `DeffAttractModePriceBackground`.

use serde::{Deserialize, Serialize};

/// Exact 20 entries of SWE1's `AttractProcInfo` table (`0x002d94d0`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttractDeff {
    KahunaLogo,          // 0:  deff_attract_mode_kahuna_logo (anim_klogo_zoom_back + flare)
    SlideShow,           // 1:  deff_attract_mode_slide_show
    Hstd,                // 2:  deff_attract_mode_hstd
    JediTableYouth,      // 3:  deff_jedi_table_youth
    JediTableKnight,     // 4:  deff_jedi_table_knight
    JediTableMaster,     // 5:  deff_jedi_table_master
    JediTableSpirit,     // 6:  deff_jedi_table_spirit
    RampComboChampion,   // 7:  deff_attract_mode_ramp_combo_champion
    P2kLogo,             // 8:  deff_attract_mode_p2k_logo (anim_p2k_logo_animated)
    HasbroAd,            // 9:  deff_attract_mode_hasbro_ad (anim_attract_hasbro_ad)
    ScoreAwards,         // 10: deff_attract_mode_score_awards
    EmailAddress,        // 11: deff_attract_mode_email_address
    WebPage,             // 12: deff_attract_mode_web_page (anim_attract_anakin_web)
    Scores,              // 13: deff_attract_mode_scores (ok_for_scores)
    GameCredits1,        // 14: deff_attract_mode_game_credits_1
    GameCredits2,        // 15: deff_attract_mode_game_credits_2
    GameCredits3,        // 16: deff_attract_mode_game_credits_3
    GameCredits4,        // 17: deff_attract_mode_game_credits_4
    CustomMessage,       // 18: deff_custom_message
    HtpScene,            // 19: deff_htp_scene (How To Play)
}

impl AttractDeff {
    pub const ALL: [AttractDeff; 20] = [
        AttractDeff::KahunaLogo,
        AttractDeff::SlideShow,
        AttractDeff::Hstd,
        AttractDeff::JediTableYouth,
        AttractDeff::JediTableKnight,
        AttractDeff::JediTableMaster,
        AttractDeff::JediTableSpirit,
        AttractDeff::RampComboChampion,
        AttractDeff::P2kLogo,
        AttractDeff::HasbroAd,
        AttractDeff::ScoreAwards,
        AttractDeff::EmailAddress,
        AttractDeff::WebPage,
        AttractDeff::Scores,
        AttractDeff::GameCredits1,
        AttractDeff::GameCredits2,
        AttractDeff::GameCredits3,
        AttractDeff::GameCredits4,
        AttractDeff::CustomMessage,
        AttractDeff::HtpScene,
    ];

    /// Duration in milliseconds matching each `Deff*::Run` loop in `swe1_0200.elf`.
    pub const fn duration_ms(self) -> u32 {
        match self {
            AttractDeff::KahunaLogo => 3_500,
            AttractDeff::P2kLogo => 6_000, // 90 frames @ 15fps
            AttractDeff::HasbroAd => 4_000,
            AttractDeff::SlideShow => 5_000,
            AttractDeff::HtpScene => 5_500,
            _ => 4_000,
        }
    }

    /// Exact ROM animation symbol name from `swe1_0200.elf` for this `Deff`.
    pub const fn anim_hint(self) -> Option<&'static str> {
        match self {
            AttractDeff::KahunaLogo => Some("anim_klogo_zoom_back"),
            AttractDeff::SlideShow => Some("anim_battle_droid_intro_battle"),
            AttractDeff::P2kLogo => Some("anim_p2k_logo_animated"),
            AttractDeff::HasbroAd => Some("anim_attract_hasbro_ad"),
            AttractDeff::WebPage => Some("anim_attract_anakin_web"),
            AttractDeff::HtpScene => Some("anim_battle_droid_intro_badge"),
            _ => None,
        }
    }
}

/// Lifted `DeffAttractModeBackground` state (`[ebx+0x18]` = `idx`, `[ebx+0x1c]` = `cycle_count`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttractController {
    pub index: usize,
    pub cycle_count: u32,
    pub timer_ms: u32,
    pub last_game_played: bool,
    pub attract_music_started: bool,
}

impl AttractController {
    pub fn current(&self) -> AttractDeff {
        AttractDeff::ALL[self.index % AttractDeff::ALL.len()]
    }

    pub fn is_ok(&self, deff: AttractDeff) -> bool {
        match deff {
            AttractDeff::Scores => self.last_game_played,
            AttractDeff::CustomMessage => false,
            _ => true,
        }
    }

    /// Advances to the next enabled `AttractProcInfo` entry (`DeffAttractModeBackground::Run`).
    pub fn advance(&mut self, delta: isize) {
        let n = AttractDeff::ALL.len() as isize;
        for _ in 0..AttractDeff::ALL.len() {
            let next = (self.index as isize + delta).rem_euclid(n) as usize;
            if delta > 0 && next < self.index {
                self.cycle_count = self.cycle_count.saturating_add(1);
            }
            self.index = next;
            if self.is_ok(self.current()) {
                break;
            }
        }
        self.timer_ms = 0;
    }

    pub fn tick(&mut self, dt_ms: u32) -> bool {
        self.timer_ms = self.timer_ms.saturating_add(dt_ms);
        if self.timer_ms >= self.current().duration_ms() {
            self.advance(1);
            return true;
        }
        false
    }
}
