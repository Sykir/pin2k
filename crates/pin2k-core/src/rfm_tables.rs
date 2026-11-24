//! Verified *Revenge From Mars* (`#50070`) hardware switch, coil/drive,
//! lamp, and sound tables lifted from `pin2000_50070_0190_game.rom`.

use pin2k_hal::SoundId;

/// RFM logical switch IDs (`0..96`).
pub mod rfm_sw {
    pub const RIGHT_RAMP_ENTRANCE: u8 = 0;
    pub const LEFT_RAMP_EXIT: u8 = 1;
    pub const START_BUTTON: u8 = 2;
    pub const DROP_TARGET_DOWN: u8 = 4;
    pub const LEFT_OUTLANE: u8 = 5;
    pub const RIGHT_RETURN_LANE: u8 = 6;
    pub const SHOOTER_LANE: u8 = 7;
    pub const LAUNCH_BUTTON: u8 = 10;
    pub const LEFT_LOOP_LOW: u8 = 12;
    pub const LEFT_RETURN_LANE: u8 = 13;
    pub const RIGHT_OUTLANE: u8 = 14;
    pub const RIGHT_RAMP_EXIT: u8 = 15;
    pub const CENTER_LOOP_REED_BOT: u8 = 16;
    pub const CENTER_LOOP_REED_TOP: u8 = 17;
    pub const CENTER_TARGET_4: u8 = 18;
    pub const CENTER_TARGET_3: u8 = 19;
    pub const CENTER_TARGET_2: u8 = 20;
    pub const CENTER_TARGET_1: u8 = 21;
    pub const MARTIAN_TARGET_4: u8 = 22;
    pub const UP_DOWN_RAMP_UP: u8 = 23;
    pub const TROUGH_JAM: u8 = 24;
    pub const TROUGH_BALL_1: u8 = 25;
    pub const TROUGH_BALL_2: u8 = 26;
    pub const TROUGH_BALL_3: u8 = 27;
    pub const TROUGH_BALL_4: u8 = 28;
    pub const RIGHT_POPPER: u8 = 29;
    pub const JET_EXIT: u8 = 30;
    pub const RIGHT_LOCKUP_1: u8 = 32;
    pub const LEFT_RAMP_ENTRANCE: u8 = 33;
    pub const LEFT_SLING: u8 = 40;
    pub const RIGHT_SLING: u8 = 41;
    pub const LEFT_JET: u8 = 42;
    pub const RIGHT_JET: u8 = 43;
    pub const BOTTOM_JET: u8 = 44;
    pub const RIGHT_LOOP_LOW: u8 = 46;
    pub const RIGHT_LOOP_HIGH: u8 = 47;
    pub const MARTIAN_TARGET_3: u8 = 48;
    pub const MARTIAN_TARGET_2: u8 = 49;
    pub const MARTIAN_TARGET_1: u8 = 50;
    pub const CENTER_LOOP_ROLLOVER: u8 = 51;
    pub const CENTER_DEFLECTOR_PANEL: u8 = 52;
    pub const RIGHT_TOP_LANE: u8 = 53;
    pub const LEFT_TOP_LANE: u8 = 54;
    pub const LEFT_LOOP_HIGH: u8 = 55;
    pub const MARTIAN_TARGET_7: u8 = 60;
    pub const MARTIAN_TARGET_6: u8 = 61;
    pub const MARTIAN_TARGET_5: u8 = 62;
}

/// RFM PDB solenoid, flasher, and flipper drive IDs (`0..48`).
pub mod rfm_drive {
    pub const LEFT_MARTIAN: u8 = 0;
    pub const RIGHT_MARTIAN: u8 = 1;
    pub const JET_EXIT_POST: u8 = 2;
    pub const RIGHT_GATE: u8 = 3;
    pub const LEFT_GATE: u8 = 4;
    pub const DROP_TARGET_DOWN: u8 = 5;
    pub const DROP_TARGET_UP: u8 = 6;
    pub const RIGHT_POPPER: u8 = 7;
    pub const TROUGH_EJECT: u8 = 8;
    pub const LEFT_SLING: u8 = 9;
    pub const RIGHT_SLING: u8 = 10;
    pub const LEFT_JET: u8 = 11;
    pub const RIGHT_JET: u8 = 12;
    pub const BOTTOM_JET: u8 = 13;
    pub const AUTOPLUNGER: u8 = 14;
    pub const RIGHT_LOCKUP: u8 = 15;
    pub const CENTER_ARROW_FL: u8 = 16;
    pub const RIGHT_POPPER_FL: u8 = 21;
    pub const LEFT_ARCH_FL: u8 = 22;
    pub const RIGHT_ARCH_FL: u8 = 24;
    pub const LEFT_MARTIAN_FL: u8 = 25;
    pub const RIGHT_MARTIAN_FL: u8 = 26;
    pub const ATTACK_MARS_FL: u8 = 27;
    pub const RIGHT_FLIPPER_POWER: u8 = 32;
    pub const RIGHT_FLIPPER_HOLD: u8 = 33;
    pub const LEFT_FLIPPER_POWER: u8 = 34;
    pub const LEFT_FLIPPER_HOLD: u8 = 35;
    pub const LOCK_DIVERTER_POWER: u8 = 36;
    pub const LOCK_DIVERTER_HOLD: u8 = 37;
    pub const UP_DOWN_RAMP_POWER: u8 = 38;
    pub const UP_DOWN_RAMP_HOLD: u8 = 39;
    pub const TICKET_DISPENSER: u8 = 47;
}

/// RFM Dual 8×8 Controlled Lamp IDs (`0..63` Matrix A, `64..127` Matrix B).
pub mod rfm_lamp {
    pub const START_BUTTON: u8 = 2;
    pub const MARTIAN_TARGET_4: u8 = 6;
    pub const EXTRA_BALL: u8 = 25;
    pub const MARTIAN_ATTACK: u8 = 26;
    pub const SHOOT_AGAIN: u8 = 36;
    pub const SAUCER: u8 = 75;
    pub const MARS_KNEADS_WOMEN: u8 = 80;
    pub const MARTIAN_HAPPY_HOUR: u8 = 88;
    pub const SAUCER_RIM: [u8; 9] = [91, 92, 93, 94, 86, 85, 84, 83, 82];
    pub const MARTIAN_TARGETS: [u8; 7] = [109, 108, 107, 6, 104, 105, 106];
    pub const RIGHT_MARTIAN_EYE: u8 = 110;
    pub const LEFT_MARTIAN_EYE: u8 = 111;
}

pub mod rfm_sound {
    use super::SoundId;
    pub const BOOT_BONG: SoundId = SoundId(0x003A);
    pub const MAIN_THEME: SoundId = SoundId(0x0001);
    pub const WAVE_MUSIC: SoundId = SoundId(0x0002);
    pub const MULTIBALL_MUSIC: SoundId = SoundId(0x0007);
    pub const LASER_SFX: SoundId = SoundId(0x000D);
    pub const EXPLOSION_SFX: SoundId = SoundId(0x0015);
}

pub fn rfm_switch_name(sw_id: u8) -> &'static str {
    match sw_id {
        0 => "Right Ramp Entrance",
        1 => "Left Ramp Exit",
        2 => "Start Button",
        4 => "Drop Target Down",
        5 => "Left Outlane",
        6 => "Right Return Lane",
        7 => "Shooter Lane",
        10 => "Launch Button",
        12 => "Left Loop (Low)",
        13 => "Left Return Lane",
        14 => "Right Outlane",
        15 => "Right Ramp Exit",
        16 => "Center Loop Reed (Bottom)",
        17 => "Center Loop Reed (Top)",
        18 => "Center Target 4",
        19 => "Center Target 3",
        20 => "Center Target 2",
        21 => "Center Target 1",
        22 => "Martian Target 4 (Center)",
        23 => "Up/Down Ramp Up",
        24 => "Trough Jam",
        25 => "Trough Ball 1",
        26 => "Trough Ball 2",
        27 => "Trough Ball 3",
        28 => "Trough Ball 4",
        29 => "Right Popper",
        30 => "Jet Exit",
        32 => "Right Lockup 1",
        33 => "Left Ramp Entrance",
        40 => "Left Slingshot",
        41 => "Right Slingshot",
        42 => "Left Jet",
        43 => "Right Jet",
        44 => "Bottom Jet",
        46 => "Right Loop (Low)",
        47 => "Right Loop (High)",
        48 => "Martian Target 3 (L. Top)",
        49 => "Martian Target 2 (L. Mid)",
        50 => "Martian Target 1 (L. Bot)",
        51 => "Center Loop Rollover",
        52 => "Center Deflector Panel",
        53 => "Right Top Lane",
        54 => "Left Top Lane",
        55 => "Left Loop (High)",
        60 => "Martian Target 7 (R. Bot)",
        61 => "Martian Target 6 (R. Mid)",
        62 => "Martian Target 5 (R. Top)",
        64..=87 => crate::swe1_tables::switch_name(sw_id),
        _ => "Not Used",
    }
}

pub fn rfm_drive_name(drive_id: u8) -> &'static str {
    match drive_id {
        0 => "Left Martian",
        1 => "Right Martian",
        2 => "Jet Exit Post",
        3 => "Right Gate",
        4 => "Left Gate",
        5 => "Drop Target Down",
        6 => "Drop Target Up",
        7 => "Right Popper",
        8 => "Trough Eject",
        9 => "Left Sling",
        10 => "Right Sling",
        11 => "Left Jet",
        12 => "Right Jet",
        13 => "Bottom Jet",
        14 => "Autoplunger",
        15 => "Right Lockup",
        16 => "Center Arrow Flasher",
        21 => "Right Popper Flasher",
        22 => "Left Arch Flasher",
        24 => "Right Arch Flasher",
        25 => "Left Martian Flasher",
        26 => "Right Martian Flasher",
        27 => "Attack Mars Flasher",
        32 => "Right Flipper Power",
        33 => "Right Flipper Hold",
        34 => "Left Flipper Power",
        35 => "Left Flipper Hold",
        36 => "Lock Diverter Power",
        37 => "Lock Diverter Hold",
        38 => "Up/Down Ramp Power",
        39 => "Up/Down Ramp Hold",
        47 => "Ticket Dispenser",
        _ => "Not Used",
    }
}
