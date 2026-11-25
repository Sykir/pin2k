use pin2k_core::rfm_tables::rfm_sw;
use pin2k_core::swe1_tables::sw;

pub(crate) fn build_demo_script() -> Vec<(u64, u8, u64)> {
    vec![
        (4, sw::LEFT_COIN, 4),
        (12, sw::START_BUTTON, 4),
        (24, sw::JETS_ROLLOVER, 4),        // Skill Shot (+500,000)
        (36, sw::LEFT_SHIELD_TARGET, 4),   // Deplete Deflector Shield (6 -> 4 -> 2 -> 0)
        (46, sw::RIGHT_SHIELD_TARGET, 4),
        (56, sw::SHIELD_HIT, 4),           // Shields Down -> Shield Popper opens!
        (68, sw::SHIELD_POPPER, 4),        // Starts Scene 1: Battle Droid Invasion!
        (74, sw::LEFT_BANK_UPPER, 3),      // Charge Lasers
        (80, sw::LEFT_ACTION_BUTTON, 3),   // Fire Left Laser Action Button!
        (90, sw::RIGHT_ACTION_BUTTON, 3),  // Fire Right Laser Action Button!
        (100, sw::RAMP_MADE_LEFT, 4),      // Scene hits -> Victory!
        (110, sw::RAMP_MADE_RIGHT, 4),
        (120, sw::LEFT_LOOP_UPPER, 4),     // Completes Battle Droid Invasion (+1,500,000)!
        (132, sw::RAMP_MADE_LEFT, 4),      // Spell J-E-D-I -> Darth Maul Saber Duel!
        (142, sw::RAMP_MADE_RIGHT, 4),
        (152, sw::RAMP_MADE_LEFT, 4),
        (162, sw::RAMP_MADE_RIGHT, 4),     // J-E-D-I Complete -> Duel of the Fates + Neon ON!
        (174, sw::RAMP_MADE_LEFT, 4),      // Lightsaber Combo vs Darth Maul!
        (184, sw::RAMP_MADE_RIGHT, 4),
        (194, sw::RAMP_MADE_LEFT, 4),      // Maul Defeated -> Promoted to Jedi Youth!
        (216, sw::LEFT_DROP_TARGET, 4),    // Open Left Saucer
        (224, sw::LEFT_SAUCER, 4),         // Lock Ball 1
        (234, sw::RIGHT_SAUCER, 4),        // Lock Ball 2
        (244, sw::LEFT_SAUCER, 4),         // Lock Ball 3 -> Theed Hangar 3-Ball Multiball!
        (256, sw::RAMP_MADE_LEFT, 4),      // Jackpot #1 (+1,000,000)
        (266, sw::RAMP_MADE_RIGHT, 4),     // Jackpot #2 (+2,000,000)
        (276, sw::SHIELD_POPPER, 4),       // Super Jackpot in Center Scoop (+3,000,000)!
    ]
}

/// Comprehensive full-game showcase exercising Boot, Attract Mode, Skill Shot,
/// all 12 SWE1 holoprojector scenes, Lasers, Jet Bumpers, Darth Maul Duel,
/// Hangar 3-Ball Multiball, End-of-Ball Bonus, and Operator Diagnostics.
pub(crate) fn build_all_sequences_script() -> Vec<(u64, u8, u64)> {
    let mut ev = vec![
        (102, sw::LEFT_COIN, 4),
        (110, sw::START_BUTTON, 4),
        (122, sw::JETS_ROLLOVER, 4), // Skill Shot!
    ];
    // For each of the 12 scenes (launched every 30 frames starting at frame 140),
    // fire shots inside the scene so animations + scoring + completion trigger.
    for idx in 0..12u64 {
        let base = 140 + idx * 30;
        ev.push((base + 8, sw::SHIELD_HIT, 4));
        ev.push((base + 14, sw::RAMP_MADE_LEFT, 4));
        ev.push((base + 20, sw::RAMP_MADE_RIGHT, 4));
    }
    // Lasers + Jet Bumpers (G-U-N-G-A-N -> Super Jets) at frames 500..536
    ev.extend_from_slice(&[
        (502, sw::LEFT_BANK_UPPER, 3),
        (506, sw::LEFT_ACTION_BUTTON, 3),
        (510, sw::RIGHT_ACTION_BUTTON, 3),
        (514, sw::UPPER_JET, 3),
        (518, sw::MIDDLE_JET, 3),
        (522, sw::LOWER_JET, 3),
        (526, sw::UPPER_JET, 3),
        (530, sw::MIDDLE_JET, 3),
        (534, sw::LOWER_JET, 3), // 6th Jet -> Super Jets!
        // Darth Maul Duel combos (launched at frame 540)
        (548, sw::RAMP_MADE_LEFT, 4),
        (556, sw::RAMP_MADE_RIGHT, 4),
        (564, sw::RAMP_MADE_LEFT, 4),
        // Hangar 3-Ball Multiball Jackpots (launched at frame 575)
        (584, sw::RAMP_MADE_LEFT, 4),
        (594, sw::RAMP_MADE_RIGHT, 4),
        (604, sw::SHIELD_POPPER, 4), // Super Jackpot!
        // Drain -> End-of-Ball Bonus
        (630, sw::TROUGH_BALL_1, 6),
        // Operator Diagnostics
        (675, sw::DIAG_ENTER, 4),
        (690, sw::DIAG_UP, 4),
        (702, sw::DIAG_ENTER, 4),
    ]);
    ev
}

pub(crate) fn build_diag_script() -> Vec<(u64, u8, u64)> {
    vec![
        (4, sw::DIAG_ENTER, 4),  // Enter Admin Root Menu
        (12, sw::DIAG_ENTER, 4), // Enter Diagnostics Submenu
        (20, sw::DIAG_UP, 4),    // Select Coil / Solenoid Test
        (28, sw::DIAG_ENTER, 4), // Open Coil Test
        (36, sw::DIAG_UP, 4),    // Pulse Coil 01
        (44, sw::DIAG_UP, 4),    // Pulse Coil 02
        (52, sw::DIAG_UP, 4),    // Pulse Coil 03 (Magnet)
    ]
}

pub(crate) fn build_admin_script() -> Vec<(u64, u8, u64)> {
    vec![
        (4, sw::DIAG_ENTER, 4),  // Open Operator Admin Menu
        (12, sw::DIAG_UP, 4),    // Cursor -> Audits
        (20, sw::DIAG_UP, 4),    // Cursor -> Adjustments
        (28, sw::DIAG_ENTER, 4), // Open Adjustments
        (36, sw::DIAG_UP, 4),    // Cursor -> Balls/Game
        (44, sw::DIAG_ENTER, 4), // Toggle Balls/Game -> 5
    ]
}

/// Scripted switch sequence dedicated to *Boonta Eve Podrace* (`--script podrace`, 300 frames):
/// Starts game, depletes Deflector Shield, enters Center Scoop (`Swe1Scene::Podrace`), plays
/// the 60-frame Boonta Eve starting grid, races through Beggar's Canyon firing Lasers & hitting
/// Checkpoints, and finishes with the full-screen Podrace Victory closeup!
pub(crate) fn build_podrace_script() -> Vec<(u64, u8, u64)> {
    vec![
        (2, sw::LEFT_COIN, 3),
        (6, sw::START_BUTTON, 3),
        (11, sw::LEFT_SHIELD_TARGET, 3),
        (16, sw::RIGHT_SHIELD_TARGET, 3),
        (21, sw::SHIELD_HIT, 3),          // Deflector Shield down -> Center Scoop opens
        (26, sw::SHIELD_POPPER, 4),       // Starts BOONTA EVE PODRACE (6 required hits)!
        (92, sw::LEFT_BANK_UPPER, 3),     // Charge Lasers
        (98, sw::LEFT_FLIPPER_BUTTON, 10),// Steer Podracer Left
        (114, sw::LEFT_ACTION_BUTTON, 3), // Hit #1: Left Laser Blast -> Sebulba explosion!
        (138, sw::RIGHT_FLIPPER_BUTTON, 10),
        (144, sw::RAMP_MADE_LEFT, 4),     // Hit #2: Left Ramp Checkpoint!
        (168, sw::RIGHT_ACTION_BUTTON, 3),// Hit #3: Right Laser Blast!
        (192, sw::RAMP_MADE_RIGHT, 4),    // Hit #4: Right Ramp Checkpoint!
        (216, sw::LEFT_LOOP_UPPER, 4),    // Hit #5: Force Loop Boost!
        (242, sw::RAMP_MADE_LEFT, 4),     // Hit #6: BOONTA EVE PODRACE VICTORY! +1,500,000!
    ]
}

pub(crate) fn build_rfm_demo_script() -> Vec<(u64, u8, u64)> {
    vec![
        (4, sw::LEFT_COIN, 4),
        (10, rfm_sw::START_BUTTON, 4),
        (18, rfm_sw::LEFT_TOP_LANE, 4),        // RFM Skill Shot (+500,000)
        (26, rfm_sw::MARTIAN_TARGET_1, 3),     // 7 Martian Targets -> Martian Attack Multiball!
        (32, rfm_sw::MARTIAN_TARGET_2, 3),
        (38, rfm_sw::MARTIAN_TARGET_3, 3),
        (44, rfm_sw::MARTIAN_TARGET_4, 3),
        (50, rfm_sw::MARTIAN_TARGET_5, 3),
        (56, rfm_sw::MARTIAN_TARGET_6, 3),
        (62, rfm_sw::MARTIAN_TARGET_7, 3),
        (74, rfm_sw::CENTER_DEFLECTOR_PANEL, 4), // Start Mars Kneads Women!
        (84, rfm_sw::LEFT_RAMP_EXIT, 4),
        (94, rfm_sw::RIGHT_RAMP_EXIT, 4),
    ]
}

