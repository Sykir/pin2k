//! Lifted *Star Wars Episode I* (`#50069`) game state, Jedi progression,
//! Deflector Shield, Lasers, Saucers, Multiball, and all 12 Holoprojector Scenes.

use serde::{Deserialize, Serialize};

use crate::swe1_tables::sound;
use pin2k_hal::SoundId;

/// Jedi rank progression (`JediTable` / `JediMultiball`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum JediRank {
    Padawan,
    Youth,
    Knight,
    Master,
    Spirit,
}

impl JediRank {
    pub const fn label(self) -> &'static str {
        match self {
            JediRank::Padawan => "PADAWAN",
            JediRank::Youth => "JEDI YOUTH",
            JediRank::Knight => "JEDI KNIGHT",
            JediRank::Master => "JEDI MASTER",
            JediRank::Spirit => "JEDI SPIRIT",
        }
    }

    pub const fn advance(self) -> Self {
        match self {
            JediRank::Padawan => JediRank::Youth,
            JediRank::Youth => JediRank::Knight,
            JediRank::Knight => JediRank::Master,
            JediRank::Master | JediRank::Spirit => JediRank::Spirit,
        }
    }
}

/// All 9 official Star Wars character names from `player_name_table_data` (`0x2dc798`).
pub const PLAYER_CHARACTER_NAMES: [&str; 9] = [
    "Qui-Gon",
    "Jar Jar",
    "Anakin",
    "Amidala",
    "Obi-Wan",
    "R2-D2",
    "Yoda",
    "Watto",
    "Darth Maul",
];

/// Formats a score with US thousands commas (`1,234,500`) matching SWE1's `%v` formatter.
pub fn format_score_commas(score: u64) -> String {
    let s = score.to_string();
    let bytes = s.as_bytes();
    let mut out = String::with_capacity(s.len() + s.len() / 3);
    for (idx, &b) in bytes.iter().enumerate() {
        if idx > 0 && (bytes.len() - idx) % 3 == 0 {
            out.push(',');
        }
        out.push(b as char);
    }
    out
}

/// All 14 *Star Wars Episode I* holoprojector scenes (`scene_table_data` @ `0x002dc95c`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Swe1Scene {
    BattleDroid,
    DestroyerDroid,
    FederationFighter,
    GunganBattle,
    HangarEscape,
    HoverTank,
    Podrace,
    ProbeDroid,
    QueenFashion,
    JarJarJuggle,
    WattosChance,
    MosEspaCantina,
    R2D2,
    QuestionMark,
}

impl Swe1Scene {
    pub const ALL: [Swe1Scene; 14] = [
        Swe1Scene::BattleDroid,
        Swe1Scene::DestroyerDroid,
        Swe1Scene::FederationFighter,
        Swe1Scene::GunganBattle,
        Swe1Scene::HangarEscape,
        Swe1Scene::HoverTank,
        Swe1Scene::Podrace,
        Swe1Scene::ProbeDroid,
        Swe1Scene::QueenFashion,
        Swe1Scene::JarJarJuggle,
        Swe1Scene::WattosChance,
        Swe1Scene::MosEspaCantina,
        Swe1Scene::R2D2,
        Swe1Scene::QuestionMark,
    ];

    pub const fn title(self) -> &'static str {
        match self {
            Swe1Scene::BattleDroid => "BATTLE DROID INVASION",
            Swe1Scene::DestroyerDroid => "DESTROYER DROIDS",
            Swe1Scene::FederationFighter => "FEDERATION FIGHTERS",
            Swe1Scene::GunganBattle => "GUNGAN ENERGY BATTLE",
            Swe1Scene::HangarEscape => "THEED HANGAR ESCAPE",
            Swe1Scene::HoverTank => "AAT HOVER TANK",
            Swe1Scene::Podrace => "BOONTA EVE PODRACE",
            Swe1Scene::ProbeDroid => "SITH PROBE DROIDS",
            Swe1Scene::QueenFashion => "QUEEN AMIDALA WARDROBE",
            Swe1Scene::JarJarJuggle => "JAR JAR JUGGLE",
            Swe1Scene::WattosChance => "WATTO'S CHANCE CUBE",
            Swe1Scene::MosEspaCantina => "MUSICAL CHAIRS",
            Swe1Scene::R2D2 => "R2-D2 REPAIR",
            Swe1Scene::QuestionMark => "MYSTERY (?)",
        }
    }

    /// Extracted intro movie/badge animation in `gfx_manifest.json` played BEFORE ball eject.
    pub const fn intro_anim_hint(self) -> &'static str {
        match self {
            Swe1Scene::BattleDroid => "anim_battle_droid_intro_battle",
            Swe1Scene::DestroyerDroid => "anim_destroyer_droid_intro_movie",
            Swe1Scene::FederationFighter => "anim_federation_fighter_intro_battle",
            Swe1Scene::GunganBattle => "anim_sub_escape_intro_movie",
            Swe1Scene::HangarEscape => "anim_hangar_escape_intro_movie",
            Swe1Scene::HoverTank => "anim_hover_tank_intro_movie",
            Swe1Scene::Podrace => "anim_podrace_intro_movie",
            Swe1Scene::ProbeDroid => "anim_probe_droid_intro_movie",
            Swe1Scene::QueenFashion => "anim_queen_ship_intro_movie",
            Swe1Scene::JarJarJuggle => "anim_jarjar_juggle_intro_movie",
            Swe1Scene::WattosChance => "anim_wattos_chance_intro_movie",
            Swe1Scene::MosEspaCantina => "anim_musical_chairs_intro_movie",
            Swe1Scene::R2D2 => "anim_r2d2_intro_movie",
            Swe1Scene::QuestionMark => "anim_question_mark_intro_movie",
        }
    }

    /// Extracted intro badge animation (`Deff*SceneIntro`) paired with the intro movie.
    pub const fn intro_badge_anim(self) -> &'static str {
        match self {
            Swe1Scene::BattleDroid => "anim_battle_droid_intro_badge",
            Swe1Scene::DestroyerDroid => "anim_destroyer_droid_intro_badge",
            Swe1Scene::FederationFighter => "anim_federation_fighter_intro_badge",
            Swe1Scene::GunganBattle => "anim_sub_escape_intro_badge",
            Swe1Scene::HangarEscape => "anim_hangar_escape_intro_badge",
            Swe1Scene::HoverTank => "anim_hover_tank_intro_badge",
            Swe1Scene::Podrace => "anim_podrace_intro_badge",
            Swe1Scene::ProbeDroid => "anim_probe_droid_intro_badge",
            Swe1Scene::QueenFashion => "anim_queen_ship_intro_badge",
            Swe1Scene::JarJarJuggle => "anim_jarjar_juggle_intro_badge",
            Swe1Scene::WattosChance => "anim_wattos_chance_intro_badge",
            Swe1Scene::MosEspaCantina => "anim_musical_chairs_intro_badge",
            Swe1Scene::R2D2 => "anim_r2d2_intro_badge",
            Swe1Scene::QuestionMark => "anim_question_mark_intro_badge",
        }
    }

    /// Native pixel width of `intro_badge_anim()` from `swe1_0200` (`gfx_manifest.json`),
    /// used to center the badge horizontally inside `anim_jedi_scene_intro_frame` (`522x195`).
    pub const fn intro_badge_width(self) -> i32 {
        match self {
            Swe1Scene::BattleDroid => 304,
            Swe1Scene::DestroyerDroid => 426,
            Swe1Scene::FederationFighter => 295,
            Swe1Scene::GunganBattle => 386,
            Swe1Scene::HangarEscape => 299,
            Swe1Scene::HoverTank => 407,
            Swe1Scene::Podrace => 266,
            Swe1Scene::ProbeDroid => 327,
            Swe1Scene::QueenFashion => 304,
            Swe1Scene::JarJarJuggle => 330,
            Swe1Scene::WattosChance => 410,
            Swe1Scene::MosEspaCantina => 394,
            Swe1Scene::R2D2 => 355,
            Swe1Scene::QuestionMark => 446,
        }
    }

    /// Extracted Obi-Wan hologram selection animation (`anim_obiwan_holo_*`) for this scene.
    pub const fn holo_anim_hint(self) -> &'static str {
        match self {
            Swe1Scene::BattleDroid => "anim_obiwan_holo_battle_droid",
            Swe1Scene::DestroyerDroid => "anim_obiwan_holo_destroyer",
            Swe1Scene::FederationFighter => "anim_obiwan_holo_federation_fighter",
            Swe1Scene::GunganBattle => "anim_obiwan_holo_gungan_sub",
            Swe1Scene::HangarEscape => "anim_obiwan_holo_naboo_fighter",
            Swe1Scene::HoverTank => "anim_obiwan_holo_hovertank",
            Swe1Scene::Podrace => "anim_obiwan_holo_anakin_pod",
            Swe1Scene::ProbeDroid => "anim_obiwan_holo_queens_gun",
            Swe1Scene::QueenFashion => "anim_obiwan_holo_queens_logo",
            Swe1Scene::JarJarJuggle => "anim_obiwan_holo_jarjar",
            Swe1Scene::WattosChance => "anim_obiwan_holo_watto",
            Swe1Scene::MosEspaCantina => "anim_obiwan_holo_yoda",
            Swe1Scene::R2D2 => "anim_obiwan_holo_r2d2",
            Swe1Scene::QuestionMark => "anim_obiwan_holo_question",
        }
    }

    /// Extracted gameplay animation played AFTER the intro finishes and ball is ejected.
    pub const fn gameplay_anim_hint(self) -> &'static str {
        match self {
            Swe1Scene::BattleDroid => "anim_bdroid_on_stap",
            Swe1Scene::DestroyerDroid => "anim_destroyer_droid_l_roll",
            Swe1Scene::FederationFighter => "anim_federation_fighter",
            Swe1Scene::GunganBattle => "anim_gungan_sub_chomp",
            Swe1Scene::HangarEscape => "anim_hangar_droid_run",
            Swe1Scene::HoverTank => "anim_hover_tank",
            Swe1Scene::Podrace => "anim_podrace_1",
            Swe1Scene::ProbeDroid => "anim_sith_probe_queen_shoot_1",
            Swe1Scene::QueenFashion => "anim_fashion_queen",
            Swe1Scene::JarJarJuggle => "anim_jarjar_juggle",
            Swe1Scene::WattosChance => "anim_watto_cube",
            Swe1Scene::MosEspaCantina => "anim_chairs",
            Swe1Scene::R2D2 => "anim_r2d2_scene_r2d2_unit",
            Swe1Scene::QuestionMark => "anim_question_mark_green",
        }
    }

    /// Exact intro duration in milliseconds matching the 15 fps ROM movie frame counts.
    pub const fn intro_duration_ms(self) -> u32 {
        match self {
            Swe1Scene::BattleDroid => 3_666,       // 55 frames @ 15fps
            Swe1Scene::DestroyerDroid => 4_466,    // 67 frames @ 15fps
            Swe1Scene::FederationFighter => 3_500,
            Swe1Scene::GunganBattle => 3_666,      // 55 frames @ 15fps
            Swe1Scene::HangarEscape => 4_933,      // 74 frames @ 15fps
            Swe1Scene::HoverTank => 3_500,
            Swe1Scene::Podrace => 3_666,           // 55 frames @ 15fps
            Swe1Scene::ProbeDroid => 5_533,        // 83 frames @ 15fps
            Swe1Scene::QueenFashion => 3_500,
            Swe1Scene::JarJarJuggle => 4_466,      // 67 frames @ 15fps
            Swe1Scene::WattosChance => 2_200,
            Swe1Scene::MosEspaCantina => 3_733,    // 56 frames @ 15fps
            Swe1Scene::R2D2 => 3_533,              // 53 frames @ 15fps
            Swe1Scene::QuestionMark => 5_133,      // 77 frames @ 15fps
        }
    }

    /// Extracted animation prefix in `gfx_manifest.json` matching this SWE1 scene.
    pub const fn anim_hint(self) -> &'static str {
        self.intro_anim_hint()
    }

    pub const fn required_hits(self) -> u8 {
        match self {
            Swe1Scene::WattosChance | Swe1Scene::QueenFashion => 3,
            Swe1Scene::Podrace | Swe1Scene::HangarEscape => 6,
            _ => 5,
        }
    }

    pub const fn duration_ms(self) -> u32 {
        match self {
            Swe1Scene::Podrace => 35_000,
            Swe1Scene::WattosChance => 20_000,
            _ => 30_000,
        }
    }

    pub const fn music_id(self) -> SoundId {
        match self {
            Swe1Scene::Podrace => sound::PODRACE_MUSIC_LOOP,
            Swe1Scene::HangarEscape | Swe1Scene::FederationFighter => sound::HANGAR_MUSIC_LOOP,
            Swe1Scene::MosEspaCantina => sound::CANTINA_MUSIC_LOOP,
            Swe1Scene::JarJarJuggle => sound::JAR_JAR_MUSIC_LOOP,
            Swe1Scene::WattosChance => sound::WATTO_MUSIC_LOOP,
            _ => sound::BATTLE_MUSIC_LOOP,
        }
    }

    /// Minimum scenes started before this scene becomes eligible (`scene_info` table `0x2dc95c`).
    pub const fn min_scenes_started(self) -> u32 {
        match self {
            Swe1Scene::MosEspaCantina
            | Swe1Scene::R2D2
            | Swe1Scene::DestroyerDroid
            | Swe1Scene::HoverTank
            | Swe1Scene::WattosChance
            | Swe1Scene::QuestionMark => 1,
            Swe1Scene::QueenFashion => 2,
            _ => 0,
        }
    }

    /// Shield hits required to open the saucer for the next scene (`Scenes::reset_remaining_hits`):
    /// `1` for the very first scene, `3` for all subsequent scenes.
    pub const fn shield_hits_to_qualify(scenes_started: u32) -> u8 {
        if scenes_started == 0 { 1 } else { 3 }
    }

    /// Authentic `ard_*` pool played when entering this scene.
    pub const fn start_ard(self) -> &'static str {
        match self {
            Swe1Scene::BattleDroid => "ard_battle_droid_scene_start",
            Swe1Scene::DestroyerDroid => "ard_destroyer_droid_scene_intro_queen_speech",
            Swe1Scene::FederationFighter => "ard_fed_fight_scene_ship_swoosh",
            Swe1Scene::GunganBattle => "ard_sub_escape_intro_speech",
            Swe1Scene::HangarEscape => "ard_hangar_escape_scene_initial_boom",
            Swe1Scene::HoverTank => "ard_begin_hover_tank_speech",
            Swe1Scene::Podrace => "ard_pod_race_intro_speech_a1",
            Swe1Scene::ProbeDroid => "ard_probe_droid_intro_scene_speech",
            Swe1Scene::QueenFashion => "ard_queen_ship_scene_start",
            Swe1Scene::JarJarJuggle => "ard_jarjar_juggle_scene_start",
            Swe1Scene::WattosChance => "ard_wattos_chance_scene_intro_watto_speech_1",
            Swe1Scene::MosEspaCantina => "ard_musical_chairs_bounce",
            Swe1Scene::R2D2 => "ard_r2d2_scene_intro_c3p0",
            Swe1Scene::QuestionMark => "ard_question_mark_scene_tune",
        }
    }

    /// Authentic `ard_*` pool played on each qualified shot in this scene.
    pub const fn hit_ard(self) -> &'static str {
        match self {
            Swe1Scene::BattleDroid => "ard_battle_droid_scene_droid_boom",
            Swe1Scene::DestroyerDroid => "ard_destroyer_droid_scene_destroyer_boom",
            Swe1Scene::FederationFighter => "ard_fed_fight_scene_fighter_boom",
            Swe1Scene::GunganBattle => "ard_hit_gungan_sub_speech",
            Swe1Scene::HangarEscape => "ard_hangar_escape_scene_droid_boom",
            Swe1Scene::HoverTank => "ard_hover_tank_scene_tank_boom",
            Swe1Scene::Podrace => "ard_pod_race_random_speech",
            Swe1Scene::ProbeDroid => "ard_probe_droid_scene_probe_boom",
            Swe1Scene::QueenFashion => "ard_fashion_expose",
            Swe1Scene::JarJarJuggle => "ard_jarjar_juggle_scene_jarjar_hit_speech",
            Swe1Scene::WattosChance => "ard_wattos_chance_scene_cube_hit_c",
            Swe1Scene::MosEspaCantina => "ard_musical_chairs_land",
            Swe1Scene::R2D2 => "ard_r2d2_scene_hit_1",
            Swe1Scene::QuestionMark => "ard_question_mark_scene_tune",
        }
    }

    /// Authentic `ard_*` pool played upon completing this scene.
    pub const fn complete_ard(self) -> &'static str {
        match self {
            Swe1Scene::BattleDroid => "ard_battle_droid_scene_complete",
            Swe1Scene::DestroyerDroid => "ard_destroyer_droid_scene_complete",
            Swe1Scene::FederationFighter => "ard_fed_fight_scene_complete",
            Swe1Scene::GunganBattle => "ard_sub_escape_opee_win",
            Swe1Scene::HangarEscape => "ard_hangar_escape_scene_complete",
            Swe1Scene::HoverTank => "ard_hover_tank_scene_complete",
            Swe1Scene::Podrace => "ard_pod_race_exit_speech",
            Swe1Scene::ProbeDroid => "ard_probe_droid_scene_complete",
            Swe1Scene::QueenFashion => "ard_queen_ship_scene_start_followup",
            Swe1Scene::JarJarJuggle => "ard_jarjar_spinner_complete_speech",
            Swe1Scene::WattosChance => "ard_wattos_chance_speech_haggle_4A",
            Swe1Scene::MosEspaCantina => "ard_musical_chairs_award",
            Swe1Scene::R2D2 => "ard_hit_r2d2_speech",
            Swe1Scene::QuestionMark => "ard_question_mark_scene_tune",
        }
    }

    /// Authentic shot award from Ghidra decompilation (`BattleDroidScene`, `PodraceScene`, etc.).
    pub const fn hit_score(self) -> u64 {
        match self {
            Swe1Scene::BattleDroid => scoring::BATTLE_DROID_SHIELD_HIT,
            Swe1Scene::DestroyerDroid => scoring::DESTROYER_DROID_DESTROY,
            Swe1Scene::FederationFighter => 250_000,
            Swe1Scene::GunganBattle => scoring::GUNGAN_SHOT,
            Swe1Scene::HangarEscape => scoring::HANGAR_ESCAPE_HIT * 5,
            Swe1Scene::HoverTank => scoring::HOVER_TANK_DESTROY,
            Swe1Scene::Podrace => 75_000,
            Swe1Scene::ProbeDroid => scoring::PROBE_DROID_DESTROY,
            Swe1Scene::QueenFashion => scoring::QUEEN_FASHION_MATCH,
            Swe1Scene::JarJarJuggle => 150_000,
            Swe1Scene::WattosChance => scoring::WATTOS_CHANCE_WIN,
            Swe1Scene::MosEspaCantina => scoring::MUSICAL_CHAIRS_CENTER,
            Swe1Scene::R2D2 => 250_000,
            Swe1Scene::QuestionMark => 500_000,
        }
    }

    /// Authentic completion award from Ghidra decompilation (`BattleDroidScene` = `2,250,000`, etc.).
    pub const fn completion_score(self) -> u64 {
        match self {
            Swe1Scene::BattleDroid => scoring::BATTLE_DROID_COMPLETE,
            Swe1Scene::Podrace
            | Swe1Scene::HangarEscape
            | Swe1Scene::DestroyerDroid
            | Swe1Scene::HoverTank
            | Swe1Scene::ProbeDroid => 1_750_000,
            _ => 1_500_000,
        }
    }
}

/// Exact scoring and rule constants extracted from `swe1_0166.elf` & SWE1 v1.4 rulebook.
pub mod scoring {
    pub const SWITCH_BASE: u64 = 2_310;
    pub const SAUCER_ARRIVAL: u64 = 2_330;
    pub const SKILL_SHOT_FIRST: u64 = 1_000_000;
    pub const SKILL_SHOT_SECOND: u64 = 2_000_000;
    pub const SKILL_SHOT_CAP: u64 = 3_000_000;
    pub const BONUS_PER_SHIP: u64 = 100_000;
    pub const BONUS_MAX_MULTIPLIER: u8 = 5;
    pub const JET_NORMAL: u64 = 10_310;
    pub const JET_SUPER: u64 = 50_000;
    pub const DROP_TARGET: u64 = 2_310;
    pub const STANDUP_NEW: u64 = 10_000;
    pub const STANDUP_LIT: u64 = 5_000;
    pub const STANDUP_BANK_COMPLETE: u64 = 500_000;
    pub const STANDUP_LASER_AWARD: u64 = 50_000;
    pub const LASER_FIRE: u64 = 1_000;
    pub const MAX_LASER_SHOTS: u8 = 20;
    pub const RAMP_COMBO_STEP: u64 = 100_000;
    pub const RAMP_COMBO_WINDOW_MS: u32 = 6_720;
    pub const C3P0_FALLBACK_AWARD: u64 = 1_000_000;
    pub const JEDI_LETTER_BASE: u64 = 550_000;
    pub const JEDI_LETTER_STEP: u64 = 50_000;
    pub const JEDI_LETTERS_COMPLETE: u64 = 2_000_000;
    pub const MULTIBALL_START: u64 = 500_000;
    pub const MULTIBALL_JACKPOT: u64 = 1_000_000;
    pub const MULTIBALL_SUPER_JACKPOT: u64 = 5_000_000;
    pub const JEDI_MB_START: u64 = 500_000;
    pub const JEDI_MB_ATTACK_MAUL: u64 = 500_000;
    pub const JEDI_MB_ATTACK_MAUL_COMBO: u64 = 700_000;
    pub const JEDI_MB_ATTACK_QUIGON: u64 = 1_000_000;
    pub const BATTLE_DROID_SHIELD_HIT: u64 = 400_000;
    pub const BATTLE_DROID_TARGET: u64 = 200_000;
    pub const BATTLE_DROID_STAP_DESTROY: u64 = 250_000;
    pub const BATTLE_DROID_STAP_BONUS: u64 = 500_000;
    pub const BATTLE_DROID_COMPLETE: u64 = 2_250_000;
    pub const GUNGAN_SHOT: u64 = 50_000;
    pub const PODRACE_SHOT: u64 = 50_000;
    pub const QUEEN_FASHION_SPIN: u64 = 20_000;
    pub const QUEEN_FASHION_MATCH: u64 = 500_000;
    pub const MUSICAL_CHAIRS_JUMP: u64 = 70_000;
    pub const MUSICAL_CHAIRS_CENTER: u64 = 200_000;
    pub const HOVER_TANK_HIT: u64 = 50_000;
    pub const HOVER_TANK_DESTROY: u64 = 250_000;
    pub const DESTROYER_DROID_HIT: u64 = 50_000;
    pub const DESTROYER_DROID_DESTROY: u64 = 250_000;
    pub const DESTROYER_DROID_SHIP: u64 = 450_000;
    pub const PROBE_DROID_HIT: u64 = 70_000;
    pub const PROBE_DROID_DESTROY: u64 = 250_000;
    pub const WATTOS_CHANCE_SPIN: u64 = 20_000;
    pub const WATTOS_CHANCE_WIN: u64 = 250_000;
    pub const HANGAR_ESCAPE_HIT: u64 = 20_000;

    /// Captive ball award for 0-based hit index `hit_idx` (no 2,310 base added).
    /// Cycles `[50..10]`, `[500..100]`, `[5000..1000]`, then `[50000..10000]` forever.
    pub const fn captive_ball_score(hit_idx: u16) -> u64 {
        let tier = hit_idx / 5;
        let step = (hit_idx % 5) as u64;
        let mult = match tier {
            0 => 10,
            1 => 100,
            2 => 1_000,
            _ => 10_000,
        };
        (5 - step) * mult
    }
}

/// Active running holoprojector scene state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveSceneState {
    pub scene: Swe1Scene,
    pub hits_remaining: u8,
    pub timer_ms: u32,
    #[serde(default)]
    pub intro_remaining_ms: u32,
    /// Milliseconds elapsed in the active (post-intro) gameplay phase.
    #[serde(default)]
    pub active_ms: u32,
}

/// Active multiball mode (`MainMultiball` or `JediMultiball` Darth Maul Saber Duel).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActiveMultiball {
    None,
    HangarMultiball {
        jackpots_lit: bool,
        jackpots_collected: u8,
    },
    DarthMaulDuel {
        maul_health: u8,
        quigon_health: u8,
        combo_window_ms: u32,
        maul_attack_ms: u32,
    },
}

/// Per-player state across multi-ball / multi-player games (`1..=4` players).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    pub score: u64,
    #[serde(default)]
    pub character_idx: usize,
    pub jedi_letters: u8, // bitmask 0..4 for J, E, D, I
    pub jedi_rank: JediRank,
    pub scenes_completed: u16, // 14-bit mask of completed `Swe1Scene`
    pub next_scene_idx: usize,
    pub shield_health: u8, // 0..=12; 0 opens Shield Popper to start scene!
    #[serde(default)]
    pub scene_select_ready: bool,
    pub laser_shots: u8,
    pub bonus_multiplier: u8, // 1..=5
    pub bonus_points: u64,
    pub saucer_locks: u8, // 0..=2 -> 3rd lock starts Hangar Multiball
    pub extra_balls: u8,
    pub jet_hits: u16,
    #[serde(default)]
    pub skill_shots_made: u8,
    #[serde(default)]
    pub gungan_letters: u8,
    #[serde(default)]
    pub super_jets_ms: u32,
    #[serde(default)]
    pub ramp_combo_count: u8,
    #[serde(default)]
    pub ramp_combo_window_ms: u32,
    #[serde(default)]
    pub ramp_combo_best: u8,
    #[serde(default)]
    pub c3p0_parts: u8,
    #[serde(default)]
    pub c3p0_reward_idx: u8,
    #[serde(default)]
    pub left_bank_mask: u8,
    #[serde(default)]
    pub right_bank_mask: u8,
    #[serde(default)]
    pub left_hotdog_lit: bool,
    #[serde(default)]
    pub right_hotdog_lit: bool,
    #[serde(default)]
    pub naboo_ship_lamps: u8,
    #[serde(default)]
    pub hold_bonus: bool,
    #[serde(default)]
    pub captive_ball_hits: u16,
    #[serde(default)]
    pub skill_shot_jedi_awarded: bool,
    #[serde(default)]
    pub jedi_letters_total: u16,
    #[serde(default)]
    pub scenes_started: u32,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            score: 0,
            character_idx: 0,
            jedi_letters: 0,
            jedi_rank: JediRank::Padawan,
            scenes_completed: 0,
            next_scene_idx: 0,
            shield_health: 6,
            scene_select_ready: false,
            laser_shots: 3,
            bonus_multiplier: 1,
            bonus_points: 0,
            saucer_locks: 0,
            extra_balls: 0,
            jet_hits: 0,
            skill_shots_made: 0,
            gungan_letters: 0,
            super_jets_ms: 0,
            ramp_combo_count: 0,
            ramp_combo_window_ms: 0,
            ramp_combo_best: 0,
            c3p0_parts: 4, // Factory "C-3P0 Game Start spot" = 4
            c3p0_reward_idx: 0,
            left_bank_mask: 0,
            right_bank_mask: 0,
            left_hotdog_lit: false,
            right_hotdog_lit: false,
            naboo_ship_lamps: 0,
            hold_bonus: false,
            captive_ball_hits: 0,
            skill_shot_jedi_awarded: false,
            jedi_letters_total: 0,
            scenes_started: 0,
        }
    }
}

impl PlayerState {
    /// Returns this player's assigned Star Wars character name from `player_name_table_data`.
    pub fn character_name(&self) -> &'static str {
        PLAYER_CHARACTER_NAMES[self.character_idx % PLAYER_CHARACTER_NAMES.len()]
    }

    /// Returns this player's `88×76` animated portrait (`anim_bonus_character_*`) for Bonus.
    pub fn bonus_character_anim(&self) -> &'static str {
        const BONUS_CHARS: [&str; 9] = [
            "anim_bonus_character_quigon",
            "anim_bonus_character_jarjar",
            "anim_bonus_character_anakin",
            "anim_bonus_character_queen1",
            "anim_bonus_character_obiwan",
            "anim_bonus_character_r2d2",
            "anim_bonus_character_yoda",
            "anim_bonus_character_watto",
            "anim_bonus_character_maul1",
        ];
        BONUS_CHARS[self.character_idx % BONUS_CHARS.len()]
    }

    /// Chooses a unique character index (`DeffScoreDisplay::is_name_unique`) not yet taken in `used`.
    pub fn pick_unique_character(seed: usize, used: &[usize]) -> usize {
        for off in 0..PLAYER_CHARACTER_NAMES.len() {
            let cand = (seed + off) % PLAYER_CHARACTER_NAMES.len();
            if !used.contains(&cand) {
                return cand;
            }
        }
        seed % PLAYER_CHARACTER_NAMES.len()
    }

    pub fn award_jedi_letter(&mut self) -> bool {
        for bit in 0..4u8 {
            if (self.jedi_letters & (1 << bit)) == 0 {
                self.jedi_letters |= 1 << bit;
                return self.jedi_letters == 0x0F;
            }
        }
        true
    }

    pub fn rotate_jedi_letters(&mut self, right: bool) {
        let l = self.jedi_letters & 0x0F;
        self.jedi_letters = if right {
            ((l << 1) | (l >> 3)) & 0x0F
        } else {
            ((l >> 1) | (l << 3)) & 0x0F
        };
    }

    pub fn choose_next_scene(&mut self) -> Swe1Scene {
        let n = Swe1Scene::ALL.len();
        for offset in 0..n {
            let idx = (self.next_scene_idx + offset) % n;
            if (self.scenes_completed & (1 << idx)) == 0 {
                self.next_scene_idx = (idx + 1) % n;
                return Swe1Scene::ALL[idx];
            }
        }
        // All 14 completed -> reset cycle!
        self.scenes_completed = 0;
        self.next_scene_idx = 1;
        Swe1Scene::ALL[0]
    }
}

/// High score / Jedi Champion entry (`HSTDtable` / `JediTable`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighScoreEntry {
    pub initials: [char; 3],
    pub score: u64,
    pub rank: JediRank,
}

pub fn default_high_scores() -> Vec<HighScoreEntry> {
    vec![
        HighScoreEntry { initials: ['Q', 'G', 'J'], score: 85_000_000, rank: JediRank::Spirit },
        HighScoreEntry { initials: ['O', 'B', 'I'], score: 60_000_000, rank: JediRank::Master },
        HighScoreEntry { initials: ['A', 'N', 'I'], score: 45_000_000, rank: JediRank::Knight },
        HighScoreEntry { initials: ['P', 'A', 'D'], score: 30_000_000, rank: JediRank::Youth },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swe1_extracted_rules_and_scoring() {
        assert_eq!(Swe1Scene::shield_hits_to_qualify(0), 1);
        assert_eq!(Swe1Scene::shield_hits_to_qualify(1), 3);
        assert_eq!(Swe1Scene::BattleDroid.min_scenes_started(), 0);
        assert_eq!(Swe1Scene::MosEspaCantina.min_scenes_started(), 1);
        assert_eq!(Swe1Scene::QueenFashion.min_scenes_started(), 2);

        assert_eq!(scoring::BONUS_PER_SHIP * 10 * (scoring::BONUS_MAX_MULTIPLIER as u64), 5_000_000);
        assert_eq!(scoring::JET_NORMAL, 10_310);
        assert_eq!(scoring::JET_SUPER, 50_000);
        assert_eq!(scoring::MULTIBALL_SUPER_JACKPOT, 5_000_000);
        assert_eq!(scoring::captive_ball_score(0), 50);
        assert_eq!(scoring::captive_ball_score(4), 10);
        assert_eq!(scoring::captive_ball_score(5), 500);
        assert_eq!(scoring::captive_ball_score(10), 5_000);
        assert_eq!(scoring::captive_ball_score(15), 50_000);
        assert_eq!(scoring::captive_ball_score(20), 50_000);
    }
}
