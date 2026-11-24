//! Verified *Star Wars Episode I* (`#50069`) hardware switch, coil/drive,
//! lamp, and sound tables lifted from `swe1_0200` (`SwitchTable`, `DriveTable`,
//! `LampTable`, `SwitchXYtable`, `DriveXYtable`, `LampXYtable`).

use pin2k_hal::SoundId;

/// SWE1 logical switch IDs (`0..96`).
pub mod sw {
    pub const START_BUTTON: u8 = 2; // M13
    pub const LEFT_DROP_TARGET: u8 = 4; // M15
    pub const LEFT_OUTLANE: u8 = 5; // M16
    pub const RIGHT_INLANE: u8 = 6; // M17
    pub const SHOOTER_LANE: u8 = 7; // M18
    pub const CAPTIVE_BALL: u8 = 8; // M21
    pub const ALWAYS_CLOSED: u8 = 11; // M24
    pub const RIGHT_DROP_TARGET: u8 = 12; // M25
    pub const LEFT_INLANE: u8 = 13; // M26
    pub const RIGHT_OUTLANE: u8 = 14; // M27
    pub const SNEAKY_LANE: u8 = 15; // M28
    pub const RIGHT_BANK_UPPER: u8 = 16; // M31
    pub const RIGHT_BANK_MIDDLE: u8 = 17; // M32
    pub const RIGHT_BANK_LOWER: u8 = 18; // M33
    pub const LEFT_BANK_UPPER: u8 = 19; // M34
    pub const LEFT_BANK_MIDDLE: u8 = 20; // M35
    pub const LEFT_BANK_LOWER: u8 = 21; // M36
    pub const LEFT_SAUCER: u8 = 22; // M37
    pub const RIGHT_SAUCER: u8 = 23; // M38
    pub const TROUGH_JAM: u8 = 24; // M41
    pub const TROUGH_BALL_1: u8 = 25; // M42
    pub const TROUGH_BALL_2: u8 = 26; // M43
    pub const TROUGH_BALL_3: u8 = 27; // M44
    pub const TROUGH_BALL_4: u8 = 28; // M45
    pub const LEFT_RAMP_ENTER: u8 = 29; // M46
    pub const RIGHT_RAMP_ENTER: u8 = 30; // M47
    pub const SHIELD_POPPER: u8 = 32; // M51
    pub const LEFT_SHIELD_TARGET: u8 = 34; // M53
    pub const RIGHT_SHIELD_TARGET: u8 = 35; // M54
    pub const RAMP_MADE_LEFT: u8 = 36; // M55
    pub const RAMP_MADE_RIGHT: u8 = 37; // M56
    pub const SHIELD_UP: u8 = 38; // M57
    pub const SHIELD_HIT: u8 = 39; // M58
    pub const LEFT_SLING: u8 = 40; // M61
    pub const RIGHT_SLING: u8 = 41; // M62
    pub const UPPER_JET: u8 = 42; // M63
    pub const MIDDLE_JET: u8 = 43; // M64
    pub const LOWER_JET: u8 = 44; // M65
    pub const JETS_ROLLOVER: u8 = 45; // M66
    pub const LEFT_LOOP_UPPER: u8 = 46; // M67
    pub const LEFT_LOOP_ROLLOVER: u8 = 47; // M68

    // Direct cabinet columns 8, 9, 10
    pub const LEFT_COIN: u8 = 64; // P8.0
    pub const CENTER_COIN: u8 = 65; // P8.1
    pub const RIGHT_COIN: u8 = 66; // P8.2
    pub const FOURTH_COIN: u8 = 67; // P8.3

    pub const DIAG_ESCAPE: u8 = 72; // P9.0
    pub const DIAG_DOWN: u8 = 73; // P9.1
    pub const DIAG_UP: u8 = 74; // P9.2
    pub const DIAG_ENTER: u8 = 75; // P9.3
    pub const RIGHT_FLIPPER_EOS: u8 = 76; // P9.4
    pub const LEFT_FLIPPER_EOS: u8 = 77; // P9.5

    pub const SLAM_TILT: u8 = 80; // P10.0
    pub const COIN_DOOR_CLOSED: u8 = 81; // P10.1
    pub const PLUMB_TILT: u8 = 82; // P10.2
    pub const RIGHT_FLIPPER_BUTTON: u8 = 84; // P10.4
    pub const LEFT_FLIPPER_BUTTON: u8 = 85; // P10.5
    pub const RIGHT_ACTION_BUTTON: u8 = 86; // P10.6
    pub const LEFT_ACTION_BUTTON: u8 = 87; // P10.7
}

/// SWE1 PDB solenoid, flasher, flipper, and neon drive IDs (`0..48`).
pub mod drive {
    pub const LEFT_SAUCER: u8 = 0;
    pub const LEFT_DROP_UP: u8 = 1;
    pub const LEFT_DROP_DOWN: u8 = 2;
    pub const MAGNET: u8 = 3;
    pub const RIGHT_DROP_DOWN: u8 = 5;
    pub const RIGHT_DROP_UP: u8 = 6;
    pub const SHIELD_POPPER: u8 = 7;
    pub const TROUGH_EJECT: u8 = 8;
    pub const LEFT_SLING: u8 = 9;
    pub const RIGHT_SLING: u8 = 10;
    pub const UPPER_JET: u8 = 11;
    pub const MIDDLE_JET: u8 = 12;
    pub const LOWER_JET: u8 = 13;
    pub const UPPER_HOTDOG_FL: u8 = 14;
    pub const RIGHT_SAUCER: u8 = 15;
    pub const LOWER_LEFT_HOTDOG_FL: u8 = 16;
    pub const LOWER_RIGHT_HOTDOG_FL: u8 = 17;
    pub const BACK_RIGHT_UPPER_FL: u8 = 18;
    pub const BACK_RIGHT_MID_FL: u8 = 19;
    pub const JET_FLASHER: u8 = 20;
    pub const LEFT_INLANE_FL: u8 = 21;
    pub const RIGHT_INLANE_FL: u8 = 22;
    pub const BACK_MIDDLE_FL: u8 = 23;
    pub const BACK_RIGHT_LOWER_FL: u8 = 24;
    pub const BACK_LEFT_UPPER_FL: u8 = 25;
    pub const BACK_LEFT_MID_FL: u8 = 26;
    pub const BACK_LEFT_LOWER_FL: u8 = 27;
    pub const RIGHT_FLIPPER_POWER: u8 = 32;
    pub const RIGHT_FLIPPER_HOLD: u8 = 33;
    pub const LEFT_FLIPPER_POWER: u8 = 34;
    pub const LEFT_FLIPPER_HOLD: u8 = 35;
    pub const SHIELD_POWER: u8 = 36;
    pub const SHIELD_HOLD: u8 = 37;
    pub const LEFT_LASER_FL: u8 = 38;
    pub const RIGHT_LASER_FL: u8 = 39;
    pub const LIGHTSABER_NEON: u8 = 40;
    pub const TICKET_DISPENSER: u8 = 47;
}

/// SWE1 Dual 8×8 Controlled Lamp IDs (`0..63` Matrix A, `64..127` Matrix B).
pub mod lamp {
    pub const START_BUTTON: u8 = 2;
    pub const COIN_DOOR: u8 = 11;

    // 12-Lamp Deflector Shield Bank (Matrix A)
    pub const SHIELD_LAMPS: [u8; 12] = [4, 5, 6, 7, 12, 13, 14, 15, 20, 21, 22, 23];

    // J-E-D-I Spirit Letters (Matrix A)
    pub const JEDI_J: u8 = 29;
    pub const JEDI_E: u8 = 28;
    pub const JEDI_D: u8 = 31;
    pub const JEDI_I: u8 = 30;
    pub const JEDI_LETTERS: [u8; 4] = [JEDI_J, JEDI_E, JEDI_D, JEDI_I];

    pub const LEFT_LOOP_ROLLOVER: u8 = 38;
    pub const LEFT_SAUCER: u8 = 39;
    pub const RIGHT_STANDUPS: [u8; 3] = [40, 41, 42];
    pub const LEFT_STANDUPS: [u8; 3] = [45, 44, 43];
    pub const MIDDLE_JET: u8 = 62;

    // Matrix B (`64..127`)
    pub const NABOO_SHIP_BODY: [u8; 9] = [64, 65, 67, 72, 73, 74, 80, 81, 88];
    pub const BONUS_X2: u8 = 82;
    pub const BONUS_X3: u8 = 83;
    pub const BONUS_X4: u8 = 75;
    pub const BONUS_X5: u8 = 66;

    pub const JEDI_YOUTH: u8 = 84;
    pub const JEDI_KNIGHT: u8 = 93;
    pub const JEDI_MASTER: u8 = 76;
    pub const JEDI_SPIRIT: u8 = 68;

    pub const FIRE_LASERS_LEFT: u8 = 92;
    pub const FIRE_LASERS_RIGHT: u8 = 77;
    pub const RIGHT_SAUCER: u8 = 78;
    pub const EXTRA_BALL: u8 = 79;
    pub const JETS_ROLLOVER: u8 = 70;
    pub const SHOOTER: u8 = 86;
    pub const SHOOT_AGAIN: u8 = 94;
    pub const LEFT_FLIPPER: u8 = 95;
    pub const RIGHT_FLIPPER: u8 = 102;
    pub const LEFT_SAUCER_INSERT: u8 = 97;
    pub const RIGHT_SAUCER_INSERT: u8 = 98;
}

/// Verified DCS-2 sound command IDs from `swe1_sound.bin` and SWE1 `.text` calls.
pub mod sound {
    use super::SoundId;
    pub const BOOT_BONG: SoundId = SoundId(0x003A);
    pub const MAIN_THEME_LOOP: SoundId = SoundId(0x0001);
    pub const BATTLE_MUSIC_LOOP: SoundId = SoundId(0x0002);
    pub const PODRACE_MUSIC_LOOP: SoundId = SoundId(0x0006);
    pub const DUEL_OF_THE_FATES_LOOP: SoundId = SoundId(0x0007);
    pub const MULTIBALL_MUSIC_LOOP: SoundId = SoundId(0x0009);
    pub const CANTINA_MUSIC_LOOP: SoundId = SoundId(0x000E);
    pub const HANGAR_MUSIC_LOOP: SoundId = SoundId(0x0010);
    pub const ATTRACT_MUSIC: SoundId = SoundId(0x012F);
    pub const GAME_OVER_MUSIC: SoundId = SoundId(0x00E1);

    pub const COIN_CREDIT: SoundId = SoundId(0x0004);
    pub const LASER_FIRE: SoundId = SoundId(0x000D);
    pub const SABER_CLASH: SoundId = SoundId(0x0014);
    pub const EXPLOSION: SoundId = SoundId(0x0015);
    pub const SHIELD_HIT_SFX: SoundId = SoundId(0x0016);
    pub const JET_BUMPER_SFX: SoundId = SoundId(0x0018);
}

/// Descriptor for a SWE1 drive (`DriveTable` + `DriveXYtable`).
#[derive(Debug, Clone, Copy)]
pub struct DriveInfo {
    pub id: u8,
    pub name: &'static str,
    pub default_pulse_ms: u8,
    pub x: u8,
    pub y: u8,
    pub is_flasher: bool,
}

pub const SWE1_DRIVES: [DriveInfo; 33] = [
    DriveInfo { id: 0, name: "Left Saucer", default_pulse_ms: 64, x: 20, y: 32, is_flasher: false },
    DriveInfo { id: 1, name: "Left Drop Target Up", default_pulse_ms: 64, x: 29, y: 22, is_flasher: false },
    DriveInfo { id: 2, name: "Left Drop Target Down", default_pulse_ms: 64, x: 29, y: 22, is_flasher: false },
    DriveInfo { id: 3, name: "Magnet", default_pulse_ms: 64, x: 68, y: 54, is_flasher: false },
    DriveInfo { id: 5, name: "Right Drop Target Down", default_pulse_ms: 64, x: 67, y: 22, is_flasher: false },
    DriveInfo { id: 6, name: "Right Drop Target Up", default_pulse_ms: 64, x: 67, y: 22, is_flasher: false },
    DriveInfo { id: 7, name: "Shield Popper", default_pulse_ms: 64, x: 46, y: 16, is_flasher: false },
    DriveInfo { id: 8, name: "Trough Eject", default_pulse_ms: 64, x: 88, y: 89, is_flasher: false },
    DriveInfo { id: 9, name: "Left Sling", default_pulse_ms: 48, x: 25, y: 73, is_flasher: false },
    DriveInfo { id: 10, name: "Right Sling", default_pulse_ms: 48, x: 69, y: 73, is_flasher: false },
    DriveInfo { id: 11, name: "Upper Jet", default_pulse_ms: 40, x: 89, y: 21, is_flasher: false },
    DriveInfo { id: 12, name: "Middle Jet", default_pulse_ms: 40, x: 77, y: 32, is_flasher: false },
    DriveInfo { id: 13, name: "Lower Jet", default_pulse_ms: 40, x: 89, y: 41, is_flasher: false },
    DriveInfo { id: 14, name: "Upper Hotdog Flashers", default_pulse_ms: 24, x: 24, y: 8, is_flasher: true },
    DriveInfo { id: 15, name: "Right Saucer", default_pulse_ms: 64, x: 71, y: 38, is_flasher: false },
    DriveInfo { id: 16, name: "Lower Left Hotdog Fl.", default_pulse_ms: 24, x: 29, y: 61, is_flasher: true },
    DriveInfo { id: 17, name: "Lower Right Hotdog Fl.", default_pulse_ms: 24, x: 66, y: 61, is_flasher: true },
    DriveInfo { id: 18, name: "Back Panel Right/Upper Fl.", default_pulse_ms: 24, x: 93, y: 4, is_flasher: true },
    DriveInfo { id: 19, name: "Back Panel Right/Mid Fl.", default_pulse_ms: 24, x: 93, y: 8, is_flasher: true },
    DriveInfo { id: 20, name: "Jet Flasher", default_pulse_ms: 32, x: 86, y: 33, is_flasher: true },
    DriveInfo { id: 21, name: "Left Inlanes Flasher", default_pulse_ms: 24, x: 13, y: 79, is_flasher: true },
    DriveInfo { id: 22, name: "Right Inlanes Flasher", default_pulse_ms: 24, x: 81, y: 79, is_flasher: true },
    DriveInfo { id: 23, name: "Back Panel Middle Fl.", default_pulse_ms: 24, x: 50, y: 4, is_flasher: true },
    DriveInfo { id: 32, name: "Right Flipper Power", default_pulse_ms: 40, x: 63, y: 86, is_flasher: false },
    DriveInfo { id: 33, name: "Right Flipper Hold", default_pulse_ms: 64, x: 63, y: 86, is_flasher: false },
    DriveInfo { id: 34, name: "Left Flipper Power", default_pulse_ms: 40, x: 32, y: 86, is_flasher: false },
    DriveInfo { id: 35, name: "Left Flipper Hold", default_pulse_ms: 64, x: 32, y: 86, is_flasher: false },
    DriveInfo { id: 36, name: "Shield Power", default_pulse_ms: 64, x: 46, y: 21, is_flasher: false },
    DriveInfo { id: 37, name: "Shield Hold", default_pulse_ms: 64, x: 46, y: 21, is_flasher: false },
    DriveInfo { id: 38, name: "Left Laser Flasher", default_pulse_ms: 32, x: 41, y: 57, is_flasher: true },
    DriveInfo { id: 39, name: "Right Laser Flasher", default_pulse_ms: 32, x: 54, y: 57, is_flasher: true },
    DriveInfo { id: 40, name: "Lightsaber Neon", default_pulse_ms: 128, x: 88, y: 68, is_flasher: true },
    DriveInfo { id: 47, name: "Ticket Dispenser", default_pulse_ms: 64, x: 5, y: 95, is_flasher: false },
];

/// Returns the official Williams English name for a SWE1 logical switch (`0..96`).
pub fn switch_name(sw_id: u8) -> &'static str {
    match sw_id {
        2 => "Start Button",
        4 => "Left Drop Target",
        5 => "Left Outlane",
        6 => "Right Inlane",
        7 => "Shooter Lane",
        8 => "Captive Ball",
        11 => "Always Closed",
        12 => "Right Drop Target",
        13 => "Left Inlane",
        14 => "Right Outlane",
        15 => "Sneaky Lane",
        16 => "Right Bank: Upper",
        17 => "Right Bank: Middle",
        18 => "Right Bank: Lower",
        19 => "Left Bank: Upper",
        20 => "Left Bank: Middle",
        21 => "Left Bank: Lower",
        22 => "Left Saucer",
        23 => "Right Saucer",
        24 => "Trough Jam",
        25 => "Trough Ball 1",
        26 => "Trough Ball 2",
        27 => "Trough Ball 3",
        28 => "Trough Ball 4",
        29 => "Left Ramp Enter",
        30 => "Right Ramp Enter",
        32 => "Shield Popper",
        34 => "Left Shield Target",
        35 => "Right Shield Target",
        36 => "Ramp Made Left",
        37 => "Ramp Made Right",
        38 => "Shield Up",
        39 => "Shield Hit",
        40 => "Left Sling",
        41 => "Right Sling",
        42 => "Upper Jet",
        43 => "Middle Jet",
        44 => "Lower Jet",
        45 => "Jets Rollover",
        46 => "Left Loop Upper",
        47 => "Left Loop Rollover",
        64 => "Left Coin Slot",
        65 => "Center Coin Slot",
        66 => "Right Coin Slot",
        67 => "4th Coin Option",
        72 => "Escape Button",
        73 => "Down Button",
        74 => "Up Button",
        75 => "Enter Button",
        76 => "Right Flipper EOS",
        77 => "Left Flipper EOS",
        80 => "Slam Tilt",
        81 => "Coin Door Closed",
        82 => "Plumb Bob Tilt",
        84 => "Right Flipper Button",
        85 => "Left Flipper Button",
        86 => "Right Action Button",
        87 => "Left Action Button",
        _ => "Not Used",
    }
}

/// Returns the official Williams English name for a SWE1 drive (`0..48`).
pub fn drive_name(drive_id: u8) -> &'static str {
    for d in &SWE1_DRIVES {
        if d.id == drive_id {
            return d.name;
        }
    }
    "Not Used"
}
