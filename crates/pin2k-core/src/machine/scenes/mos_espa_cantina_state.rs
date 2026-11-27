//! State & ROM constants for `MusicalChairsScene` (`MosEspaCantina`, `0x00158b58..0x0015bd00`).

use serde::{Deserialize, Serialize};

pub const COUNCIL_ANIMS: [&str; 6] = [
    "anim_jedi_council_mace",
    "anim_jedi_council_neck",
    "anim_jedi_council_yoda",
    "anim_jedi_council_member_1",
    "anim_jedi_council_member_2",
    "anim_jedi_council_chic",
];
pub const COUNCIL_FRAME_COUNTS: [usize; 6] = [1, 12, 6, 6, 5, 1];

pub const BG_X: i32 = 6;
pub const BG_Y: i32 = 3;
/// Exact ROM chair positions (`Member::jump_to_slot(..., 0xaa)` -> bottom Y=170, sprite h=58 -> Y=112).
pub const CHAIR_X: [i32; 5] = [82, 166, 256, 348, 438];
pub const CHAIR_Y: i32 = 112;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CouncilAward {
    Points(u64),
    AddJediLetter,
    AdvanceBonusX,
}

impl CouncilAward {
    pub fn label(self) -> String {
        match self {
            CouncilAward::Points(p) => format!("{p}"),
            CouncilAward::AddJediLetter => "ADD JEDI LETTER".to_string(),
            CouncilAward::AdvanceBonusX => "ADVANCE BONUS X".to_string(),
        }
    }

    /// Two-line ROM display (`Member::plot_award_text` from `member_award_table_*`).
    pub fn lines(self) -> (&'static str, &'static str) {
        match self {
            CouncilAward::AddJediLetter => ("ADD JEDI", "LETTER"),
            CouncilAward::AdvanceBonusX => ("ADVANCE", "BONUS X"),
            CouncilAward::Points(1_000) => ("1,000", "POINTS"),
            CouncilAward::Points(5_000) => ("5,000", "POINTS"),
            CouncilAward::Points(10_000) => ("10,000", "POINTS"),
            CouncilAward::Points(15_000) => ("15,000", "POINTS"),
            CouncilAward::Points(20_000) => ("20,000", "POINTS"),
            CouncilAward::Points(50_000) => ("50,000", "POINTS"),
            CouncilAward::Points(100_000) => ("100,000", "POINTS"),
            CouncilAward::Points(500_000) => ("500,000", "POINTS"),
            CouncilAward::Points(1_000_000) => ("1,000,000", "POINTS"),
            CouncilAward::Points(2_000_000) => ("2,000,000", "POINTS"),
            CouncilAward::Points(_) => ("200,000", "POINTS"),
        }
    }
}

pub const RARE_CHAIR_AWARDS: [CouncilAward; 2] = [
    CouncilAward::AddJediLetter,
    CouncilAward::AdvanceBonusX,
];

pub const DEFAULT_CHAIR_AWARDS: [CouncilAward; 10] = [
    CouncilAward::Points(1_000),
    CouncilAward::Points(5_000),
    CouncilAward::Points(10_000),
    CouncilAward::Points(15_000),
    CouncilAward::Points(20_000),
    CouncilAward::Points(50_000),
    CouncilAward::Points(100_000),
    CouncilAward::Points(500_000),
    CouncilAward::Points(1_000_000),
    CouncilAward::Points(2_000_000),
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MosEspaCantinaState {
    /// Number of departures remaining to win the scene (3 in authentic ROM & rulebook).
    pub seats_remaining: u8,
    pub music_stopped: bool,
    pub phase_ms: u32,
    pub music_duration_ms: u32,
    /// Permutation of member indices across the 5 chairs (`order[2]` is center seat).
    pub order: [usize; 5],
    /// Bitmask of departed seat slots (`0..5`).
    pub departed_mask: u8,
    pub awards: [CouncilAward; 5],
    pub jump_ms: u32,
    /// Per-member idle countdown (ms) before next animation burst (`1,000..6,000 ms`, `DeffMusicalChairsBackground`).
    pub member_idle_ms: [u32; 6],
    /// Per-member remaining burst duration (ms) while playing (`0` = idle at frame `0`).
    pub member_burst_ms: [u32; 6],
    /// Per-member total burst duration (ms) for ping-pong / loop frame calculation.
    pub member_burst_total_ms: [u32; 6],
    pub rng_state: u32,
}

impl MosEspaCantinaState {
    pub fn new() -> Self {
        Self {
            seats_remaining: 3,
            music_stopped: false,
            phase_ms: 0,
            music_duration_ms: 5_000,
            order: [0, 1, 2, 3, 5],
            departed_mask: 0,
            awards: [
                DEFAULT_CHAIR_AWARDS[6],
                DEFAULT_CHAIR_AWARDS[7],
                RARE_CHAIR_AWARDS[0],
                RARE_CHAIR_AWARDS[1],
                DEFAULT_CHAIR_AWARDS[8],
            ],
            jump_ms: 0,
            member_idle_ms: [1_200, 2_100, 3_000, 3_900, 4_800, 5_700],
            member_burst_ms: [0; 6],
            member_burst_total_ms: [1_120; 6],
            rng_state: 0x5EED_1999,
        }
    }

    /// Authentic `MusicalChairsScene::select_awards()`: up to 2 rare awards at 40% each,
    /// then unique random picks from `member_award_table_default`.
    pub fn select_awards(&mut self, seed: u32, allow_jedi_letter: bool, allow_bonus_x: bool) {
        self.rng_state = seed ^ 0x5EED_1999;
        let mut chosen: [Option<CouncilAward>; 5] = [None; 5];
        for _ in 0..RARE_CHAIR_AWARDS.len() {
            if (self.next_rand() % 100) < 40 {
                let start = (self.next_rand() as usize) % RARE_CHAIR_AWARDS.len();
                for off in 0..RARE_CHAIR_AWARDS.len() {
                    let cand = RARE_CHAIR_AWARDS[(start + off) % RARE_CHAIR_AWARDS.len()];
                    let valid = match cand {
                        CouncilAward::AddJediLetter => allow_jedi_letter,
                        CouncilAward::AdvanceBonusX => allow_bonus_x,
                        _ => true,
                    };
                    if valid && !chosen.contains(&Some(cand)) {
                        if let Some(slot) = chosen.iter_mut().find(|s| s.is_none()) {
                            *slot = Some(cand);
                        }
                        break;
                    }
                }
            }
        }
        for idx in 0..chosen.len() {
            if chosen[idx].is_none() {
                let start = (self.next_rand() as usize) % DEFAULT_CHAIR_AWARDS.len();
                for off in 0..DEFAULT_CHAIR_AWARDS.len() {
                    let cand = DEFAULT_CHAIR_AWARDS[(start + off) % DEFAULT_CHAIR_AWARDS.len()];
                    if !chosen.contains(&Some(cand)) {
                        chosen[idx] = Some(cand);
                        break;
                    }
                }
            }
        }
        for (i, slot) in chosen.into_iter().enumerate() {
            self.awards[i] = slot.unwrap_or(DEFAULT_CHAIR_AWARDS[i % DEFAULT_CHAIR_AWARDS.len()]);
        }
    }

    fn next_rand(&mut self) -> u32 {
        self.rng_state = self.rng_state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        self.rng_state
    }

    /// Advances authentic `member_control_process_*` idle/burst timers (`sleep100(rand()%500+100)`, step `70ms`).
    pub fn tick_member_anims(&mut self, dt_ms: u32) {
        for idx in 0..6 {
            if self.member_burst_ms[idx] > 0 {
                self.member_burst_ms[idx] = self.member_burst_ms[idx].saturating_sub(dt_ms);
                if self.member_burst_ms[idx] == 0 {
                    let r = self.next_rand();
                    self.member_idle_ms[idx] = 1_000 + (r % 5_000);
                }
            } else {
                self.member_idle_ms[idx] = self.member_idle_ms[idx].saturating_sub(dt_ms);
                if self.member_idle_ms[idx] == 0 {
                    let r = self.next_rand();
                    let dur = if idx == 1 || idx == 2 {
                        70 * 8 * (1 + (r % 3))
                    } else {
                        70 * 14
                    };
                    self.member_burst_total_ms[idx] = dur;
                    self.member_burst_ms[idx] = dur;
                }
            }
        }
    }

    /// Current frame index for member `idx`: `0` while idle, `70ms`/step ping-pong or neck loop during burst.
    pub fn member_frame(&self, idx: usize) -> usize {
        let len = COUNCIL_FRAME_COUNTS.get(idx).copied().unwrap_or(1);
        if len <= 1 {
            return 0;
        }
        let rem = self.member_burst_ms.get(idx).copied().unwrap_or(0);
        if rem == 0 {
            return 0;
        }
        let total = self.member_burst_total_ms[idx].max(70);
        let elapsed = total.saturating_sub(rem);
        let step = (elapsed / 70) as usize;
        if idx == 1 {
            // `member_control_process_neck`: plays forward loop 1..4 times
            step % len
        } else {
            // `member_control_process_ping_pong` / `yoda`: forward then reverse back to frame 0
            let max_f = len - 1;
            let cycle = max_f * 2;
            let pos = step % cycle;
            if pos <= max_f {
                pos
            } else {
                cycle - pos
            }
        }
    }

    pub fn shuffle(&mut self) {
        let active: Vec<usize> = (0..5).filter(|&i| !self.is_seat_departed(i)).collect();
        if active.len() <= 1 {
            return;
        }
        let first_ord = self.order[active[0]];
        let first_aw = self.awards[active[0]];
        for w in 0..active.len() - 1 {
            self.order[active[w]] = self.order[active[w + 1]];
            self.awards[active[w]] = self.awards[active[w + 1]];
        }
        let last = active[active.len() - 1];
        self.order[last] = first_ord;
        self.awards[last] = first_aw;
    }

    pub fn is_seat_departed(&self, idx: usize) -> bool {
        (self.departed_mask & (1 << idx)) != 0
    }

    /// Center member (`slot 2`) departs: center (`slot 2`) always stays occupied,
    /// and outer chairs are removed in order: far-right (`slot 4`) first, then far-left (`slot 0`).
    pub fn mark_center_departed(&mut self) {
        match self.seats_remaining {
            3 => {
                self.order[2] = self.order[4];
                self.awards[2] = self.awards[4];
                self.departed_mask |= 1 << 4;
            }
            2 => {
                self.order[2] = self.order[0];
                self.awards[2] = self.awards[0];
                self.departed_mask |= 1 << 0;
            }
            _ => {
                self.departed_mask |= 1 << 2;
            }
        }
        self.seats_remaining = self.seats_remaining.saturating_sub(1);
    }
}

impl Default for MosEspaCantinaState {
    fn default() -> Self {
        Self::new()
    }
}
