// Thing on a Spring - Rob Hubbard - 1985 Gremlin Graphics

use super::rhsongs::{InstrFx, Instrument, RhSongs, SidT, SoundFx};
#[allow(dead_code)]
pub static RHSONGS: RhSongs = RhSongs {
    version: 10,
    total: 2,
    tracks: &TRACKS,
    patterns: &PATTERNS,
    instruments: &INSTRUMENTS,
    soundfx: &SOUNDFX,
    instrfx: &INSTRFX,
    resetspd: 1,
    skydive_v1_when: 0,
    skydive_v1_add: 0,
};

#[allow(dead_code)]
pub static TRACK_0: [u8; 70] = [
    0, 1, 2, 3, 4, 9, 30, 9, 31, 0, 10, 32, 10, 33, 10, 32, 10, 33, 10, 32, 10, 33, 13, 1, 2, 3, 4,
    9, 30, 9, 31, 0, 10, 32, 10, 33, 10, 32, 10, 33, 20, 25, 25, 25, 25, 21, 21, 21, 26, 10, 32,
    10, 33, 20, 20, 10, 32, 10, 33, 20, 20, 25, 25, 25, 25, 25, 25, 25, 25, 255,
];

#[allow(dead_code)]
pub static TRACK_1: [u8; 97] = [
    27, 27, 27, 27, 27, 19, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 27, 27,
    27, 27, 27, 27, 27, 26, 28, 28, 28, 28, 28, 19, 14, 14, 14, 15, 29, 29, 17, 17, 17, 17, 17, 17,
    17, 21, 17, 17, 17, 17, 17, 17, 17, 29, 29, 27, 27, 27, 27, 27, 27, 27, 26, 28, 28, 28, 28, 28,
    19, 14, 14, 24, 23, 23, 23, 23, 23, 23, 14, 24, 24, 14, 24, 24, 25, 25, 25, 25, 25, 25, 25, 25,
    255,
];

#[allow(dead_code)]
pub static TRACK_2: [u8; 81] = [
    18, 5, 6, 7, 8, 11, 34, 11, 35, 18, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 16,
    5, 6, 7, 8, 11, 34, 11, 35, 18, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 22, 22, 22, 22, 22, 22,
    29, 29, 29, 29, 29, 29, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 25, 25,
    25, 25, 25, 25, 25, 25, 255,
];

#[allow(dead_code)]
pub static TRACKS: [&[&[u8]; 3]; 1] = [&[&TRACK_0, &TRACK_1, &TRACK_2]];

#[allow(dead_code)]
pub static PATTERNS: [&[u8]; 36] = [
    &PATTERN_0,
    &PATTERN_1,
    &PATTERN_2,
    &PATTERN_3,
    &PATTERN_4,
    &PATTERN_5,
    &PATTERN_6,
    &PATTERN_7,
    &PATTERN_8,
    &PATTERN_9,
    &PATTERN_10,
    &PATTERN_11,
    &PATTERN_12,
    &PATTERN_13,
    &PATTERN_14,
    &PATTERN_15,
    &PATTERN_16,
    &PATTERN_17,
    &PATTERN_18,
    &PATTERN_19,
    &PATTERN_20,
    &PATTERN_21,
    &PATTERN_22,
    &PATTERN_23,
    &PATTERN_24,
    &PATTERN_25,
    &PATTERN_26,
    &PATTERN_27,
    &PATTERN_28,
    &PATTERN_29,
    &PATTERN_30,
    &PATTERN_31,
    &PATTERN_32,
    &PATTERN_33,
    &PATTERN_34,
    &PATTERN_35,
];
#[allow(dead_code)]
pub static PATTERN_0: [u8; 121] = [
    0x85, 0x5, 0x58, 0x5, 0x4C, 0x5, 0x58, 0x2, 0x4C, 0x5, 0x58, 0x2, 0x4C, 0x5, 0x58, 0x5, 0x4C,
    0x5, 0x58, 0x5, 0x58, 0x5, 0x4C, 0x5, 0x58, 0x2, 0x4C, 0x2, 0x58, 0x4B, 0x8B, 0xD, 0x1B, 0x85,
    0x5, 0x58, 0x5, 0x4C, 0x5, 0x58, 0x2, 0x4C, 0x5, 0x58, 0x2, 0x4C, 0x5, 0x58, 0x5, 0x4C, 0x5,
    0x58, 0x5, 0x58, 0x5, 0x4C, 0x5, 0x58, 0x2, 0x4C, 0x2, 0x58, 0x4B, 0x8B, 0xD, 0x17, 0x85, 0x5,
    0x58, 0x5, 0x4C, 0x5, 0x58, 0x2, 0x4C, 0x5, 0x58, 0x2, 0x4C, 0x5, 0x58, 0x5, 0x4C, 0x5, 0x58,
    0x5, 0x58, 0x5, 0x4C, 0x5, 0x58, 0x2, 0x4C, 0x2, 0x58, 0x82, 0x0, 0x29, 0x5, 0x29, 0x2, 0x29,
    0x2, 0x2A, 0x5, 0x2A, 0x2, 0x2A, 0x2, 0x2B, 0x5, 0x2B, 0x2, 0x2B, 0x2, 0x2C, 0x5, 0x2C, 0x2,
    0x2C, 0xFF,
];
#[allow(dead_code)]
pub static PATTERN_1: [u8; 71] = [
    0x2, 0x3C, 0x2, 0x3D, 0x5, 0x40, 0x5, 0x40, 0x2, 0x3D, 0x2, 0x40, 0x2, 0x42, 0x2, 0x40, 0x5,
    0x3C, 0x2, 0x3B, 0x8, 0x39, 0x2, 0x3C, 0x2, 0x3D, 0x5, 0x40, 0x2, 0x40, 0x2, 0x40, 0x2, 0x3D,
    0x2, 0x40, 0x2, 0x45, 0x2, 0x45, 0x5, 0x43, 0x8, 0x40, 0x2, 0x40, 0x2, 0x40, 0x2, 0x42, 0x5,
    0x45, 0x5, 0x45, 0x5, 0x45, 0x5, 0x42, 0x5, 0x40, 0x5, 0x3C, 0x2, 0x3B, 0x22, 0x39, 0x37, 0x39,
    0x17, 0x39, 0xFF,
];
#[allow(dead_code)]
pub static PATTERN_2: [u8; 73] = [
    0x2, 0x3C, 0x2, 0x3D, 0x5, 0x40, 0x5, 0x40, 0x2, 0x3D, 0x2, 0x40, 0x2, 0x42, 0x2, 0x40, 0x5,
    0x3C, 0x2, 0x3B, 0x8, 0x39, 0x2, 0x3C, 0x2, 0x3D, 0x5, 0x40, 0x5, 0x42, 0x2, 0x3D, 0x2, 0x40,
    0x4B, 0x42, 0x2, 0x40, 0x2, 0x42, 0x2, 0x43, 0x2, 0x44, 0x2, 0x44, 0x2, 0x43, 0x2, 0x42, 0x5,
    0x40, 0x2, 0x3D, 0x5, 0x40, 0x2, 0x40, 0x2, 0x3F, 0x2, 0x3E, 0x5, 0x3C, 0x2, 0x3B, 0x22, 0x39,
    0x37, 0x39, 0x17, 0x39, 0xFF,
];
#[allow(dead_code)]
pub static PATTERN_3: [u8; 63] = [
    0x88, 0x7, 0x40, 0x2, 0x40, 0x2, 0x42, 0x2, 0x40, 0x2, 0x43, 0xB, 0x44, 0x42, 0x5, 0x42, 0x5,
    0x3D, 0x5, 0x40, 0x5, 0x40, 0x2, 0x41, 0x2, 0x42, 0x2, 0x3D, 0x17, 0x40, 0x42, 0x8, 0x40, 0x2,
    0x40, 0x2, 0x42, 0x2, 0x40, 0x2, 0x43, 0xB, 0x44, 0x42, 0x5, 0x42, 0x5, 0x41, 0x5, 0x42, 0x5,
    0x42, 0x2, 0x42, 0x2, 0x40, 0x2, 0x3D, 0xB, 0x39, 0x42, 0x4B, 0xFF,
];
#[allow(dead_code)]
pub static PATTERN_4: [u8; 80] = [
    0x2, 0x39, 0x2, 0x3A, 0x5, 0x3D, 0x5, 0x3D, 0x2, 0x3A, 0x2, 0x3D, 0x2, 0x44, 0x2, 0x43, 0x5,
    0x42, 0x2, 0x42, 0x2, 0x41, 0x5, 0x40, 0x5, 0x3F, 0x5, 0x3F, 0x2, 0x3F, 0x2, 0x3E, 0x2, 0x3F,
    0x17, 0x3B, 0x42, 0x2, 0x37, 0x2, 0x38, 0x5, 0x3B, 0x5, 0x3B, 0x2, 0x38, 0x2, 0x3B, 0x2, 0x40,
    0x2, 0x3F, 0x5, 0x3E, 0x2, 0x40, 0x2, 0x41, 0x2, 0x42, 0x2, 0x44, 0x5, 0x45, 0x5, 0x45, 0x2,
    0x42, 0x2, 0x40, 0x5, 0x3D, 0x5, 0x40, 0x8B, 0xD, 0x40, 0x45, 0xFF,
];
#[allow(dead_code)]
pub static PATTERN_5: [u8; 85] = [
    0x5, 0x21, 0x5, 0x21, 0x2, 0x24, 0x2, 0x25, 0x2, 0x2A, 0x2, 0x28, 0x5, 0x21, 0x5, 0x21, 0x2,
    0x1E, 0x2, 0x1F, 0x5, 0x20, 0x5, 0x21, 0x5, 0x21, 0x2, 0x24, 0x2, 0x25, 0x2, 0x2A, 0x2, 0x28,
    0x5, 0x21, 0x5, 0x21, 0x2, 0x2A, 0x2, 0x29, 0x2, 0x28, 0x2, 0x20, 0x5, 0x21, 0x5, 0x21, 0x2,
    0x24, 0x2, 0x25, 0x2, 0x2A, 0x2, 0x28, 0x5, 0x21, 0x5, 0x21, 0x2, 0x1E, 0x2, 0x1F, 0x5, 0x20,
    0x5, 0x21, 0x5, 0x2D, 0x5, 0x1E, 0x5, 0x2A, 0x5, 0x1F, 0x5, 0x2B, 0x5, 0x20, 0x5, 0x2C, 0xFF,
];
#[allow(dead_code)]
pub static PATTERN_6: [u8; 83] = [
    0x5, 0x21, 0x5, 0x21, 0x2, 0x24, 0x2, 0x25, 0x2, 0x2A, 0x2, 0x28, 0x5, 0x21, 0x5, 0x21, 0x2,
    0x1E, 0x2, 0x1F, 0x5, 0x20, 0x5, 0x21, 0x5, 0x21, 0x2, 0x24, 0x2, 0x25, 0x2, 0x2A, 0x2, 0x28,
    0x5, 0x21, 0x5, 0x21, 0x2, 0x2A, 0x2, 0x29, 0x2, 0x28, 0x2, 0x20, 0x5, 0x1C, 0x5, 0x1C, 0x2,
    0x1F, 0x2, 0x20, 0x2, 0x25, 0x2, 0x23, 0x5, 0x1C, 0x5, 0x1C, 0x2, 0x19, 0x2, 0x1A, 0x5, 0x1C,
    0x5, 0x21, 0x5, 0x21, 0x2, 0x24, 0x2, 0x25, 0x2, 0x2A, 0x2, 0x28, 0x17, 0x21, 0xFF,
];
#[allow(dead_code)]
pub static PATTERN_7: [u8; 89] = [
    0x5, 0x1C, 0x5, 0x1C, 0x2, 0x1F, 0x2, 0x20, 0x2, 0x25, 0x2, 0x23, 0x5, 0x1C, 0x5, 0x1C, 0x2,
    0x19, 0x2, 0x1A, 0x5, 0x1C, 0x5, 0x21, 0x5, 0x21, 0x2, 0x24, 0x2, 0x25, 0x2, 0x2A, 0x2, 0x28,
    0x5, 0x21, 0x5, 0x21, 0x2, 0x2A, 0x2, 0x29, 0x2, 0x28, 0x2, 0x20, 0x5, 0x1C, 0x5, 0x1C, 0x2,
    0x1F, 0x2, 0x20, 0x2, 0x25, 0x2, 0x23, 0x5, 0x1C, 0x5, 0x1C, 0x2, 0x19, 0x2, 0x1A, 0x5, 0x1C,
    0x5, 0x21, 0x5, 0x21, 0x2, 0x24, 0x2, 0x25, 0x2, 0x2A, 0x2, 0x28, 0x5, 0x21, 0x5, 0x21, 0x5,
    0x20, 0x5, 0x1F, 0xFF,
];
#[allow(dead_code)]
pub static PATTERN_8: [u8; 83] = [
    0x5, 0x1E, 0x5, 0x1E, 0x2, 0x21, 0x2, 0x22, 0x2, 0x27, 0x2, 0x25, 0x5, 0x1E, 0x5, 0x1E, 0x2,
    0x27, 0x2, 0x26, 0x5, 0x25, 0x5, 0x23, 0x5, 0x23, 0x2, 0x26, 0x2, 0x27, 0x2, 0x2C, 0x2, 0x2A,
    0x5, 0x23, 0x5, 0x23, 0x2, 0x2C, 0x2, 0x2B, 0x5, 0x2A, 0x5, 0x1C, 0x5, 0x1C, 0x2, 0x1F, 0x2,
    0x20, 0x2, 0x25, 0x2, 0x23, 0x5, 0x1C, 0x5, 0x1C, 0x2, 0x25, 0x2, 0x24, 0x5, 0x23, 0x5, 0x21,
    0x5, 0x21, 0x2, 0x24, 0x2, 0x25, 0x5, 0x1E, 0x5, 0x1C, 0x8B, 0xD, 0x1C, 0x45, 0xFF,
];
#[allow(dead_code)]
pub static PATTERN_9: [u8; 54] = [
    0x85, 0x4, 0x2D, 0x5, 0x2D, 0x2, 0x40, 0x2, 0x3F, 0x2, 0x40, 0x2, 0x3F, 0x2, 0x40, 0x5, 0x42,
    0x8, 0x40, 0x5, 0x3E, 0x5, 0x2D, 0x5, 0x2D, 0x2, 0x3D, 0x2, 0x3C, 0x2, 0x3D, 0x2, 0x3C, 0x2,
    0x3D, 0x5, 0x3E, 0x8, 0x3D, 0x5, 0x3C, 0x5, 0x2F, 0x5, 0x2F, 0x2, 0x3B, 0x2, 0x3A, 0x2, 0x3B,
    0x2, 0x3A, 0xFF,
];
#[allow(dead_code)]
pub static PATTERN_10: [u8; 35] = [
    0x82, 0xA, 0x3C, 0x2, 0x3D, 0x5, 0x40, 0x5, 0x40, 0x2, 0x3D, 0x2, 0x40, 0x2, 0x42, 0x2, 0x42,
    0x2, 0x41, 0x2, 0x40, 0x4B, 0x2, 0x3C, 0x2, 0x3D, 0x5, 0x40, 0x5, 0x40, 0x2, 0x3D, 0x2, 0x40,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_11: [u8; 54] = [
    0x85, 0x2, 0x21, 0x5, 0x21, 0x2, 0x3D, 0x2, 0x3C, 0x2, 0x3D, 0x2, 0x3C, 0x2, 0x3D, 0x5, 0x3E,
    0x8, 0x3D, 0x5, 0x3B, 0x5, 0x21, 0x5, 0x21, 0x2, 0x39, 0x2, 0x38, 0x2, 0x39, 0x2, 0x38, 0x2,
    0x39, 0x5, 0x3B, 0x8, 0x39, 0x5, 0x39, 0x5, 0x23, 0x5, 0x23, 0x2, 0x38, 0x2, 0x37, 0x2, 0x38,
    0x2, 0x37, 0xFF,
];
#[allow(dead_code)]
pub static PATTERN_12: [u8; 26] = [
    0x85, 0x2, 0x21, 0x5, 0x21, 0x82, 0x9, 0x38, 0x82, 0x2, 0x24, 0x5, 0x25, 0x5, 0x23, 0x5, 0x21,
    0x85, 0x9, 0x38, 0x82, 0x2, 0x1C, 0x2, 0x1E, 0xFF,
];
#[allow(dead_code)]
pub static PATTERN_13: [u8; 109] = [
    0x2, 0x3C, 0x2, 0x3D, 0x5, 0x40, 0x5, 0x40, 0x2, 0x3D, 0x2, 0x40, 0x2, 0x42, 0x2, 0x42, 0x2,
    0x41, 0x5, 0x40, 0x2, 0x40, 0x2, 0x42, 0x2, 0x45, 0x2, 0x45, 0x2, 0x45, 0x2, 0x44, 0x8, 0x43,
    0x2, 0x42, 0x2, 0x42, 0x2, 0x41, 0x8, 0x40, 0x2, 0x3F, 0x2, 0x3F, 0x2, 0x3E, 0x8, 0x3D, 0x2,
    0x3C, 0x2, 0x3C, 0x2, 0x3B, 0x5, 0x39, 0x42, 0x81, 0xE, 0x34, 0x1, 0x33, 0x1, 0x32, 0x1, 0x31,
    0x1, 0x30, 0x1, 0x2F, 0x1, 0x2E, 0x1, 0x2D, 0x1, 0x2C, 0x1, 0x2B, 0x1, 0x2A, 0x1, 0x29, 0x1,
    0x28, 0x1, 0x29, 0x1, 0x2A, 0x1, 0x2B, 0x1, 0x2C, 0x1, 0x2D, 0x1, 0x2E, 0x1, 0x2F, 0x1, 0x30,
    0x1, 0x31, 0x1, 0x32, 0x1, 0x33, 0xFF,
];
#[allow(dead_code)]
pub static PATTERN_14: [u8; 85] = [
    0x82, 0xB, 0x33, 0x2, 0x34, 0x5, 0x37, 0x5, 0x37, 0x2, 0x34, 0x2, 0x37, 0x2, 0x39, 0x2, 0x39,
    0x2, 0x38, 0x2, 0x37, 0x4B, 0x2, 0x33, 0x2, 0x34, 0x5, 0x37, 0x5, 0x37, 0x2, 0x34, 0x2, 0x37,
    0x42, 0x8, 0x33, 0x2, 0x32, 0x5, 0x31, 0x42, 0x2, 0x33, 0x2, 0x34, 0x5, 0x37, 0x5, 0x37, 0x2,
    0x34, 0x2, 0x37, 0x2, 0x39, 0x2, 0x39, 0x2, 0x38, 0x2, 0x37, 0x4B, 0x2, 0x33, 0x2, 0x34, 0x5,
    0x37, 0x5, 0x37, 0x2, 0x34, 0x2, 0x37, 0x2, 0x3D, 0x2, 0x3D, 0x2, 0x39, 0xB, 0x2D, 0x42, 0xFF,
];
#[allow(dead_code)]
pub static PATTERN_15: [u8; 59] = [
    0x2, 0x33, 0x2, 0x34, 0x5, 0x37, 0x5, 0x37, 0x2, 0x34, 0x2, 0x37, 0x2, 0x39, 0x2, 0x39, 0x2,
    0x38, 0x5, 0x37, 0x2, 0x37, 0x2, 0x39, 0x2, 0x3D, 0x2, 0x3D, 0x2, 0x3D, 0x2, 0x3C, 0x8, 0x3B,
    0x2, 0x39, 0x2, 0x39, 0x2, 0x38, 0x8, 0x37, 0x2, 0x36, 0x2, 0x36, 0x2, 0x35, 0x8, 0x34, 0x2,
    0x33, 0x2, 0x33, 0x2, 0x32, 0x8, 0x31, 0xFF,
];
#[allow(dead_code)]
pub static PATTERN_16: [u8; 16] = [
    0x85, 0x2, 0x21, 0x5, 0x21, 0x82, 0x9, 0x38, 0x88, 0x2, 0x21, 0x37, 0x1C, 0x17, 0x1C, 0xFF,
];
#[allow(dead_code)]
pub static PATTERN_17: [u8; 15] = [
    0x85, 0x1, 0x25, 0x45, 0x8B, 0x3, 0x3A, 0x85, 0x1, 0x25, 0x45, 0x8B, 0x3, 0x3A, 0xFF,
];
#[allow(dead_code)]
pub static PATTERN_18: [u8; 126] = [
    0x85, 0x2, 0x1C, 0x5, 0x1C, 0x2, 0x1F, 0x2, 0x20, 0x5, 0x1C, 0x5, 0x1C, 0x5, 0x1A, 0x5, 0x19,
    0x5, 0x1C, 0x5, 0x1C, 0x5, 0x1C, 0x2, 0x1F, 0x2, 0x20, 0x2, 0x1A, 0x2, 0x1C, 0x4B, 0x8B, 0xD,
    0x17, 0x85, 0x2, 0x1C, 0x5, 0x1C, 0x2, 0x1F, 0x2, 0x20, 0x5, 0x1C, 0x5, 0x1C, 0x5, 0x1A, 0x5,
    0x19, 0x5, 0x1C, 0x5, 0x1C, 0x5, 0x1C, 0x2, 0x1F, 0x2, 0x20, 0x2, 0x1A, 0x2, 0x1C, 0x4B, 0x8B,
    0xD, 0x23, 0x85, 0x2, 0x1C, 0x5, 0x1C, 0x2, 0x1F, 0x2, 0x20, 0x5, 0x1C, 0x5, 0x1C, 0x5, 0x1A,
    0x5, 0x19, 0x5, 0x1C, 0x5, 0x1C, 0x5, 0x1C, 0x2, 0x1F, 0x2, 0x20, 0x2, 0x1A, 0x2, 0x1C, 0x2,
    0x1D, 0x5, 0x1D, 0x2, 0x1D, 0x2, 0x1E, 0x5, 0x1E, 0x2, 0x1E, 0x2, 0x1F, 0x5, 0x1F, 0x2, 0x1F,
    0x2, 0x20, 0x5, 0x20, 0x2, 0x20, 0xFF,
];
#[allow(dead_code)]
pub static PATTERN_19: [u8; 34] = [
    0x85, 0x1, 0x25, 0x45, 0x8B, 0x3, 0x3A, 0x82, 0x6, 0x33, 0x5, 0x33, 0x2, 0x33, 0x2, 0x30, 0x5,
    0x30, 0x2, 0x30, 0x82, 0x6, 0x2E, 0x5, 0x2E, 0x2, 0x2E, 0x2, 0x2A, 0x5, 0x2A, 0x2, 0x2A, 0xFF,
];
#[allow(dead_code)]
pub static PATTERN_20: [u8; 52] = [
    0x82, 0xE, 0x45, 0x2, 0x45, 0x5, 0x42, 0x2, 0x45, 0x2, 0x40, 0x2, 0x42, 0x2, 0x45, 0x2, 0x48,
    0x2, 0x45, 0x2, 0x47, 0x2, 0x42, 0x5, 0x45, 0x2, 0x40, 0x5, 0x43, 0x2, 0x40, 0x2, 0x42, 0x2,
    0x3D, 0x5, 0x40, 0x2, 0x39, 0x5, 0x3C, 0x2, 0x39, 0x2, 0x3B, 0x2, 0x36, 0x2, 0x39, 0x8, 0x45,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_21: [u8; 21] = [
    0x45, 0x85, 0x6, 0x33, 0x2, 0x30, 0x2, 0x33, 0x5, 0x30, 0x5, 0x33, 0x5, 0x30, 0x2, 0x30, 0x2,
    0x33, 0x5, 0x30, 0xFF,
];
#[allow(dead_code)]
pub static PATTERN_22: [u8; 10] = [0x85, 0x6, 0x28, 0x5, 0x28, 0x5, 0x28, 0x5, 0x28, 0xFF];
#[allow(dead_code)]
pub static PATTERN_23: [u8; 20] = [
    0x85, 0x5, 0x2D, 0x5, 0x58, 0x5, 0x58, 0x2, 0x4C, 0x5, 0x58, 0x2, 0x4C, 0x5, 0x58, 0x5, 0x4C,
    0x5, 0x58, 0xFF,
];
#[allow(dead_code)]
pub static PATTERN_24: [u8; 52] = [
    0x82, 0x0, 0x3D, 0x2, 0x3D, 0x5, 0x39, 0x2, 0x3D, 0x2, 0x37, 0x2, 0x39, 0x2, 0x3D, 0x2, 0x3F,
    0x2, 0x3D, 0x2, 0x3E, 0x2, 0x39, 0x5, 0x3D, 0x2, 0x37, 0x5, 0x3B, 0x2, 0x37, 0x2, 0x39, 0x2,
    0x34, 0x5, 0x37, 0x2, 0x31, 0x5, 0x33, 0x2, 0x31, 0x2, 0x32, 0x2, 0x2D, 0x2, 0x31, 0x8, 0x39,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_25: [u8; 2] = [0x57, 0xFF];
#[allow(dead_code)]
pub static PATTERN_26: [u8; 27] = [
    0x82, 0x6, 0x33, 0x5, 0x33, 0x2, 0x33, 0x2, 0x30, 0x5, 0x30, 0x2, 0x30, 0x82, 0x6, 0x2E, 0x5,
    0x2E, 0x2, 0x2E, 0x2, 0x2A, 0x5, 0x2A, 0x2, 0x2A, 0xFF,
];
#[allow(dead_code)]
pub static PATTERN_27: [u8; 34] = [
    0x82, 0x8, 0x50, 0x2, 0x60, 0x2, 0x40, 0x2, 0x50, 0x2, 0x60, 0x2, 0x40, 0x2, 0x50, 0x2, 0x60,
    0x2, 0x50, 0x2, 0x60, 0x2, 0x40, 0x2, 0x50, 0x2, 0x60, 0x2, 0x40, 0x2, 0x50, 0x2, 0x60, 0xFF,
];
#[allow(dead_code)]
pub static PATTERN_28: [u8; 34] = [
    0x82, 0xC, 0x64, 0x2, 0x58, 0x2, 0x40, 0x2, 0x64, 0x2, 0x58, 0x2, 0x40, 0x2, 0x64, 0x2, 0x58,
    0x2, 0x40, 0x2, 0x64, 0x2, 0x58, 0x2, 0x40, 0x2, 0x64, 0x2, 0x58, 0x2, 0x40, 0x2, 0x64, 0xFF,
];
#[allow(dead_code)]
pub static PATTERN_29: [u8; 18] = [
    0x82, 0x6, 0x28, 0x2, 0x28, 0x2, 0x28, 0x2, 0x28, 0x2, 0x28, 0x2, 0x28, 0x2, 0x28, 0x2, 0x28,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_30: [u8; 27] = [
    0x2, 0x3B, 0x5, 0x3D, 0x5, 0x3B, 0x2, 0x3A, 0x5, 0x39, 0x5, 0x38, 0x5, 0x38, 0x2, 0x39, 0x8,
    0x39, 0x5, 0x3A, 0x5, 0x3A, 0x2, 0x3B, 0x8, 0x3B, 0xFF,
];
#[allow(dead_code)]
pub static PATTERN_31: [u8; 35] = [
    0x2, 0x3B, 0x5, 0x3D, 0x5, 0x3B, 0x2, 0x3A, 0x5, 0x3B, 0x2, 0x3C, 0x5, 0x3C, 0x2, 0x3C, 0x2,
    0x3D, 0x5, 0x3D, 0x2, 0x3D, 0x2, 0x3E, 0x5, 0x3E, 0x2, 0x3E, 0x2, 0x3F, 0x5, 0x3F, 0x2, 0x3F,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_32: [u8; 9] = [0x42, 0x8, 0x3C, 0x2, 0x3B, 0x5, 0x39, 0x42, 0xFF];
#[allow(dead_code)]
pub static PATTERN_33: [u8; 10] = [0x2, 0x45, 0x2, 0x45, 0x2, 0x42, 0xB, 0x45, 0x42, 0xFF];
#[allow(dead_code)]
pub static PATTERN_34: [u8; 27] = [
    0x2, 0x38, 0x5, 0x39, 0x5, 0x38, 0x2, 0x37, 0x5, 0x36, 0x5, 0x28, 0x5, 0x28, 0x2, 0x2A, 0x8,
    0x2A, 0x5, 0x2B, 0x5, 0x2B, 0x2, 0x2C, 0x8, 0x2C, 0xFF,
];
#[allow(dead_code)]
pub static PATTERN_35: [u8; 35] = [
    0x2, 0x38, 0x5, 0x39, 0x5, 0x38, 0x2, 0x37, 0x5, 0x38, 0x2, 0x24, 0x5, 0x24, 0x2, 0x24, 0x2,
    0x25, 0x5, 0x25, 0x2, 0x25, 0x2, 0x26, 0x5, 0x26, 0x2, 0x26, 0x2, 0x27, 0x5, 0x27, 0x2, 0x27,
    0xFF,
];

#[allow(dead_code)]
pub static INSTRUMENTS: [Instrument; 45] = [
    Instrument {
        pulse_width: 3392,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x17,
        sustain_and_release: 0x65,
        vibrato_depth: 2,
        pulse_speed: 0x41,
        fx: 0b00000000,
    },
    Instrument {
        pulse_width: 2048,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x08,
        sustain_and_release: 0x08,
        vibrato_depth: 0,
        pulse_speed: 0x00,
        fx: 0b00000001,
    },
    Instrument {
        pulse_width: 2368,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x09,
        sustain_and_release: 0x39,
        vibrato_depth: 0,
        pulse_speed: 0x41,
        fx: 0b00000000,
    },
    Instrument {
        pulse_width: 512,
        ctrl_register: 0b10000001,
        attack_and_decay: 0x08,
        sustain_and_release: 0x0A,
        vibrato_depth: 0,
        pulse_speed: 0x00,
        fx: 0b00000001,
    },
    Instrument {
        pulse_width: 320,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x49,
        sustain_and_release: 0x87,
        vibrato_depth: 2,
        pulse_speed: 0x00,
        fx: 0b00000000,
    },
    Instrument {
        pulse_width: 2048,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x02,
        sustain_and_release: 0x00,
        vibrato_depth: 0,
        pulse_speed: 0x00,
        fx: 0b00000000,
    },
    Instrument {
        pulse_width: 2048,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x03,
        sustain_and_release: 0x0A,
        vibrato_depth: 0,
        pulse_speed: 0x00,
        fx: 0b00000001,
    },
    Instrument {
        pulse_width: 2176,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x56,
        sustain_and_release: 0x87,
        vibrato_depth: 3,
        pulse_speed: 0x00,
        fx: 0b00000000,
    },
    Instrument {
        pulse_width: 1664,
        ctrl_register: 0b10000001,
        attack_and_decay: 0x02,
        sustain_and_release: 0x09,
        vibrato_depth: 0,
        pulse_speed: 0x00,
        fx: 0b00000001,
    },
    Instrument {
        pulse_width: 512,
        ctrl_register: 0b10000001,
        attack_and_decay: 0x09,
        sustain_and_release: 0xA9,
        vibrato_depth: 0,
        pulse_speed: 0x00,
        fx: 0b00000001,
    },
    Instrument {
        pulse_width: 896,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x49,
        sustain_and_release: 0x39,
        vibrato_depth: 2,
        pulse_speed: 0x00,
        fx: 0b00000000,
    },
    Instrument {
        pulse_width: 1920,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x47,
        sustain_and_release: 0x29,
        vibrato_depth: 2,
        pulse_speed: 0x00,
        fx: 0b00000000,
    },
    Instrument {
        pulse_width: 1664,
        ctrl_register: 0b00010001,
        attack_and_decay: 0x06,
        sustain_and_release: 0x7B,
        vibrato_depth: 0,
        pulse_speed: 0x00,
        fx: 0b00000001,
    },
    Instrument {
        pulse_width: 2176,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x90,
        sustain_and_release: 0xF0,
        vibrato_depth: 1,
        pulse_speed: 0xE8,
        fx: 0b00000010,
    },
    Instrument {
        pulse_width: 3200,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x17,
        sustain_and_release: 0x65,
        vibrato_depth: 2,
        pulse_speed: 0x41,
        fx: 0b00000100,
    },
    Instrument {
        pulse_width: 8618,
        ctrl_register: 0b00000100,
        attack_and_decay: 0x80,
        sustain_and_release: 0x00,
        vibrato_depth: 65,
        pulse_speed: 0x0A,
        fx: 0b00000000,
    },
    Instrument {
        pulse_width: 3328,
        ctrl_register: 0b00000000,
        attack_and_decay: 0x02,
        sustain_and_release: 0x57,
        vibrato_depth: 12,
        pulse_speed: 0x00,
        fx: 0b00101000,
    },
    Instrument {
        pulse_width: 24595,
        ctrl_register: 0b00010000,
        attack_and_decay: 0x00,
        sustain_and_release: 0x04,
        vibrato_depth: 17,
        pulse_speed: 0x0F,
        fx: 0b00000000,
    },
    Instrument {
        pulse_width: 1800,
        ctrl_register: 0b10000000,
        attack_and_decay: 0x00,
        sustain_and_release: 0x11,
        vibrato_depth: 15,
        pulse_speed: 0x00,
        fx: 0b00010110,
    },
    Instrument {
        pulse_width: 4260,
        ctrl_register: 0b00100100,
        attack_and_decay: 0x00,
        sustain_and_release: 0x04,
        vibrato_depth: 0,
        pulse_speed: 0x0A,
        fx: 0b00000000,
    },
    Instrument {
        pulse_width: 2312,
        ctrl_register: 0b00000000,
        attack_and_decay: 0x02,
        sustain_and_release: 0x15,
        vibrato_depth: 11,
        pulse_speed: 0x00,
        fx: 0b00100000,
    },
    Instrument {
        pulse_width: 10787,
        ctrl_register: 0b11111001,
        attack_and_decay: 0x80,
        sustain_and_release: 0x00,
        vibrato_depth: 129,
        pulse_speed: 0x95,
        fx: 0b00000000,
    },
    Instrument {
        pulse_width: 40,
        ctrl_register: 0b00000000,
        attack_and_decay: 0x02,
        sustain_and_release: 0x81,
        vibrato_depth: 173,
        pulse_speed: 0x40,
        fx: 0b01011111,
    },
    Instrument {
        pulse_width: 16482,
        ctrl_register: 0b00000110,
        attack_and_decay: 0x80,
        sustain_and_release: 0x00,
        vibrato_depth: 65,
        pulse_speed: 0x0C,
        fx: 0b00000000,
    },
    Instrument {
        pulse_width: 5122,
        ctrl_register: 0b00010100,
        attack_and_decay: 0x02,
        sustain_and_release: 0x13,
        vibrato_depth: 13,
        pulse_speed: 0x00,
        fx: 0b01100000,
    },
    Instrument {
        pulse_width: 28817,
        ctrl_register: 0b00110011,
        attack_and_decay: 0x80,
        sustain_and_release: 0x02,
        vibrato_depth: 65,
        pulse_speed: 0x0B,
        fx: 0b00000000,
    },
    Instrument {
        pulse_width: 12800,
        ctrl_register: 0b10000000,
        attack_and_decay: 0x02,
        sustain_and_release: 0x41,
        vibrato_depth: 11,
        pulse_speed: 0x00,
        fx: 0b01000000,
    },
    Instrument {
        pulse_width: 12960,
        ctrl_register: 0b01010100,
        attack_and_decay: 0x00,
        sustain_and_release: 0x08,
        vibrato_depth: 17,
        pulse_speed: 0x04,
        fx: 0b11110000,
    },
    Instrument {
        pulse_width: 18882,
        ctrl_register: 0b00000000,
        attack_and_decay: 0x08,
        sustain_and_release: 0x11,
        vibrato_depth: 4,
        pulse_speed: 0xF0,
        fx: 0b01011001,
    },
    Instrument {
        pulse_width: 6816,
        ctrl_register: 0b00000110,
        attack_and_decay: 0x40,
        sustain_and_release: 0x00,
        vibrato_depth: 129,
        pulse_speed: 0x0A,
        fx: 0b00000000,
    },
    Instrument {
        pulse_width: 6144,
        ctrl_register: 0b00000000,
        attack_and_decay: 0x08,
        sustain_and_release: 0x81,
        vibrato_depth: 9,
        pulse_speed: 0x00,
        fx: 0b00110010,
    },
    Instrument {
        pulse_width: 8212,
        ctrl_register: 0b00000000,
        attack_and_decay: 0x00,
        sustain_and_release: 0x04,
        vibrato_depth: 81,
        pulse_speed: 0x0F,
        fx: 0b01110000,
    },
    Instrument {
        pulse_width: 9984,
        ctrl_register: 0b00000000,
        attack_and_decay: 0x08,
        sustain_and_release: 0x51,
        vibrato_depth: 15,
        pulse_speed: 0x70,
        fx: 0b00000000,
    },
    Instrument {
        pulse_width: 4262,
        ctrl_register: 0b00111101,
        attack_and_decay: 0x80,
        sustain_and_release: 0x08,
        vibrato_depth: 65,
        pulse_speed: 0x88,
        fx: 0b00010111,
    },
    Instrument {
        pulse_width: 14016,
        ctrl_register: 0b00000000,
        attack_and_decay: 0x08,
        sustain_and_release: 0x41,
        vibrato_depth: 136,
        pulse_speed: 0x17,
        fx: 0b00101111,
    },
    Instrument {
        pulse_width: 8224,
        ctrl_register: 0b00000000,
        attack_and_decay: 0x00,
        sustain_and_release: 0x00,
        vibrato_depth: 129,
        pulse_speed: 0x0B,
        fx: 0b00000000,
    },
    Instrument {
        pulse_width: 10047,
        ctrl_register: 0b01000000,
        attack_and_decay: 0x08,
        sustain_and_release: 0x13,
        vibrato_depth: 11,
        pulse_speed: 0x00,
        fx: 0b01011111,
    },
    Instrument {
        pulse_width: 16545,
        ctrl_register: 0b01000100,
        attack_and_decay: 0x00,
        sustain_and_release: 0x05,
        vibrato_depth: 65,
        pulse_speed: 0x05,
        fx: 0b11110000,
    },
    Instrument {
        pulse_width: 15418,
        ctrl_register: 0b00000000,
        attack_and_decay: 0x08,
        sustain_and_release: 0x41,
        vibrato_depth: 9,
        pulse_speed: 0x80,
        fx: 0b01000001,
    },
    Instrument {
        pulse_width: 6694,
        ctrl_register: 0b00110000,
        attack_and_decay: 0x00,
        sustain_and_release: 0x08,
        vibrato_depth: 67,
        pulse_speed: 0x03,
        fx: 0b11110000,
    },
    Instrument {
        pulse_width: 10177,
        ctrl_register: 0b01000000,
        attack_and_decay: 0x08,
        sustain_and_release: 0x15,
        vibrato_depth: 5,
        pulse_speed: 0xF0,
        fx: 0b01011111,
    },
    Instrument {
        pulse_width: 20130,
        ctrl_register: 0b00110100,
        attack_and_decay: 0x80,
        sustain_and_release: 0x04,
        vibrato_depth: 65,
        pulse_speed: 0x03,
        fx: 0b11110000,
    },
    Instrument {
        pulse_width: 15360,
        ctrl_register: 0b00000000,
        attack_and_decay: 0x03,
        sustain_and_release: 0x41,
        vibrato_depth: 3,
        pulse_speed: 0xF0,
        fx: 0b01010001,
    },
    Instrument {
        pulse_width: 1391,
        ctrl_register: 0b01000100,
        attack_and_decay: 0x00,
        sustain_and_release: 0x07,
        vibrato_depth: 17,
        pulse_speed: 0x0A,
        fx: 0b00000000,
    },
    Instrument {
        pulse_width: 7556,
        ctrl_register: 0b00000000,
        attack_and_decay: 0x02,
        sustain_and_release: 0x15,
        vibrato_depth: 12,
        pulse_speed: 0xD0,
        fx: 0b01001111,
    },
];

#[allow(dead_code)]
pub static SOUNDFX: [SoundFx; 1] = [SoundFx {
    incdec: 0b10010110,
    voice0: SidT {
        freq: 0x4451, // REAL: lower part is used as start note.
        pulse_width: 256,
        ctrl: 0b01000001,
        attack_and_decay_len: 0x0A,
        sustain_vol_and_release_len: 0x00,
    },
    voice1: SidT {
        freq: 0x1DC0, // REAL: value & 0b0011_1111 = note_delta ; sometime if value&0b1_0000000: flip_flop voice0 ctrl ; if value&0b0_1_000000: flip_flop voice1 ctrl
        pulse_width: 512,
        ctrl: 0b00010101,
        attack_and_decay_len: 0x0C,
        sustain_vol_and_release_len: 0x00,
    },
    sfx_note_dest: 0x38, // REAL: end note
}];

#[allow(dead_code)]
pub static INSTRFX: [InstrFx; 0] = [];
