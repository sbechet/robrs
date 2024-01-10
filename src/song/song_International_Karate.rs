// International Karate - Rob Hubbard - 1986 System 3

use crate::rhplayer::rhsongs::{InstrFx, Instrument, RhSongs, SidT, SoundFx};
#[allow(dead_code)]
pub static RHSONGS: RhSongs = RhSongs {
    version: 20,
    total: 1,
    channels: &CHANNELS,
    tracks: &TRACKS,
    instruments: &INSTRUMENTS,
    soundfx: &SOUNDFX,
    instrfx: &INSTRFX,
    resetspd: 2,
    skydive_v1_when: 0,
    skydive_v1_add: 0,
};

#[allow(dead_code)]
pub static CHANNEL_0: [u8; 137] = [
    0, 0, 8, 8, 2, 3, 2, 3, 9, 9, 11, 11, 14, 15, 15, 11, 9, 9, 2, 3, 2, 3, 17, 18, 17, 17, 0, 0,
    20, 20, 22, 22, 20, 20, 22, 22, 25, 25, 26, 26, 27, 27, 28, 28, 28, 28, 30, 30, 31, 31, 30, 30,
    32, 32, 33, 34, 33, 34, 35, 36, 36, 37, 37, 38, 39, 39, 39, 39, 39, 39, 39, 39, 40, 41, 38, 40,
    42, 43, 43, 43, 43, 44, 44, 44, 44, 45, 45, 45, 45, 46, 35, 0, 0, 25, 26, 27, 28, 28, 30, 30,
    31, 31, 30, 30, 32, 32, 33, 34, 33, 34, 20, 20, 22, 22, 22, 47, 47, 47, 47, 0, 0, 17, 18, 17,
    18, 2, 3, 2, 3, 2, 52, 0, 0, 0, 0, 0, 255,
];

#[allow(dead_code)]
pub static CHANNEL_1: [u8; 225] = [
    49, 49, 49, 49, 49, 49, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 10, 10, 10, 10, 10, 10, 10, 10, 16,
    16, 16, 16, 16, 16, 16, 16, 16, 16, 10, 10, 10, 10, 10, 10, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29,
    29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29,
    29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29,
    29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29,
    29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29,
    29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 0, 49, 49,
    49, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 49, 49, 49, 49, 0, 0, 51, 0, 0, 0, 0, 0, 255,
];

#[allow(dead_code)]
pub static CHANNEL_2: [u8; 251] = [
    0, 0, 8, 12, 12, 7, 7, 12, 12, 7, 7, 1, 5, 6, 6, 1, 5, 6, 6, 1, 7, 12, 12, 13, 5, 6, 6, 1, 1,
    13, 13, 1, 1, 13, 13, 5, 5, 6, 6, 5, 5, 6, 6, 5, 5, 6, 6, 5, 5, 6, 6, 5, 5, 6, 6, 5, 5, 6, 6,
    5, 5, 6, 6, 5, 5, 6, 6, 1, 1, 13, 13, 1, 1, 13, 13, 1, 5, 6, 6, 1, 5, 6, 6, 1, 7, 12, 12, 13,
    5, 6, 6, 1, 5, 6, 6, 1, 5, 6, 6, 1, 7, 12, 12, 13, 5, 6, 6, 0, 19, 19, 19, 21, 21, 21, 21, 19,
    19, 21, 21, 21, 21, 19, 19, 23, 23, 23, 23, 24, 24, 24, 24, 21, 21, 21, 21, 24, 24, 23, 23, 24,
    24, 21, 21, 19, 21, 21, 19, 21, 21, 19, 21, 21, 19, 21, 21, 19, 21, 21, 19, 21, 21, 19, 21, 21,
    19, 21, 21, 19, 21, 21, 19, 21, 21, 19, 21, 21, 19, 21, 21, 19, 21, 21, 19, 21, 21, 0, 0, 19,
    23, 23, 24, 24, 21, 21, 24, 24, 23, 23, 24, 24, 21, 21, 19, 21, 21, 19, 21, 21, 19, 19, 21, 21,
    21, 21, 0, 0, 0, 0, 48, 1, 5, 6, 6, 1, 5, 6, 6, 1, 7, 12, 12, 13, 5, 6, 6, 1, 5, 6, 6, 1, 5, 6,
    6, 8, 50, 0, 0, 0, 0, 0, 255,
];

#[allow(dead_code)]
pub static CHANNELS: [&[&[u8]; 3]; 1] = [&[&CHANNEL_0, &CHANNEL_1, &CHANNEL_2]];

#[allow(dead_code)]
pub static TRACKS: [&[u8]; 53] = [
    &TRACK_0, &TRACK_1, &TRACK_2, &TRACK_3, &TRACK_4, &TRACK_5, &TRACK_6, &TRACK_7, &TRACK_8,
    &TRACK_9, &TRACK_10, &TRACK_11, &TRACK_12, &TRACK_13, &TRACK_14, &TRACK_15, &TRACK_16,
    &TRACK_17, &TRACK_18, &TRACK_19, &TRACK_20, &TRACK_21, &TRACK_22, &TRACK_23, &TRACK_24,
    &TRACK_25, &TRACK_26, &TRACK_27, &TRACK_28, &TRACK_29, &TRACK_30, &TRACK_31, &TRACK_32,
    &TRACK_33, &TRACK_34, &TRACK_35, &TRACK_36, &TRACK_37, &TRACK_38, &TRACK_39, &TRACK_40,
    &TRACK_41, &TRACK_42, &TRACK_43, &TRACK_44, &TRACK_45, &TRACK_46, &TRACK_47, &TRACK_48,
    &TRACK_49, &TRACK_50, &TRACK_51, &TRACK_52,
];
#[allow(dead_code)]
pub static TRACK_0: [u8; 3] = [0x5F, 0x5F, 0xFF];
#[allow(dead_code)]
pub static TRACK_1: [u8; 14] = [
    0x83, 0x2, 0x12, 0x7, 0x12, 0x3, 0x12, 0x87, 0x3, 0x3E, 0x87, 0x2, 0x12, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_2: [u8; 36] = [
    0x83, 0x0, 0x3F, 0x3, 0x41, 0x3, 0x3F, 0x3, 0x41, 0xF, 0x3A, 0x47, 0x3, 0x3F, 0x3, 0x44, 0x3,
    0x41, 0x3, 0x3F, 0x7, 0x41, 0x3, 0x3F, 0x3, 0x41, 0x3, 0x3F, 0x3, 0x3D, 0x7, 0x3F, 0x7, 0x3A,
    0x5F, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_3: [u8; 38] = [
    0x83, 0x0, 0x3F, 0x3, 0x41, 0x3, 0x3F, 0x3, 0x41, 0xF, 0x3A, 0x47, 0x3, 0x3F, 0x3, 0x44, 0x3,
    0x46, 0x3, 0x44, 0x3, 0x41, 0x3, 0x3F, 0x3, 0x3F, 0x3, 0x3D, 0x3, 0x3F, 0x3, 0x3D, 0x7, 0x38,
    0x7, 0x3A, 0x5F, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_4: [u8; 34] = [
    0x83, 0x1, 0x49, 0x7, 0x46, 0x3, 0x46, 0x83, 0x4, 0x52, 0x3, 0x46, 0x83, 0x1, 0x46, 0x3, 0x49,
    0x3, 0x4B, 0x3, 0x4B, 0x7, 0x49, 0x83, 0x4, 0x46, 0x3, 0x52, 0x83, 0x5, 0x52, 0x3, 0x46, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_5: [u8; 16] = [
    0x83, 0x2, 0x14, 0x7, 0x14, 0x3, 0x14, 0x87, 0x3, 0x3E, 0x83, 0x2, 0x14, 0x3, 0x20, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_6: [u8; 14] = [
    0x83, 0x2, 0x16, 0x7, 0x16, 0x3, 0x16, 0x87, 0x3, 0x3E, 0x87, 0x2, 0x16, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_7: [u8; 16] = [
    0x83, 0x2, 0x1D, 0x7, 0x1D, 0x3, 0x1D, 0x87, 0x3, 0x3E, 0x83, 0x2, 0x1D, 0x3, 0x1D, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_8: [u8; 35] = [
    0x97, 0x6, 0x33, 0x23, 0x35, 0xA3, 0xFD, 0x0, 0x35, 0x3F, 0x2E, 0xF, 0x2E, 0x23, 0x35, 0xA3,
    0xF4, 0x0, 0x35, 0x23, 0x38, 0xA3, 0xFD, 0x0, 0x38, 0xF, 0x33, 0xF, 0x35, 0x17, 0x33, 0x25,
    0x31, 0xA1, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_9: [u8; 66] = [
    0x83, 0x7, 0x52, 0x3, 0x52, 0x3, 0x52, 0x3, 0x50, 0x3, 0x50, 0x3, 0x50, 0x3, 0x4D, 0x3, 0x50,
    0x3, 0x4D, 0x3, 0x4D, 0x3, 0x4D, 0x3, 0x4B, 0x3, 0x50, 0x3, 0x50, 0x3, 0x4D, 0x3, 0x50, 0x3,
    0x52, 0x3, 0x52, 0x3, 0x52, 0x3, 0x50, 0x3, 0x50, 0x3, 0x50, 0x3, 0x4D, 0x3, 0x50, 0x3, 0x4D,
    0x3, 0x4D, 0x3, 0x4D, 0x3, 0x4B, 0x3, 0x4B, 0x3, 0x49, 0x3, 0x49, 0x3, 0x46, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_10: [u8; 42] = [
    0x83, 0x1, 0x55, 0x3, 0x57, 0x3, 0x55, 0x3, 0x57, 0x83, 0x4, 0x52, 0x83, 0x1, 0x52, 0x83, 0x4,
    0x55, 0x83, 0x1, 0x55, 0x3, 0x57, 0x3, 0x55, 0x3, 0x57, 0x3, 0x55, 0x83, 0x4, 0x52, 0x83, 0x5,
    0x49, 0x83, 0x4, 0x55, 0x83, 0x5, 0x3D, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_11: [u8; 42] = [
    0xA1, 0x8, 0x2C, 0xA1, 0xFE, 0x0, 0x2C, 0x13, 0x2E, 0x24, 0x33, 0xA2, 0xE0, 0x0, 0x33, 0xF,
    0x35, 0xF, 0x2C, 0x21, 0x2C, 0xA1, 0xFE, 0x0, 0x2C, 0x13, 0x2E, 0x23, 0x38, 0xA3, 0xE0, 0x0,
    0x38, 0x2C, 0x3A, 0xA2, 0x81, 0x1, 0x3A, 0xF, 0x35, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_12: [u8; 16] = [
    0x83, 0x2, 0x1B, 0x7, 0x1B, 0x3, 0x1B, 0x87, 0x3, 0x3E, 0x83, 0x2, 0x1B, 0x3, 0x27, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_13: [u8; 14] = [
    0x83, 0x2, 0x19, 0x7, 0x19, 0x3, 0x19, 0x87, 0x3, 0x3E, 0x87, 0x2, 0x19, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_14: [u8; 13] = [
    0x92, 0x8, 0x2E, 0xA4, 0xA0, 0x1, 0x2E, 0x7, 0x3A, 0x2B, 0x38, 0x83, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_15: [u8; 58] = [
    0xA3, 0x8, 0x3F, 0x7, 0x41, 0x23, 0x3F, 0x7, 0x41, 0x23, 0x3F, 0x23, 0x41, 0x23, 0x3F, 0x7,
    0x41, 0x3, 0x3F, 0x24, 0x41, 0xA2, 0x81, 0x1, 0x41, 0x23, 0x3D, 0x3, 0x3A, 0x23, 0x41, 0x7,
    0x44, 0x23, 0x41, 0x7, 0x44, 0x23, 0x41, 0x23, 0x44, 0x23, 0x41, 0x7, 0x44, 0x3, 0x41, 0x23,
    0x46, 0xA3, 0xA9, 0x1, 0x46, 0x23, 0x41, 0x3, 0x3D, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_16: [u8; 43] = [
    0x83, 0x9, 0x44, 0x3, 0x46, 0x3, 0x46, 0x3, 0x44, 0x83, 0x4, 0x52, 0x83, 0x9, 0x46, 0x83, 0x4,
    0x55, 0x83, 0x9, 0x44, 0x83, 0x9, 0x46, 0x3, 0x46, 0x3, 0x44, 0x3, 0x46, 0x83, 0x4, 0x46, 0x83,
    0xA, 0x44, 0x83, 0x4, 0x55, 0x83, 0xA, 0x24, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_17: [u8; 49] = [
    0x83, 0xB, 0x4B, 0x3, 0x4D, 0x3, 0x4B, 0x3, 0x4D, 0xF, 0x46, 0x47, 0x3, 0x4B, 0x3, 0x50, 0x3,
    0x4D, 0x3, 0x4B, 0x7, 0x4D, 0x3, 0x4B, 0x3, 0x4D, 0x3, 0x4B, 0x3, 0x49, 0x7, 0x4B, 0x7, 0x46,
    0x83, 0xC, 0x30, 0x7, 0x2E, 0x3, 0x2C, 0x7, 0x2C, 0x83, 0xA, 0x4D, 0x3, 0x2B, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_18: [u8; 53] = [
    0x83, 0xB, 0x4B, 0x3, 0x4D, 0x3, 0x4B, 0x3, 0x4D, 0xF, 0x46, 0x47, 0x3, 0x4B, 0x3, 0x50, 0x3,
    0x52, 0x3, 0x50, 0x3, 0x4D, 0x3, 0x4B, 0x3, 0x4B, 0x3, 0x49, 0x3, 0x4B, 0x3, 0x49, 0x7, 0x44,
    0x7, 0x46, 0x83, 0xC, 0x30, 0x7, 0x2E, 0x3, 0x2C, 0x3, 0x2C, 0x83, 0xB, 0x50, 0x3, 0x4D, 0x3,
    0x4B, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_19: [u8; 46] = [
    0x85, 0x2, 0x18, 0x1, 0x18, 0x83, 0x3, 0x3E, 0x87, 0x2, 0x18, 0x1, 0x1B, 0x1, 0x1D, 0x83, 0x3,
    0x3E, 0x81, 0x2, 0x22, 0x1, 0x24, 0x5, 0x18, 0x1, 0x18, 0x83, 0x3, 0x3E, 0x87, 0x2, 0x18, 0x1,
    0x1B, 0x1, 0x1D, 0x83, 0x3, 0x3E, 0x81, 0x2, 0x22, 0x1, 0x24, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_20: [u8; 41] = [
    0x81, 0xD, 0x46, 0x1, 0x48, 0x3, 0x48, 0x3, 0x48, 0x1, 0x48, 0x1, 0x46, 0x3, 0x48, 0x1, 0x48,
    0x1, 0x46, 0x1, 0x48, 0x1, 0x4B, 0x3, 0x48, 0x1, 0x46, 0x1, 0x48, 0x3, 0x48, 0x3, 0x48, 0x1,
    0x46, 0x1, 0x48, 0x7, 0x43, 0x47, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_21: [u8; 24] = [
    0x85, 0x2, 0x1F, 0x1, 0x1F, 0x83, 0x3, 0x3E, 0x87, 0x2, 0x1F, 0x1, 0x22, 0x1, 0x24, 0x83, 0x3,
    0x3E, 0x81, 0x2, 0x29, 0x1, 0x2B, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_22: [u8; 41] = [
    0x81, 0xD, 0x4A, 0x1, 0x4D, 0x3, 0x4F, 0x3, 0x4F, 0x1, 0x4F, 0x1, 0x4D, 0x1, 0x4F, 0x1, 0x52,
    0x1, 0x4F, 0x1, 0x52, 0x3, 0x4F, 0x3, 0x4A, 0x1, 0x4A, 0x1, 0x4D, 0x1, 0x4A, 0x1, 0x4D, 0x3,
    0x4A, 0x3, 0x48, 0x7, 0x4A, 0x47, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_23: [u8; 24] = [
    0x85, 0x2, 0x1A, 0x1, 0x1A, 0x83, 0x3, 0x3E, 0x87, 0x2, 0x1A, 0x1, 0x1D, 0x1, 0x1F, 0x83, 0x3,
    0x3E, 0x81, 0x2, 0x24, 0x1, 0x26, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_24: [u8; 24] = [
    0x85, 0x2, 0x1D, 0x1, 0x1D, 0x83, 0x3, 0x3E, 0x87, 0x2, 0x1D, 0x1, 0x20, 0x1, 0x22, 0x83, 0x3,
    0x3E, 0x81, 0x2, 0x27, 0x1, 0x29, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_25: [u8; 34] = [
    0x85, 0xE, 0x3C, 0x5, 0x3A, 0x7, 0x3C, 0x1, 0x41, 0x1, 0x3F, 0x1, 0x41, 0x1, 0x41, 0x3, 0x3F,
    0x5, 0x3C, 0x5, 0x3A, 0x7, 0x3C, 0x1, 0x41, 0x1, 0x43, 0x1, 0x46, 0x1, 0x48, 0x3, 0x48, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_26: [u8; 38] = [
    0x85, 0xE, 0x3E, 0x5, 0x3C, 0x7, 0x3E, 0x1, 0x41, 0x1, 0x43, 0x1, 0x41, 0x1, 0x43, 0x1, 0x41,
    0x1, 0x43, 0x5, 0x3E, 0x5, 0x3C, 0x7, 0x3E, 0x1, 0x48, 0x1, 0x45, 0x1, 0x48, 0x1, 0x4A, 0x1,
    0x4A, 0x1, 0x4A, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_27: [u8; 46] = [
    0x83, 0xE, 0x41, 0x1, 0x44, 0x3, 0x41, 0x1, 0x3F, 0x7, 0x41, 0x1, 0x44, 0x1, 0x41, 0x1, 0x46,
    0x1, 0x44, 0x1, 0x46, 0x1, 0x44, 0x3, 0x41, 0x1, 0x44, 0x3, 0x41, 0x1, 0x3F, 0x7, 0x41, 0x1,
    0x4D, 0x1, 0x4B, 0x1, 0x4D, 0x1, 0x4D, 0x1, 0x4B, 0x1, 0x48, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_28: [u8; 24] = [
    0x83, 0xE, 0x43, 0x1, 0x46, 0x3, 0x43, 0x1, 0x41, 0x7, 0x43, 0x1, 0x4F, 0x1, 0x4D, 0x1, 0x4F,
    0x1, 0x4F, 0x1, 0x4D, 0x1, 0x4F, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_29: [u8; 45] = [
    0xA1, 0xC, 0x2B, 0x81, 0x10, 0x3C, 0x1, 0x30, 0x81, 0xF, 0x5B, 0x81, 0x10, 0x37, 0x1, 0x3C,
    0x81, 0xF, 0x5B, 0x81, 0x10, 0x3C, 0xA1, 0xC, 0x2B, 0x81, 0x10, 0x3C, 0x1, 0x48, 0x1, 0x30,
    0x81, 0xF, 0x5B, 0x81, 0x10, 0x3C, 0x81, 0xF, 0x5B, 0x81, 0x10, 0x30, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_30: [u8; 25] = [
    0x81, 0x11, 0x48, 0x1, 0x4D, 0x1, 0x4B, 0x1, 0x48, 0x1, 0x4B, 0x1, 0x48, 0x1, 0x46, 0x1, 0x4B,
    0x1, 0x48, 0x1, 0x46, 0x3, 0x48, 0x47, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_31: [u8; 25] = [
    0x81, 0x11, 0x45, 0x1, 0x4A, 0x1, 0x48, 0x1, 0x45, 0x1, 0x48, 0x1, 0x45, 0x1, 0x43, 0x1, 0x45,
    0x1, 0x43, 0x1, 0x41, 0x3, 0x3E, 0x47, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_32: [u8; 30] = [
    0x83, 0x11, 0x4A, 0x1, 0x4A, 0x1, 0x48, 0x1, 0x4A, 0x1, 0x4D, 0x1, 0x4A, 0x1, 0x48, 0x3, 0x4A,
    0x1, 0x4A, 0x1, 0x48, 0x1, 0x4A, 0x1, 0x4D, 0x1, 0x4A, 0x1, 0x48, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_33: [u8; 66] = [
    0x81, 0x11, 0x4B, 0x1, 0x48, 0x1, 0x4B, 0x1, 0x48, 0x23, 0x4B, 0x3, 0x4D, 0x22, 0x4F, 0xA0,
    0xF0, 0x3, 0x4F, 0x22, 0x52, 0xA0, 0xF1, 0x3, 0x52, 0x22, 0x4F, 0xA0, 0xF1, 0x3, 0x4F, 0x3,
    0x4D, 0x1, 0x4B, 0x1, 0x48, 0x1, 0x4B, 0x1, 0x48, 0x23, 0x4B, 0x3, 0x4D, 0x22, 0x4F, 0xA0,
    0xF1, 0x3, 0x4F, 0x22, 0x4D, 0xA0, 0xF1, 0x3, 0x4D, 0x22, 0x4B, 0xA0, 0xF1, 0x3, 0x4B, 0x3,
    0x48, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_34: [u8; 65] = [
    0x1, 0x4A, 0x1, 0x4D, 0x1, 0x4A, 0x1, 0x4D, 0x23, 0x4A, 0x3, 0x4D, 0x22, 0x4D, 0xA0, 0xF0, 0x3,
    0x4D, 0x22, 0x4F, 0xA0, 0xF1, 0x3, 0x4F, 0x22, 0x4A, 0xA0, 0xF0, 0x3, 0x4A, 0x3, 0x4D, 0x1,
    0x4A, 0x1, 0x4D, 0x1, 0x4A, 0x1, 0x4D, 0x23, 0x4A, 0x3, 0x4D, 0x22, 0x4A, 0xA0, 0xF1, 0x3,
    0x4A, 0x22, 0x48, 0xA0, 0xF1, 0x3, 0x48, 0x22, 0x46, 0xA0, 0xF1, 0x3, 0x46, 0x3, 0x43, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_35: [u8; 99] = [
    0xA3, 0x12, 0x3A, 0xA3, 0xC0, 0x0, 0x3A, 0xF, 0x3C, 0x3, 0x3C, 0x3, 0x3A, 0x23, 0x3A, 0xA3,
    0xC0, 0x0, 0x3A, 0xF, 0x3C, 0x3, 0x3C, 0x3, 0x3E, 0x23, 0x3A, 0xA3, 0xE0, 0x0, 0x3A, 0xF, 0x37,
    0x23, 0x35, 0xA3, 0xB8, 0x0, 0x35, 0x1F, 0x37, 0x23, 0x3A, 0xA3, 0xC0, 0x0, 0x3A, 0xF, 0x3C,
    0x3, 0x3C, 0x3, 0x3F, 0x23, 0x3F, 0xA3, 0xE0, 0x0, 0x3F, 0xF, 0x41, 0x3, 0x41, 0x3, 0x43, 0x23,
    0x41, 0xA3, 0xFE, 0x0, 0x41, 0xF, 0x3E, 0x23, 0x3C, 0xA3, 0xD0, 0x0, 0x3C, 0xA1, 0x13, 0x3E,
    0x21, 0x3C, 0x21, 0x3E, 0x21, 0x3C, 0x21, 0x3E, 0x21, 0x3C, 0x21, 0x3E, 0x21, 0x3C, 0xF, 0x3E,
    0xFF,
];
#[allow(dead_code)]
pub static TRACK_36: [u8; 53] = [
    0xA1, 0x80, 0x1, 0x3F, 0x21, 0x41, 0xA1, 0x80, 0x1, 0x3F, 0x25, 0x41, 0xA1, 0x80, 0x1, 0x3F,
    0x25, 0x41, 0xA3, 0xF1, 0x0, 0x41, 0x3, 0x3F, 0x3, 0x3C, 0xA1, 0x80, 0x1, 0x41, 0x21, 0x43,
    0xA1, 0x80, 0x1, 0x41, 0x25, 0x43, 0xA1, 0x80, 0x1, 0x41, 0x25, 0x43, 0xA3, 0xF1, 0x0, 0x43,
    0x3, 0x41, 0x3, 0x43, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_37: [u8; 26] = [
    0x1, 0x43, 0x21, 0x48, 0x1, 0x46, 0x1, 0x43, 0x21, 0x48, 0x1, 0x46, 0x1, 0x43, 0x21, 0x48, 0x1,
    0x46, 0x1, 0x43, 0x21, 0x48, 0x1, 0x46, 0xA3, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_38: [u8; 97] = [
    0x0, 0x24, 0x0, 0x22, 0x1, 0x24, 0x0, 0x27, 0x0, 0x24, 0x1, 0x27, 0x0, 0x29, 0x0, 0x27, 0x1,
    0x29, 0x0, 0x2B, 0x0, 0x29, 0x1, 0x2B, 0x0, 0x2E, 0x0, 0x2B, 0x1, 0x2E, 0x0, 0x30, 0x0, 0x2E,
    0x1, 0x30, 0x0, 0x33, 0x0, 0x30, 0x1, 0x33, 0x0, 0x35, 0x0, 0x33, 0x1, 0x35, 0x0, 0x37, 0x0,
    0x35, 0x1, 0x37, 0x0, 0x3A, 0x0, 0x37, 0x1, 0x3A, 0x0, 0x3C, 0x0, 0x3A, 0x1, 0x3C, 0x0, 0x3F,
    0x0, 0x3C, 0x1, 0x3F, 0x0, 0x41, 0x0, 0x3F, 0x1, 0x41, 0x0, 0x43, 0x0, 0x41, 0x1, 0x43, 0x0,
    0x46, 0x0, 0x43, 0x1, 0x46, 0x0, 0x48, 0x0, 0x46, 0x1, 0x48, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_39: [u8; 33] = [
    0x0, 0x41, 0x0, 0x3F, 0x0, 0x3C, 0x0, 0x3F, 0x0, 0x43, 0x0, 0x3F, 0x0, 0x3C, 0x0, 0x3F, 0x0,
    0x41, 0x0, 0x3F, 0x0, 0x3C, 0x0, 0x3F, 0x0, 0x43, 0x0, 0x3F, 0x0, 0x3C, 0x0, 0x3F, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_40: [u8; 49] = [
    0x0, 0x43, 0x0, 0x41, 0x0, 0x3E, 0x0, 0x41, 0x0, 0x46, 0x0, 0x41, 0x0, 0x3E, 0x0, 0x41, 0x0,
    0x43, 0x0, 0x41, 0x0, 0x3E, 0x0, 0x41, 0x0, 0x46, 0x0, 0x41, 0x0, 0x3E, 0x0, 0x41, 0x0, 0x43,
    0x0, 0x41, 0x0, 0x3E, 0x0, 0x41, 0x0, 0x46, 0x0, 0x41, 0x0, 0x3E, 0x0, 0x41, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_41: [u8; 8] = [0xA7, 0xE0, 0x0, 0x46, 0x27, 0x48, 0xA7, 0xFF];
#[allow(dead_code)]
pub static TRACK_42: [u8; 17] = [
    0xA7, 0xD0, 0x0, 0x41, 0x27, 0x43, 0xA7, 0xDF, 0x0, 0x43, 0x27, 0x41, 0x3, 0x3F, 0x3, 0x41,
    0xFF,
];
#[allow(dead_code)]
pub static TRACK_43: [u8; 21] = [
    0x1, 0x41, 0x1, 0x41, 0xA1, 0x80, 0x1, 0x3F, 0x1, 0x41, 0x1, 0x41, 0x1, 0x41, 0xA1, 0x80, 0x1,
    0x3F, 0x1, 0x41, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_44: [u8; 21] = [
    0x1, 0x43, 0x1, 0x43, 0xA1, 0x80, 0x1, 0x41, 0x1, 0x43, 0x1, 0x43, 0x1, 0x43, 0xA1, 0x80, 0x1,
    0x41, 0x1, 0x43, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_45: [u8; 23] = [
    0xA1, 0xFE, 0x0, 0x41, 0x1, 0x43, 0x1, 0x3F, 0x1, 0x3C, 0xA1, 0x80, 0x1, 0x3F, 0x1, 0x41, 0xA1,
    0x80, 0x1, 0x41, 0x1, 0x43, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_46: [u8; 83] = [
    0xA1, 0x80, 0x1, 0x41, 0x1, 0x43, 0x1, 0x43, 0xA1, 0x80, 0x1, 0x41, 0x1, 0x43, 0x1, 0x43, 0xA1,
    0x80, 0x1, 0x41, 0x1, 0x43, 0x1, 0x43, 0xA1, 0x80, 0x1, 0x41, 0x1, 0x43, 0x1, 0x43, 0xA1, 0x80,
    0x1, 0x41, 0x1, 0x43, 0x1, 0x43, 0xA1, 0x80, 0x1, 0x41, 0x1, 0x43, 0x1, 0x43, 0xA1, 0x80, 0x1,
    0x41, 0x1, 0x43, 0x1, 0x43, 0xA1, 0x80, 0x1, 0x41, 0x1, 0x43, 0x1, 0x43, 0x27, 0x43, 0x0, 0x42,
    0x0, 0x41, 0x0, 0x40, 0x0, 0x3F, 0x0, 0x3E, 0x0, 0x3D, 0x0, 0x3C, 0x0, 0x3B, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_47: [u8; 16] = [
    0x1, 0x4D, 0x1, 0x50, 0x1, 0x4D, 0x1, 0x50, 0x3, 0x4D, 0x3, 0x4B, 0x7, 0x4D, 0x47, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_48: [u8; 21] = [
    0x47, 0x87, 0xC, 0x30, 0x3, 0x2E, 0xB, 0x2C, 0x3, 0x30, 0x7, 0x30, 0x7, 0x2E, 0x3, 0x2E, 0x3,
    0x2C, 0x3, 0x2C, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_49: [u8; 18] = [
    0x83, 0x1, 0x49, 0x7, 0x46, 0xB, 0x46, 0x3, 0x46, 0x3, 0x49, 0x3, 0x4B, 0x3, 0x4B, 0x17, 0x49,
    0xFF,
];
#[allow(dead_code)]
pub static TRACK_50: [u8; 17] = [
    0xBF, 0x6, 0x33, 0x3F, 0x33, 0x3F, 0x33, 0x3F, 0x33, 0x3F, 0x2E, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F,
    0xFF,
];
#[allow(dead_code)]
pub static TRACK_51: [u8; 9] = [0xBF, 0x6, 0x59, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0xFF];
#[allow(dead_code)]
pub static TRACK_52: [u8; 9] = [0xBF, 0x6, 0x5E, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0xFF];

#[allow(dead_code)]
pub static INSTRUMENTS: [Instrument; 20] = [
    Instrument {
        pulse_width: 2048,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x0A,
        sustain_and_release: 0x0A,
        vibrato_depth: 0,
        pulse_speed: 0x20,
        fx: 0b01010101,
    },
    Instrument {
        pulse_width: 1408,
        ctrl_register: 0b00100001,
        attack_and_decay: 0x09,
        sustain_and_release: 0x0A,
        vibrato_depth: 0,
        pulse_speed: 0x00,
        fx: 0b01010101,
    },
    Instrument {
        pulse_width: 433,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x0B,
        sustain_and_release: 0xB0,
        vibrato_depth: 29,
        pulse_speed: 0x20,
        fx: 0b00001000,
    },
    Instrument {
        pulse_width: 2048,
        ctrl_register: 0b10000001,
        attack_and_decay: 0x0F,
        sustain_and_release: 0x0B,
        vibrato_depth: 0,
        pulse_speed: 0x00,
        fx: 0b11000101,
    },
    Instrument {
        pulse_width: 2176,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x0A,
        sustain_and_release: 0x08,
        vibrato_depth: 0,
        pulse_speed: 0x00,
        fx: 0b11000101,
    },
    Instrument {
        pulse_width: 128,
        ctrl_register: 0b00010101,
        attack_and_decay: 0x0F,
        sustain_and_release: 0xAD,
        vibrato_depth: 16,
        pulse_speed: 0x00,
        fx: 0b00000000,
    },
    Instrument {
        pulse_width: 2336,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x6D,
        sustain_and_release: 0x9F,
        vibrato_depth: 43,
        pulse_speed: 0x14,
        fx: 0b00000000,
    },
    Instrument {
        pulse_width: 2048,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x58,
        sustain_and_release: 0x00,
        vibrato_depth: 0,
        pulse_speed: 0x20,
        fx: 0b01010100,
    },
    Instrument {
        pulse_width: 320,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x1D,
        sustain_and_release: 0x9F,
        vibrato_depth: 43,
        pulse_speed: 0x50,
        fx: 0b00000000,
    },
    Instrument {
        pulse_width: 512,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x09,
        sustain_and_release: 0x0A,
        vibrato_depth: 0,
        pulse_speed: 0x30,
        fx: 0b01110101,
    },
    Instrument {
        pulse_width: 128,
        ctrl_register: 0b00010101,
        attack_and_decay: 0x0F,
        sustain_and_release: 0xAD,
        vibrato_depth: 0,
        pulse_speed: 0x00,
        fx: 0b11110101,
    },
    Instrument {
        pulse_width: 2048,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x09,
        sustain_and_release: 0xAA,
        vibrato_depth: 0,
        pulse_speed: 0x00,
        fx: 0b01010101,
    },
    Instrument {
        pulse_width: 2048,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x0F,
        sustain_and_release: 0x0B,
        vibrato_depth: 0,
        pulse_speed: 0x00,
        fx: 0b00000001,
    },
    Instrument {
        pulse_width: 2048,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x09,
        sustain_and_release: 0x08,
        vibrato_depth: 0,
        pulse_speed: 0x90,
        fx: 0b01010101,
    },
    Instrument {
        pulse_width: 640,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x09,
        sustain_and_release: 0x08,
        vibrato_depth: 0,
        pulse_speed: 0x40,
        fx: 0b01010100,
    },
    Instrument {
        pulse_width: 2048,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x0F,
        sustain_and_release: 0xFF,
        vibrato_depth: 0,
        pulse_speed: 0x00,
        fx: 0b00000101,
    },
    Instrument {
        pulse_width: 512,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x09,
        sustain_and_release: 0x0B,
        vibrato_depth: 0,
        pulse_speed: 0x00,
        fx: 0b00000101,
    },
    Instrument {
        pulse_width: 2048,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x1A,
        sustain_and_release: 0x9F,
        vibrato_depth: 17,
        pulse_speed: 0x80,
        fx: 0b00000000,
    },
    Instrument {
        pulse_width: 384,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x1C,
        sustain_and_release: 0xDF,
        vibrato_depth: 43,
        pulse_speed: 0x21,
        fx: 0b00000000,
    },
    Instrument {
        pulse_width: 512,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x1C,
        sustain_and_release: 0xDF,
        vibrato_depth: 51,
        pulse_speed: 0x20,
        fx: 0b00000000,
    },
];

#[allow(dead_code)]
pub static SOUNDFX: [SoundFx; 0] = [];

#[allow(dead_code)]
pub static INSTRFX: [InstrFx; 0] = [];