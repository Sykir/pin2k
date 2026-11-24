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

/// All 12 *Star Wars Episode I* holoprojector scenes (`Scenes` class in `swe1_0200`).
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
}

impl Swe1Scene {
    pub const ALL: [Swe1Scene; 12] = [
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
            Swe1Scene::MosEspaCantina => "MOS ESPA CANTINA",
        }
    }

    /// Extracted animation prefix in `gfx_manifest.json` matching this SWE1 scene.
    pub const fn anim_hint(self) -> &'static str {
        match self {
            Swe1Scene::BattleDroid => "anim_battle_droid_intro_battle",
            Swe1Scene::DestroyerDroid => "anim_ddroid",
            Swe1Scene::FederationFighter => "anim_federation_fighter",
            Swe1Scene::GunganBattle => "anim_gungan_sub_chomp",
            Swe1Scene::HangarEscape => "anim_hangar_droid_explosion",
            Swe1Scene::HoverTank => "anim_hover_tank_intro_movie",
            Swe1Scene::Podrace => "anim_podrace_intro_movie",
            Swe1Scene::ProbeDroid => "anim_sith_probe_queen_shoot_1",
            Swe1Scene::QueenFashion => "anim_fashion_queen",
            Swe1Scene::JarJarJuggle => "anim_jarjar_juggle",
            Swe1Scene::WattosChance => "anim_watto_cube",
            Swe1Scene::MosEspaCantina => "anim_chairs",
        }
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
            Swe1Scene::MosEspaCantina | Swe1Scene::JarJarJuggle => sound::CANTINA_MUSIC_LOOP,
            _ => sound::BATTLE_MUSIC_LOOP,
        }
    }
}

/// Active running holoprojector scene state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveSceneState {
    pub scene: Swe1Scene,
    pub hits_remaining: u8,
    pub timer_ms: u32,
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
        combo_window_ms: u32,
    },
}

/// Per-player state across multi-ball / multi-player games (`1..=4` players).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    pub score: u64,
    pub jedi_letters: u8, // bitmask 0..4 for J, E, D, I
    pub jedi_rank: JediRank,
    pub scenes_completed: u16, // 12-bit mask of completed `Swe1Scene`
    pub next_scene_idx: usize,
    pub shield_health: u8, // 0..=12; 0 opens Shield Popper to start scene!
    pub laser_shots: u8,
    pub bonus_multiplier: u8, // 1..=5
    pub bonus_points: u64,
    pub saucer_locks: u8, // 0..=2 -> 3rd lock starts Hangar Multiball
    pub extra_balls: u8,
    pub jet_hits: u16,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            score: 0,
            jedi_letters: 0,
            jedi_rank: JediRank::Padawan,
            scenes_completed: 0,
            next_scene_idx: 0,
            shield_health: 6,
            laser_shots: 3,
            bonus_multiplier: 1,
            bonus_points: 10_000,
            saucer_locks: 0,
            extra_balls: 0,
            jet_hits: 0,
        }
    }
}

impl PlayerState {
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
        for offset in 0..12 {
            let idx = (self.next_scene_idx + offset) % 12;
            if (self.scenes_completed & (1 << idx)) == 0 {
                self.next_scene_idx = (idx + 1) % 12;
                return Swe1Scene::ALL[idx];
            }
        }
        // All 12 completed -> reset cycle!
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
