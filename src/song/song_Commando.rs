// Commando - Rob Hubbard - 1985 Elite

use crate::rhplayer::rhsongs::{InstrFx, Instrument, RhSongs, SidT, SoundFx};
#[allow(dead_code)]
pub static RHSONGS: RhSongs = RhSongs {
    version: 10,
    total: 19,
    channels: &CHANNELS,
    tracks: &TRACKS,
    instruments: &INSTRUMENTS,
    soundfx: &SOUNDFX,
    instrfx: &INSTRFX,
    resetspd: 2,
    skydive_v1_when: 2,
    skydive_v1_add: 512,
};

#[allow(dead_code)]
pub static CHANNEL_0: [u8; 65] = [
    19, 19, 19, 19, 7, 7, 9, 12, 12, 16, 16, 16, 16, 15, 15, 17, 17, 18, 23, 23, 23, 23, 23, 23,
    23, 23, 16, 16, 16, 16, 23, 23, 23, 23, 16, 16, 23, 23, 26, 27, 28, 28, 28, 28, 29, 29, 29, 29,
    30, 30, 30, 30, 15, 23, 23, 31, 16, 16, 23, 17, 23, 18, 23, 31, 255,
];

#[allow(dead_code)]
pub static CHANNEL_1: [u8; 64] = [
    8, 8, 8, 10, 8, 10, 8, 8, 8, 19, 19, 20, 20, 20, 20, 21, 21, 22, 22, 24, 24, 24, 24, 24, 24,
    24, 24, 19, 19, 24, 24, 24, 24, 19, 24, 24, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 20, 20, 20,
    20, 20, 20, 24, 24, 31, 19, 24, 21, 24, 22, 22, 24, 31, 255,
];

#[allow(dead_code)]
pub static CHANNEL_2: [u8; 124] = [
    1, 1, 2, 3, 1, 1, 2, 3, 1, 1, 2, 3, 4, 4, 5, 6, 1, 1, 2, 3, 4, 4, 5, 6, 1, 11, 3, 1, 1, 1, 2,
    3, 1, 1, 2, 3, 1, 1, 11, 11, 13, 13, 14, 14, 13, 13, 14, 14, 11, 11, 11, 11, 3, 3, 3, 3, 25,
    25, 25, 25, 25, 25, 25, 25, 1, 1, 11, 11, 25, 25, 25, 25, 1, 11, 25, 25, 1, 1, 11, 11, 1, 1,
    11, 11, 1, 1, 11, 11, 1, 1, 11, 11, 1, 1, 11, 11, 13, 13, 14, 14, 13, 13, 14, 14, 13, 13, 14,
    14, 25, 25, 31, 1, 11, 25, 11, 11, 25, 3, 3, 3, 3, 25, 31, 255,
];

#[allow(dead_code)]
pub static CHANNEL_3: [u8; 10] = [36, 37, 36, 38, 39, 36, 38, 40, 40, 255];

#[allow(dead_code)]
pub static CHANNEL_4: [u8; 2] = [41, 255];

#[allow(dead_code)]
pub static CHANNEL_5: [u8; 10] = [32, 32, 34, 33, 32, 35, 35, 35, 35, 255];

#[allow(dead_code)]
pub static CHANNEL_6: [u8; 3] = [42, 0, 254];

#[allow(dead_code)]
pub static CHANNEL_7: [u8; 3] = [43, 0, 254];

#[allow(dead_code)]
pub static CHANNEL_8: [u8; 3] = [44, 0, 254];

#[allow(dead_code)]
pub static CHANNELS: [&[&[u8]; 3]; 3] = [
    &[&CHANNEL_0, &CHANNEL_1, &CHANNEL_2],
    &[&CHANNEL_3, &CHANNEL_4, &CHANNEL_5],
    &[&CHANNEL_6, &CHANNEL_7, &CHANNEL_8],
];

#[allow(dead_code)]
pub static TRACKS: [&[u8]; 45] = [
    &TRACK_0, &TRACK_1, &TRACK_2, &TRACK_3, &TRACK_4, &TRACK_5, &TRACK_6, &TRACK_7, &TRACK_8,
    &TRACK_9, &TRACK_10, &TRACK_11, &TRACK_12, &TRACK_13, &TRACK_14, &TRACK_15, &TRACK_16,
    &TRACK_17, &TRACK_18, &TRACK_19, &TRACK_20, &TRACK_21, &TRACK_22, &TRACK_23, &TRACK_24,
    &TRACK_25, &TRACK_26, &TRACK_27, &TRACK_28, &TRACK_29, &TRACK_30, &TRACK_31, &TRACK_32,
    &TRACK_33, &TRACK_34, &TRACK_35, &TRACK_36, &TRACK_37, &TRACK_38, &TRACK_39, &TRACK_40,
    &TRACK_41, &TRACK_42, &TRACK_43, &TRACK_44,
];
#[allow(dead_code)]
pub static TRACK_0: [u8; 2] = [0x5F, 0xFF];
#[allow(dead_code)]
pub static TRACK_1: [u8; 22] = [
    0x85, 0x2, 0x15, 0x1, 0x21, 0x83, 0x3, 0x2E, 0x83, 0x2, 0x15, 0x7, 0x15, 0x83, 0x3, 0x2E, 0x81,
    0x2, 0x1F, 0x1, 0x21, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_2: [u8; 22] = [
    0x85, 0x2, 0x16, 0x1, 0x22, 0x83, 0x3, 0x2E, 0x83, 0x2, 0x16, 0x7, 0x16, 0x83, 0x3, 0x2E, 0x81,
    0x2, 0x21, 0x1, 0x22, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_3: [u8; 22] = [
    0x85, 0x2, 0x10, 0x1, 0x1C, 0x83, 0x3, 0x2E, 0x83, 0x2, 0x10, 0x7, 0x10, 0x83, 0x3, 0x2E, 0x81,
    0x2, 0x1A, 0x1, 0x1C, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_4: [u8; 22] = [
    0x85, 0x2, 0x18, 0x1, 0x24, 0x83, 0x3, 0x2E, 0x83, 0x2, 0x18, 0x7, 0x18, 0x83, 0x3, 0x2E, 0x81,
    0x2, 0x22, 0x1, 0x24, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_5: [u8; 22] = [
    0x85, 0x2, 0x19, 0x1, 0x25, 0x83, 0x3, 0x2E, 0x83, 0x2, 0x19, 0x7, 0x19, 0x83, 0x3, 0x2E, 0x81,
    0x2, 0x24, 0x1, 0x25, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_6: [u8; 22] = [
    0x85, 0x2, 0x13, 0x1, 0x1F, 0x83, 0x3, 0x2E, 0x83, 0x2, 0x13, 0x7, 0x13, 0x83, 0x3, 0x2E, 0x81,
    0x2, 0x1C, 0x1, 0x1C, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_7: [u8; 113] = [
    0x81, 0x3, 0x32, 0x81, 0x0, 0x39, 0x3, 0x39, 0x3, 0x39, 0x3, 0x39, 0x7, 0x39, 0x5, 0x39, 0x3,
    0x39, 0x1, 0x40, 0x3, 0x40, 0x3, 0x40, 0x3, 0x40, 0x7, 0x40, 0x87, 0xC, 0x2C, 0x87, 0x0, 0x41,
    0x7, 0x40, 0x7, 0x41, 0x7, 0x40, 0x41, 0x1, 0x3B, 0x3, 0x3B, 0x3, 0x3B, 0x3, 0x3B, 0x7, 0x3B,
    0x87, 0xC, 0x2C, 0x81, 0x3, 0x32, 0x81, 0x0, 0x3C, 0x3, 0x3C, 0x3, 0x3C, 0x3, 0x3C, 0x7, 0x3C,
    0x5, 0x3C, 0x3, 0x3C, 0x1, 0x43, 0x3, 0x43, 0x3, 0x43, 0x3, 0x43, 0x7, 0x43, 0x87, 0xC, 0x2C,
    0x87, 0x0, 0x44, 0x7, 0x43, 0x7, 0x44, 0x7, 0x43, 0x41, 0x1, 0x3E, 0x3, 0x3E, 0x3, 0x3E, 0x3,
    0x3E, 0x7, 0x3E, 0x83, 0xC, 0x2F, 0x1, 0x2C, 0x1, 0x2C, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_8: [u8; 71] = [
    0x81, 0x4, 0x68, 0x1, 0x68, 0x1, 0x68, 0x1, 0x68, 0x83, 0x1, 0x34, 0x3, 0x34, 0x5, 0x35, 0x5,
    0x34, 0x3, 0x32, 0x81, 0x4, 0x68, 0x1, 0x68, 0x1, 0x68, 0x1, 0x68, 0x83, 0x1, 0x34, 0x3, 0x34,
    0x7, 0x34, 0x47, 0x81, 0x4, 0x68, 0x1, 0x68, 0x1, 0x68, 0x1, 0x68, 0x83, 0x1, 0x34, 0x3, 0x34,
    0x5, 0x35, 0x5, 0x34, 0x3, 0x32, 0x41, 0x81, 0x1, 0x34, 0x3, 0x34, 0x3, 0x34, 0x3, 0x34, 0x7,
    0x34, 0x47, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_9: [u8; 74] = [
    0x83, 0x3, 0x32, 0x3, 0x32, 0x83, 0x0, 0x39, 0x3, 0x39, 0x1, 0x39, 0x1, 0x39, 0x3, 0x39, 0x3,
    0x3B, 0x3, 0x3C, 0x1, 0x3E, 0x1, 0x3E, 0x3, 0x3E, 0x3, 0x3E, 0x3, 0x3E, 0x7, 0x3E, 0x83, 0xC,
    0x2C, 0x81, 0x0, 0x3E, 0x1, 0x40, 0x1, 0x41, 0x1, 0x41, 0x3, 0x40, 0x3, 0x3E, 0x3, 0x3C, 0x3,
    0x3B, 0x3, 0x39, 0x7, 0x38, 0x81, 0x3, 0x32, 0x81, 0x0, 0x39, 0x3, 0x39, 0x3, 0x39, 0x3, 0x3B,
    0x7, 0x39, 0x87, 0xC, 0x2C, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_10: [u8; 71] = [
    0x81, 0x4, 0x68, 0x1, 0x68, 0x1, 0x68, 0x1, 0x68, 0x83, 0x1, 0x37, 0x3, 0x37, 0x5, 0x38, 0x5,
    0x37, 0x3, 0x35, 0x81, 0x4, 0x68, 0x1, 0x68, 0x1, 0x68, 0x1, 0x68, 0x83, 0x1, 0x37, 0x3, 0x37,
    0x7, 0x37, 0x47, 0x81, 0x4, 0x68, 0x1, 0x68, 0x1, 0x68, 0x1, 0x68, 0x83, 0x1, 0x37, 0x3, 0x37,
    0x5, 0x38, 0x5, 0x37, 0x3, 0x35, 0x41, 0x81, 0x1, 0x37, 0x3, 0x37, 0x3, 0x37, 0x3, 0x37, 0x7,
    0x37, 0x47, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_11: [u8; 22] = [
    0x85, 0x2, 0x1A, 0x1, 0x26, 0x83, 0x3, 0x2E, 0x83, 0x2, 0x1A, 0x7, 0x1A, 0x83, 0x3, 0x2E, 0x81,
    0x2, 0x24, 0x1, 0x26, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_12: [u8; 84] = [
    0x81, 0x5, 0x3C, 0x3, 0x3B, 0x1, 0x3A, 0x3, 0x39, 0x1, 0x3C, 0x3, 0x3B, 0x1, 0x3A, 0x3, 0x39,
    0x1, 0x3C, 0x3, 0x3B, 0x1, 0x3A, 0x3, 0x39, 0x1, 0x3C, 0x3, 0x3B, 0x1, 0x3A, 0x3, 0x39, 0x1,
    0x3C, 0x3, 0x3B, 0x1, 0x39, 0x3, 0x41, 0x3, 0x40, 0x1, 0x41, 0x3, 0x40, 0x1, 0x3F, 0x3, 0x3E,
    0x1, 0x41, 0x3, 0x40, 0x1, 0x3F, 0x3, 0x3E, 0x3, 0x41, 0x3, 0x40, 0x1, 0x3B, 0x3, 0x3A, 0x1,
    0x39, 0x3, 0x38, 0x1, 0x3B, 0x3, 0x3A, 0x1, 0x39, 0x3, 0x38, 0x3, 0x3C, 0x3, 0x3B, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_13: [u8; 22] = [
    0x85, 0x2, 0x12, 0x1, 0x1E, 0x83, 0x3, 0x2E, 0x83, 0x2, 0x12, 0x7, 0x12, 0x83, 0x3, 0x2E, 0x81,
    0x2, 0x1C, 0x1, 0x1E, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_14: [u8; 22] = [
    0x85, 0x2, 0x19, 0x1, 0x25, 0x83, 0x3, 0x2E, 0x83, 0x2, 0x19, 0x7, 0x19, 0x83, 0x3, 0x2E, 0x81,
    0x2, 0x23, 0x1, 0x25, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_15: [u8; 55] = [
    0x8B, 0x6, 0x42, 0xA3, 0xCF, 0x42, 0x7, 0x40, 0x3, 0x3D, 0x3, 0x3B, 0xA3, 0xBF, 0x3B, 0x3,
    0x3A, 0x3, 0x3A, 0x83, 0xC, 0x2C, 0x3, 0x2C, 0x81, 0x6, 0x3A, 0x1, 0x3A, 0x3, 0x3B, 0x3, 0x3D,
    0xB, 0x3D, 0x3, 0x3D, 0x5, 0x40, 0x5, 0x3D, 0xA3, 0xA8, 0x3B, 0x7, 0x3D, 0x8F, 0xD1, 0x3D,
    0x83, 0xC, 0x2C, 0x3, 0x2C, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_16: [u8; 22] = [
    0x81, 0x6, 0x3C, 0x3, 0x3B, 0x1, 0x3A, 0x3, 0x39, 0x1, 0x3C, 0x3, 0x3B, 0x1, 0x3A, 0x3, 0x39,
    0x3, 0x3C, 0x3, 0x3E, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_17: [u8; 30] = [
    0x87, 0x6, 0x3E, 0x83, 0xC, 0x2C, 0x81, 0x6, 0x3E, 0x1, 0x3E, 0x5, 0x40, 0x5, 0x3E, 0xA3, 0xA8,
    0x3C, 0x7, 0x3E, 0x83, 0xC, 0x2F, 0xB, 0x2C, 0x3, 0x2F, 0x3, 0x2C, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_18: [u8; 60] = [
    0x87, 0x6, 0x40, 0x83, 0xC, 0x2C, 0x81, 0x6, 0x40, 0x1, 0x40, 0x5, 0x42, 0x5, 0x40, 0xA3, 0xA8,
    0x3E, 0x7, 0x40, 0x83, 0xC, 0x2F, 0xB, 0x2C, 0x3, 0x2F, 0x3, 0x2C, 0x87, 0x6, 0x40, 0x83, 0xC,
    0x2C, 0x81, 0x6, 0x40, 0x1, 0x40, 0x5, 0x42, 0x5, 0x40, 0xA3, 0xA8, 0x3E, 0x5, 0x40, 0x5, 0x42,
    0x3, 0x44, 0x5, 0x42, 0x5, 0x44, 0x3, 0x45, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_19: [u8; 36] = [
    0x83, 0x7, 0x58, 0x3, 0x51, 0x83, 0x1, 0x39, 0x3, 0x39, 0x5, 0x39, 0x5, 0x39, 0x5, 0x37, 0x1,
    0x39, 0x3, 0x39, 0x3, 0x39, 0x3, 0x37, 0x1, 0x39, 0x1, 0x37, 0x3, 0x39, 0x83, 0x7, 0x58, 0x3,
    0x51, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_20: [u8; 36] = [
    0x83, 0x7, 0x55, 0x3, 0x4E, 0x83, 0x1, 0x31, 0x3, 0x31, 0x5, 0x31, 0x5, 0x31, 0x5, 0x2F, 0x1,
    0x31, 0x3, 0x31, 0x3, 0x31, 0x3, 0x2F, 0x1, 0x31, 0x1, 0x2F, 0x3, 0x31, 0x83, 0x7, 0x55, 0x3,
    0x4E, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_21: [u8; 36] = [
    0x83, 0x7, 0x5D, 0x3, 0x56, 0x83, 0x1, 0x32, 0x3, 0x32, 0x5, 0x32, 0x5, 0x32, 0x5, 0x30, 0x1,
    0x32, 0x3, 0x32, 0x3, 0x32, 0x3, 0x30, 0x1, 0x32, 0x1, 0x30, 0x3, 0x32, 0x83, 0x7, 0x5D, 0x3,
    0x56, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_22: [u8; 36] = [
    0x83, 0x7, 0x5F, 0x3, 0x58, 0x83, 0x1, 0x34, 0x3, 0x34, 0x5, 0x34, 0x5, 0x34, 0x5, 0x32, 0x1,
    0x34, 0x3, 0x34, 0x3, 0x34, 0x3, 0x32, 0x1, 0x34, 0x1, 0x32, 0x3, 0x34, 0x83, 0x7, 0x5F, 0x3,
    0x58, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_23: [u8; 32] = [
    0x81, 0x5, 0x46, 0x1, 0x46, 0x1, 0x46, 0x1, 0x46, 0x1, 0x46, 0x1, 0x46, 0x1, 0x44, 0x1, 0x46,
    0x1, 0x46, 0x1, 0x44, 0x3, 0x46, 0x1, 0x46, 0x1, 0x46, 0x1, 0x44, 0x1, 0x44, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_24: [u8; 32] = [
    0x81, 0x5, 0x43, 0x1, 0x43, 0x1, 0x43, 0x1, 0x43, 0x1, 0x43, 0x1, 0x43, 0x1, 0x41, 0x1, 0x43,
    0x1, 0x43, 0x1, 0x41, 0x3, 0x43, 0x1, 0x43, 0x1, 0x43, 0x1, 0x41, 0x1, 0x41, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_25: [u8; 29] = [
    0x81, 0x5, 0x27, 0x1, 0x27, 0x1, 0x27, 0x1, 0x27, 0x83, 0xC, 0x2C, 0x81, 0x5, 0x25, 0x3, 0x27,
    0x1, 0x25, 0x1, 0x27, 0x1, 0x27, 0x83, 0xC, 0x2F, 0x3, 0x2C, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_26: [u8; 144] = [
    0xA7, 0x6, 0x37, 0xA7, 0xA8, 0x37, 0x17, 0x39, 0x3, 0x37, 0x3, 0x39, 0x3, 0x3E, 0x3, 0x3C, 0x7,
    0x39, 0x27, 0x3C, 0xA7, 0xAA, 0x3C, 0x17, 0x3E, 0x3, 0x3E, 0x3, 0x43, 0x3, 0x42, 0x3, 0x3E,
    0x7, 0x39, 0x27, 0x37, 0xA7, 0x90, 0x37, 0x17, 0x39, 0xA7, 0xA9, 0x3F, 0x3, 0x3E, 0x3, 0x3C,
    0x7, 0x39, 0x27, 0x3E, 0xA7, 0xA9, 0x3E, 0x17, 0x3C, 0x3, 0x3E, 0x3, 0x40, 0x3, 0x43, 0x3,
    0x42, 0x3, 0x43, 0x3, 0x45, 0x27, 0x43, 0xA7, 0xB4, 0x43, 0x7, 0x45, 0x1, 0x45, 0x3, 0x45, 0x1,
    0x45, 0x1, 0x45, 0x3, 0x45, 0x1, 0x43, 0x3, 0x45, 0x1, 0x43, 0x3, 0x42, 0x1, 0x43, 0x3, 0x42,
    0x3, 0x40, 0x3, 0x3E, 0x1, 0x3E, 0x3, 0x3E, 0x1, 0x3C, 0x3, 0x3E, 0x1, 0x3C, 0x3, 0x3B, 0x1,
    0x3C, 0x3, 0x3B, 0x3, 0x39, 0x3, 0x37, 0x1, 0x39, 0x3, 0x39, 0x1, 0x37, 0x3, 0x39, 0x1, 0x3B,
    0x3, 0x3C, 0x1, 0x3E, 0x3, 0x40, 0x3, 0x42, 0x3, 0x43, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_27: [u8; 46] = [
    0x27, 0x47, 0xA7, 0xB1, 0x47, 0x17, 0x45, 0x3, 0x43, 0x3, 0x45, 0x1, 0x48, 0x1, 0x48, 0x3,
    0x45, 0x1, 0x4A, 0x1, 0x4A, 0x3, 0x48, 0x27, 0x4C, 0xA7, 0xD1, 0x4C, 0x1F, 0x4A, 0x41, 0x1,
    0x4C, 0x1, 0x4C, 0x1, 0x40, 0x1, 0x4D, 0x1, 0x40, 0x1, 0x48, 0x1, 0x4A, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_28: [u8; 25] = [
    0x1, 0x4C, 0x1, 0x4C, 0x3, 0x40, 0x3, 0x4A, 0x1, 0x40, 0x3, 0x48, 0x1, 0x40, 0x3, 0x47, 0x1,
    0x48, 0x1, 0x40, 0x1, 0x48, 0x1, 0x4A, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_29: [u8; 25] = [
    0x1, 0x4C, 0x1, 0x4C, 0x3, 0x40, 0x3, 0x4B, 0x1, 0x40, 0x3, 0x49, 0x1, 0x40, 0x3, 0x47, 0x1,
    0x49, 0x1, 0x40, 0x1, 0x49, 0x1, 0x4B, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_30: [u8; 25] = [
    0x1, 0x49, 0x1, 0x49, 0x3, 0x3D, 0x3, 0x47, 0x1, 0x3D, 0x3, 0x46, 0x1, 0x3D, 0x3, 0x44, 0x1,
    0x42, 0x1, 0x3D, 0x1, 0x47, 0x1, 0x49, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_31: [u8; 10] = [0x87, 0x7, 0x68, 0x4F, 0x83, 0xC, 0x2C, 0x3, 0x2C, 0xFF];
#[allow(dead_code)]
pub static TRACK_32: [u8; 66] = [
    0x87, 0x2, 0x15, 0x83, 0x3, 0x2E, 0x83, 0x2, 0x15, 0x7, 0x15, 0x83, 0x3, 0x2E, 0x83, 0x2, 0x15,
    0x7, 0x13, 0x83, 0x3, 0x2E, 0x83, 0x2, 0x13, 0x7, 0x13, 0x83, 0x3, 0x2E, 0x83, 0x2, 0x13, 0x7,
    0x1A, 0x83, 0x3, 0x2E, 0x83, 0x2, 0x1A, 0x7, 0x1A, 0x83, 0x3, 0x2E, 0x83, 0x2, 0x1A, 0x7, 0x1C,
    0x83, 0x3, 0x2E, 0x83, 0x2, 0x1C, 0x7, 0x1C, 0x83, 0x3, 0x2E, 0x83, 0x2, 0x1C, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_33: [u8; 33] = [
    0x7, 0x1A, 0x83, 0x3, 0x2E, 0x83, 0x2, 0x1A, 0x7, 0x1A, 0x83, 0x3, 0x2E, 0x83, 0x2, 0x1A, 0x7,
    0x1C, 0x83, 0x3, 0x2E, 0x83, 0x2, 0x1C, 0x7, 0x1C, 0x83, 0x3, 0x2E, 0x83, 0x2, 0x1C, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_34: [u8; 33] = [
    0x7, 0x1E, 0x83, 0x3, 0x2E, 0x83, 0x2, 0x1E, 0x7, 0x1E, 0x83, 0x3, 0x2E, 0x83, 0x2, 0x1E, 0x7,
    0x19, 0x83, 0x3, 0x2E, 0x83, 0x2, 0x19, 0x7, 0x19, 0x83, 0x3, 0x2E, 0x83, 0x2, 0x19, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_35: [u8; 33] = [
    0x7, 0x15, 0x83, 0x3, 0x2E, 0x83, 0x2, 0x15, 0x7, 0x15, 0x83, 0x3, 0x2E, 0x83, 0x2, 0x15, 0x7,
    0x1A, 0x83, 0x3, 0x2E, 0x83, 0x2, 0x1A, 0x7, 0x1C, 0x83, 0x3, 0x2E, 0x83, 0x2, 0x1C, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_36: [u8; 18] = [
    0x97, 0x8, 0x3D, 0x7, 0x3B, 0x3, 0x3E, 0x7, 0x3D, 0x7, 0x3B, 0xB, 0x3D, 0x17, 0x39, 0x7, 0x39,
    0xFF,
];
#[allow(dead_code)]
pub static TRACK_37: [u8; 11] = [0x3, 0x39, 0x7, 0x38, 0xB, 0x36, 0x3, 0x38, 0x3, 0x39, 0xFF];
#[allow(dead_code)]
pub static TRACK_38: [u8; 9] = [0x3, 0x39, 0x7, 0x38, 0x7, 0x36, 0xB, 0x34, 0xFF];
#[allow(dead_code)]
pub static TRACK_39: [u8; 31] = [
    0x17, 0x34, 0x3, 0x36, 0x3, 0x39, 0x7, 0x39, 0xF, 0x38, 0x3, 0x39, 0x3, 0x3B, 0x7, 0x3D, 0xF,
    0x3B, 0x3, 0x3D, 0x3, 0x3E, 0x7, 0x3E, 0x7, 0x3D, 0x7, 0x3B, 0x7, 0x39, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_40: [u8; 35] = [
    0x1F, 0x39, 0x43, 0x3, 0x32, 0x3, 0x34, 0x3, 0x32, 0x3, 0x39, 0x3, 0x38, 0x3, 0x34, 0x3, 0x3B,
    0x1F, 0x39, 0x43, 0x3, 0x32, 0x3, 0x34, 0x3, 0x32, 0x3, 0x39, 0x3, 0x38, 0x3, 0x34, 0x3, 0x32,
    0xFF,
];
#[allow(dead_code)]
pub static TRACK_41: [u8; 18] = [
    0x83, 0x9, 0x2D, 0x3, 0x34, 0x3, 0x39, 0x3, 0x34, 0x3, 0x39, 0x3, 0x39, 0x3, 0x34, 0x3, 0x39,
    0xFF,
];
#[allow(dead_code)]
pub static TRACK_42: [u8; 41] = [
    0x81, 0xA, 0x34, 0x1, 0x34, 0x3, 0x34, 0x3, 0x34, 0x3, 0x34, 0x7, 0x37, 0x7, 0x39, 0x1, 0x34,
    0x1, 0x34, 0x3, 0x34, 0x3, 0x34, 0x3, 0x34, 0x7, 0x37, 0x7, 0x39, 0x41, 0x1, 0x3B, 0x3, 0x3B,
    0x3, 0x3B, 0x3, 0x3D, 0xF, 0x3B, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_43: [u8; 41] = [
    0x81, 0xA, 0x31, 0x1, 0x31, 0x3, 0x31, 0x3, 0x31, 0x3, 0x31, 0x7, 0x34, 0x7, 0x36, 0x1, 0x31,
    0x1, 0x31, 0x3, 0x31, 0x3, 0x31, 0x3, 0x31, 0x7, 0x34, 0x7, 0x36, 0x41, 0x1, 0x38, 0x3, 0x38,
    0x3, 0x38, 0x3, 0x3A, 0xF, 0x38, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_44: [u8; 41] = [
    0x81, 0xA, 0x2D, 0x1, 0x2D, 0x3, 0x2D, 0x3, 0x2D, 0x3, 0x2D, 0x7, 0x30, 0x7, 0x32, 0x1, 0x2D,
    0x1, 0x2D, 0x3, 0x2D, 0x3, 0x2D, 0x3, 0x2D, 0x7, 0x30, 0x7, 0x32, 0x41, 0x1, 0x34, 0x3, 0x34,
    0x3, 0x34, 0x3, 0x36, 0xF, 0x34, 0xFF,
];

#[allow(dead_code)]
pub static INSTRUMENTS: [Instrument; 13] = [
    Instrument {
        pulse_width: 2304,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x29,
        sustain_and_release: 0x5F,
        vibrato_depth: 2,
        pulse_speed: 0xE0,
        fx: 0b00000000,
    },
    Instrument {
        pulse_width: 384,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x06,
        sustain_and_release: 0x4B,
        vibrato_depth: 0,
        pulse_speed: 0x00,
        fx: 0b00000101,
    },
    Instrument {
        pulse_width: 384,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x09,
        sustain_and_release: 0x9F,
        vibrato_depth: 0,
        pulse_speed: 0x16,
        fx: 0b00001000,
    },
    Instrument {
        pulse_width: 512,
        ctrl_register: 0b10000001,
        attack_and_decay: 0x0A,
        sustain_and_release: 0x09,
        vibrato_depth: 0,
        pulse_speed: 0x00,
        fx: 0b00000101,
    },
    Instrument {
        pulse_width: 512,
        ctrl_register: 0b01000011,
        attack_and_decay: 0x0F,
        sustain_and_release: 0xC4,
        vibrato_depth: 0,
        pulse_speed: 0x00,
        fx: 0b00000011,
    },
    Instrument {
        pulse_width: 2176,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x05,
        sustain_and_release: 0xA9,
        vibrato_depth: 0,
        pulse_speed: 0x02,
        fx: 0b00001101,
    },
    Instrument {
        pulse_width: 2048,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x38,
        sustain_and_release: 0x7A,
        vibrato_depth: 2,
        pulse_speed: 0xE0,
        fx: 0b00000000,
    },
    Instrument {
        pulse_width: 384,
        ctrl_register: 0b00010101,
        attack_and_decay: 0x0D,
        sustain_and_release: 0xFB,
        vibrato_depth: 1,
        pulse_speed: 0x00,
        fx: 0b00000101,
    },
    Instrument {
        pulse_width: 2048,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x49,
        sustain_and_release: 0x5B,
        vibrato_depth: 2,
        pulse_speed: 0x03,
        fx: 0b00001000,
    },
    Instrument {
        pulse_width: 2048,
        ctrl_register: 0b00100001,
        attack_and_decay: 0x04,
        sustain_and_release: 0x6F,
        vibrato_depth: 3,
        pulse_speed: 0x00,
        fx: 0b00000101,
    },
    Instrument {
        pulse_width: 768,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x09,
        sustain_and_release: 0x6B,
        vibrato_depth: 2,
        pulse_speed: 0x01,
        fx: 0b00001101,
    },
    Instrument {
        pulse_width: 512,
        ctrl_register: 0b01000011,
        attack_and_decay: 0x07,
        sustain_and_release: 0x09,
        vibrato_depth: 1,
        pulse_speed: 0x00,
        fx: 0b00000001,
    },
    Instrument {
        pulse_width: 2048,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x09,
        sustain_and_release: 0x0A,
        vibrato_depth: 0,
        pulse_speed: 0x00,
        fx: 0b00000001,
    },
];

#[allow(dead_code)]
pub static SOUNDFX: [SoundFx; 16] = [
    SoundFx {
        incdec: 0b00010000,
        voice0: SidT {
            freq: 0x2450, // REAL: lower part is used as start note.
            pulse_width: 4480,
            ctrl: 0b00010001,
            attack_and_decay_len: 0x7C,
            sustain_vol_and_release_len: 0x30,
        },
        voice1: SidT {
            freq: 0x0011, // REAL: value & 0b0011_1111 = note_delta ; sometime if value&0b1_0000000: flip_flop voice0 ctrl ; if value&0b0_1_000000: flip_flop voice1 ctrl
            pulse_width: 2176,
            ctrl: 0b00010101,
            attack_and_decay_len: 0x4C,
            sustain_vol_and_release_len: 0x00,
        },
        sfx_note_dest: 0x18, // REAL: end note
    },
    SoundFx {
        incdec: 0b01100000,
        voice0: SidT {
            freq: 0x5838, // REAL: lower part is used as start note.
            pulse_width: 4480,
            ctrl: 0b10000001,
            attack_and_decay_len: 0x0A,
            sustain_vol_and_release_len: 0x30,
        },
        voice1: SidT {
            freq: 0x0000, // REAL: value & 0b0011_1111 = note_delta ; sometime if value&0b1_0000000: flip_flop voice0 ctrl ; if value&0b0_1_000000: flip_flop voice1 ctrl
            pulse_width: 2176,
            ctrl: 0b00010101,
            attack_and_decay_len: 0x0A,
            sustain_vol_and_release_len: 0x90,
        },
        sfx_note_dest: 0x58, // REAL: end note
    },
    SoundFx {
        incdec: 0b01010001,
        voice0: SidT {
            freq: 0x0F3F, // REAL: lower part is used as start note.
            pulse_width: 4480,
            ctrl: 0b10000001,
            attack_and_decay_len: 0x0A,
            sustain_vol_and_release_len: 0x10,
        },
        voice1: SidT {
            freq: 0x0020, // REAL: value & 0b0011_1111 = note_delta ; sometime if value&0b1_0000000: flip_flop voice0 ctrl ; if value&0b0_1_000000: flip_flop voice1 ctrl
            pulse_width: 2176,
            ctrl: 0b10000001,
            attack_and_decay_len: 0x0A,
            sustain_vol_and_release_len: 0x10,
        },
        sfx_note_dest: 0x27, // REAL: end note
    },
    SoundFx {
        incdec: 0b00010001,
        voice0: SidT {
            freq: 0x506F, // REAL: lower part is used as start note.
            pulse_width: 2048,
            ctrl: 0b01000001,
            attack_and_decay_len: 0x0D,
            sustain_vol_and_release_len: 0x40,
        },
        voice1: SidT {
            freq: 0x0F03, // REAL: value & 0b0011_1111 = note_delta ; sometime if value&0b1_0000000: flip_flop voice0 ctrl ; if value&0b0_1_000000: flip_flop voice1 ctrl
            pulse_width: 2176,
            ctrl: 0b00010101,
            attack_and_decay_len: 0x0D,
            sustain_vol_and_release_len: 0x40,
        },
        sfx_note_dest: 0x00, // REAL: end note
    },
    SoundFx {
        incdec: 0b00010001,
        voice0: SidT {
            freq: 0x005F, // REAL: lower part is used as start note.
            pulse_width: 2176,
            ctrl: 0b01000001,
            attack_and_decay_len: 0x00,
            sustain_vol_and_release_len: 0xC0,
        },
        voice1: SidT {
            freq: 0x00C4, // REAL: value & 0b0011_1111 = note_delta ; sometime if value&0b1_0000000: flip_flop voice0 ctrl ; if value&0b0_1_000000: flip_flop voice1 ctrl
            pulse_width: 2176,
            ctrl: 0b01000001,
            attack_and_decay_len: 0x00,
            sustain_vol_and_release_len: 0xC0,
        },
        sfx_note_dest: 0x30, // REAL: end note
    },
    SoundFx {
        incdec: 0b10100011,
        voice0: SidT {
            freq: 0x0133, // REAL: lower part is used as start note.
            pulse_width: 0,
            ctrl: 0b01010001,
            attack_and_decay_len: 0x0F,
            sustain_vol_and_release_len: 0xF0,
        },
        voice1: SidT {
            freq: 0x0200, // REAL: value & 0b0011_1111 = note_delta ; sometime if value&0b1_0000000: flip_flop voice0 ctrl ; if value&0b0_1_000000: flip_flop voice1 ctrl
            pulse_width: 0,
            ctrl: 0b01010111,
            attack_and_decay_len: 0x0F,
            sustain_vol_and_release_len: 0xF0,
        },
        sfx_note_dest: 0x5F, // REAL: end note
    },
    SoundFx {
        incdec: 0b01100110,
        voice0: SidT {
            freq: 0x0200, // REAL: lower part is used as start note.
            pulse_width: 0,
            ctrl: 0b01010001,
            attack_and_decay_len: 0x0F,
            sustain_vol_and_release_len: 0xF0,
        },
        voice1: SidT {
            freq: 0x0300, // REAL: value & 0b0011_1111 = note_delta ; sometime if value&0b1_0000000: flip_flop voice0 ctrl ; if value&0b0_1_000000: flip_flop voice1 ctrl
            pulse_width: 0,
            ctrl: 0b01010111,
            attack_and_decay_len: 0x0F,
            sustain_vol_and_release_len: 0xF0,
        },
        sfx_note_dest: 0x28, // REAL: end note
    },
    SoundFx {
        incdec: 0b01100001,
        voice0: SidT {
            freq: 0x0700, // REAL: lower part is used as start note.
            pulse_width: 640,
            ctrl: 0b01000001,
            attack_and_decay_len: 0x00,
            sustain_vol_and_release_len: 0xFF,
        },
        voice1: SidT {
            freq: 0x2700, // REAL: value & 0b0011_1111 = note_delta ; sometime if value&0b1_0000000: flip_flop voice0 ctrl ; if value&0b0_1_000000: flip_flop voice1 ctrl
            pulse_width: 1024,
            ctrl: 0b01000011,
            attack_and_decay_len: 0x00,
            sustain_vol_and_release_len: 0xFF,
        },
        sfx_note_dest: 0x4F, // REAL: end note
    },
    SoundFx {
        incdec: 0b00010010,
        voice0: SidT {
            freq: 0x0830, // REAL: lower part is used as start note.
            pulse_width: 256,
            ctrl: 0b10000001,
            attack_and_decay_len: 0x0C,
            sustain_vol_and_release_len: 0x40,
        },
        voice1: SidT {
            freq: 0x0404, // REAL: value & 0b0011_1111 = note_delta ; sometime if value&0b1_0000000: flip_flop voice0 ctrl ; if value&0b0_1_000000: flip_flop voice1 ctrl
            pulse_width: 2048,
            ctrl: 0b00010101,
            attack_and_decay_len: 0x0C,
            sustain_vol_and_release_len: 0x00,
        },
        sfx_note_dest: 0x10, // REAL: end note
    },
    SoundFx {
        incdec: 0b01100001,
        voice0: SidT {
            freq: 0x1800, // REAL: lower part is used as start note.
            pulse_width: 2048,
            ctrl: 0b10000001,
            attack_and_decay_len: 0x07,
            sustain_vol_and_release_len: 0x00,
        },
        voice1: SidT {
            freq: 0x01C5, // REAL: value & 0b0011_1111 = note_delta ; sometime if value&0b1_0000000: flip_flop voice0 ctrl ; if value&0b0_1_000000: flip_flop voice1 ctrl
            pulse_width: 512,
            ctrl: 0b10000101,
            attack_and_decay_len: 0x06,
            sustain_vol_and_release_len: 0x00,
        },
        sfx_note_dest: 0x48, // REAL: end note
    },
    SoundFx {
        incdec: 0b01010000,
        voice0: SidT {
            freq: 0x1430, // REAL: lower part is used as start note.
            pulse_width: 2048,
            ctrl: 0b10000001,
            attack_and_decay_len: 0x0C,
            sustain_vol_and_release_len: 0x00,
        },
        voice1: SidT {
            freq: 0x0125, // REAL: value & 0b0011_1111 = note_delta ; sometime if value&0b1_0000000: flip_flop voice0 ctrl ; if value&0b0_1_000000: flip_flop voice1 ctrl
            pulse_width: 256,
            ctrl: 0b01000011,
            attack_and_decay_len: 0x0A,
            sustain_vol_and_release_len: 0x00,
        },
        sfx_note_dest: 0x08, // REAL: end note
    },
    SoundFx {
        incdec: 0b01010000,
        voice0: SidT {
            freq: 0x205F, // REAL: lower part is used as start note.
            pulse_width: 2176,
            ctrl: 0b10000001,
            attack_and_decay_len: 0x0B,
            sustain_vol_and_release_len: 0x00,
        },
        voice1: SidT {
            freq: 0x2700, // REAL: value & 0b0011_1111 = note_delta ; sometime if value&0b1_0000000: flip_flop voice0 ctrl ; if value&0b0_1_000000: flip_flop voice1 ctrl
            pulse_width: 512,
            ctrl: 0b00010101,
            attack_and_decay_len: 0x0A,
            sustain_vol_and_release_len: 0x00,
        },
        sfx_note_dest: 0x27, // REAL: end note
    },
    SoundFx {
        incdec: 0b00100010,
        voice0: SidT {
            freq: 0x1426, // REAL: lower part is used as start note.
            pulse_width: 896,
            ctrl: 0b01000001,
            attack_and_decay_len: 0x0D,
            sustain_vol_and_release_len: 0x00,
        },
        voice1: SidT {
            freq: 0x1006, // REAL: value & 0b0011_1111 = note_delta ; sometime if value&0b1_0000000: flip_flop voice0 ctrl ; if value&0b0_1_000000: flip_flop voice1 ctrl
            pulse_width: 1536,
            ctrl: 0b01000001,
            attack_and_decay_len: 0x0D,
            sustain_vol_and_release_len: 0x00,
        },
        sfx_note_dest: 0x53, // REAL: end note
    },
    SoundFx {
        incdec: 0b10100010,
        voice0: SidT {
            freq: 0x1800, // REAL: lower part is used as start note.
            pulse_width: 2176,
            ctrl: 0b01000001,
            attack_and_decay_len: 0x09,
            sustain_vol_and_release_len: 0x20,
        },
        voice1: SidT {
            freq: 0x37CE, // REAL: value & 0b0011_1111 = note_delta ; sometime if value&0b1_0000000: flip_flop voice0 ctrl ; if value&0b0_1_000000: flip_flop voice1 ctrl
            pulse_width: 512,
            ctrl: 0b00010101,
            attack_and_decay_len: 0x09,
            sustain_vol_and_release_len: 0x10,
        },
        sfx_note_dest: 0x26, // REAL: end note
    },
    SoundFx {
        incdec: 0b10100010,
        voice0: SidT {
            freq: 0x0A36, // REAL: lower part is used as start note.
            pulse_width: 2176,
            ctrl: 0b10000001,
            attack_and_decay_len: 0x0A,
            sustain_vol_and_release_len: 0x00,
        },
        voice1: SidT {
            freq: 0x1700, // REAL: value & 0b0011_1111 = note_delta ; sometime if value&0b1_0000000: flip_flop voice0 ctrl ; if value&0b0_1_000000: flip_flop voice1 ctrl
            pulse_width: 672,
            ctrl: 0b10000001,
            attack_and_decay_len: 0x0A,
            sustain_vol_and_release_len: 0x00,
        },
        sfx_note_dest: 0x4F, // REAL: end note
    },
    SoundFx {
        incdec: 0b01010000,
        voice0: SidT {
            freq: 0x0930, // REAL: lower part is used as start note.
            pulse_width: 2176,
            ctrl: 0b10000001,
            attack_and_decay_len: 0x09,
            sustain_vol_and_release_len: 0x00,
        },
        voice1: SidT {
            freq: 0x0200, // REAL: value & 0b0011_1111 = note_delta ; sometime if value&0b1_0000000: flip_flop voice0 ctrl ; if value&0b0_1_000000: flip_flop voice1 ctrl
            pulse_width: 512,
            ctrl: 0b10000001,
            attack_and_decay_len: 0x09,
            sustain_vol_and_release_len: 0x00,
        },
        sfx_note_dest: 0x07, // REAL: end note
    },
];

#[allow(dead_code)]
pub static INSTRFX: [InstrFx; 0] = [];
