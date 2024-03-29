// Delta - Rob Hubbard - 1987 Thalamus

use crate::rhplayer::rhsongs::{InstrFx, Instrument, RhSongs, SidT, SoundFx};
#[allow(dead_code)]
pub static RHSONGS: RhSongs = RhSongs {
    version: 30,
    total: 13,
    channels: &CHANNELS,
    tracks: &TRACKS,
    instruments: &INSTRUMENTS,
    soundfx: &SOUNDFX,
    instrfx: &INSTRFX,
    resetspd: 1,
    skydive_v1_when: 0,
    skydive_v1_add: 0,
};

#[allow(dead_code)]
pub static CHANNEL_0: [u8; 106] = [
    3, 1, 2, 111, 1, 112, 1, 2, 0, 1, 4, 1, 2, 79, 1, 1, 5, 1, 2, 79, 1, 28, 21, 4, 27, 4, 21, 4,
    27, 4, 21, 12, 21, 4, 27, 4, 21, 4, 27, 4, 21, 64, 30, 8, 33, 8, 30, 8, 33, 8, 30, 4, 37, 4,
    39, 8, 41, 4, 37, 4, 39, 8, 41, 4, 37, 4, 39, 8, 41, 32, 41, 4, 37, 4, 39, 8, 41, 32, 41, 4,
    37, 4, 39, 8, 41, 16, 33, 4, 37, 12, 39, 16, 33, 4, 37, 12, 39, 64, 39, 32, 48, 32, 51, 32, 54,
    255,
];

#[allow(dead_code)]
pub static CHANNEL_1: [u8; 118] = [
    0, 27, 0, 1, 18, 1, 19, 15, 11, 80, 11, 1, 20, 1, 19, 79, 11, 28, 22, 2, 28, 2, 29, 4, 22, 2,
    28, 2, 29, 4, 22, 12, 22, 2, 28, 2, 29, 4, 22, 2, 28, 2, 29, 4, 22, 64, 31, 4, 34, 4, 35, 8,
    31, 4, 34, 4, 35, 8, 31, 4, 38, 4, 40, 8, 31, 4, 38, 4, 40, 8, 31, 4, 38, 4, 40, 8, 31, 32, 43,
    4, 38, 4, 40, 8, 31, 32, 43, 4, 38, 4, 40, 8, 31, 8, 34, 8, 35, 4, 46, 12, 47, 8, 34, 8, 35, 4,
    46, 12, 47, 64, 47, 96, 49, 64, 52, 64, 52, 255,
];

#[allow(dead_code)]
pub static CHANNEL_2: [u8; 116] = [
    8, 1, 8, 1, 9, 1, 8, 1, 10, 1, 8, 1, 13, 1, 8, 1, 14, 1, 15, 1, 16, 1, 17, 1, 16, 1, 17, 4, 23,
    4, 24, 8, 23, 4, 24, 4, 25, 4, 26, 2, 26, 2, 25, 4, 23, 2, 26, 2, 25, 4, 23, 2, 24, 2, 23, 2,
    24, 1, 25, 1, 26, 4, 23, 2, 26, 2, 25, 4, 23, 2, 26, 2, 25, 4, 23, 2, 32, 2, 36, 2, 24, 2, 26,
    4, 23, 2, 24, 2, 26, 4, 23, 2, 24, 2, 26, 4, 23, 2, 42, 1, 44, 1, 45, 1, 44, 1, 45, 64, 1, 2,
    50, 2, 53, 2, 55, 255,
];

#[allow(dead_code)]
pub static CHANNEL_3: [u8; 2] = [56, 254];

#[allow(dead_code)]
pub static CHANNEL_4: [u8; 2] = [57, 254];

#[allow(dead_code)]
pub static CHANNEL_5: [u8; 2] = [58, 254];

#[allow(dead_code)]
pub static CHANNEL_6: [u8; 2] = [59, 254];

#[allow(dead_code)]
pub static CHANNEL_7: [u8; 2] = [60, 254];

#[allow(dead_code)]
pub static CHANNEL_8: [u8; 2] = [61, 254];

#[allow(dead_code)]
pub static CHANNEL_9: [u8; 2] = [62, 254];

#[allow(dead_code)]
pub static CHANNEL_10: [u8; 2] = [63, 254];

#[allow(dead_code)]
pub static CHANNEL_11: [u8; 2] = [64, 254];

#[allow(dead_code)]
pub static CHANNEL_12: [u8; 2] = [65, 254];

#[allow(dead_code)]
pub static CHANNEL_13: [u8; 2] = [66, 254];

#[allow(dead_code)]
pub static CHANNEL_14: [u8; 2] = [67, 254];

#[allow(dead_code)]
pub static CHANNEL_15: [u8; 2] = [68, 254];

#[allow(dead_code)]
pub static CHANNEL_16: [u8; 2] = [69, 254];

#[allow(dead_code)]
pub static CHANNEL_17: [u8; 2] = [70, 254];

#[allow(dead_code)]
pub static CHANNEL_18: [u8; 2] = [71, 254];

#[allow(dead_code)]
pub static CHANNEL_19: [u8; 2] = [72, 254];

#[allow(dead_code)]
pub static CHANNEL_20: [u8; 2] = [73, 254];

#[allow(dead_code)]
pub static CHANNEL_21: [u8; 2] = [74, 254];

#[allow(dead_code)]
pub static CHANNEL_22: [u8; 2] = [75, 254];

#[allow(dead_code)]
pub static CHANNEL_23: [u8; 2] = [76, 254];

#[allow(dead_code)]
pub static CHANNEL_24: [u8; 2] = [77, 254];

#[allow(dead_code)]
pub static CHANNEL_25: [u8; 2] = [78, 254];

#[allow(dead_code)]
pub static CHANNEL_26: [u8; 2] = [79, 254];

#[allow(dead_code)]
pub static CHANNEL_27: [u8; 2] = [80, 254];

#[allow(dead_code)]
pub static CHANNEL_28: [u8; 2] = [81, 254];

#[allow(dead_code)]
pub static CHANNEL_29: [u8; 2] = [82, 254];

#[allow(dead_code)]
pub static CHANNEL_30: [u8; 2] = [83, 254];

#[allow(dead_code)]
pub static CHANNEL_31: [u8; 2] = [84, 254];

#[allow(dead_code)]
pub static CHANNEL_32: [u8; 2] = [85, 254];

#[allow(dead_code)]
pub static CHANNEL_33: [u8; 10] = [89, 1, 89, 2, 87, 1, 95, 1, 96, 255];

#[allow(dead_code)]
pub static CHANNEL_34: [u8; 30] = [
    88, 11, 88, 4, 97, 12, 88, 4, 97, 12, 88, 4, 97, 12, 88, 10, 97, 2, 88, 6, 97, 2, 88, 8, 97, 4,
    88, 4, 97, 255,
];

#[allow(dead_code)]
pub static CHANNEL_35: [u8; 28] = [
    86, 3, 86, 1, 90, 1, 91, 1, 92, 1, 93, 1, 90, 1, 91, 1, 92, 1, 93, 2, 91, 2, 92, 2, 93, 2, 94,
    255,
];

#[allow(dead_code)]
pub static CHANNEL_36: [u8; 10] = [103, 1, 103, 1, 104, 1, 106, 2, 108, 255];

#[allow(dead_code)]
pub static CHANNEL_37: [u8; 42] = [
    102, 7, 102, 4, 37, 4, 39, 8, 102, 4, 37, 4, 39, 8, 41, 4, 37, 4, 39, 8, 41, 4, 37, 4, 39, 8,
    107, 8, 41, 8, 107, 8, 41, 8, 102, 8, 41, 8, 102, 8, 41, 255,
];

#[allow(dead_code)]
pub static CHANNEL_38: [u8; 44] = [
    6, 1, 98, 1, 100, 1, 99, 1, 6, 1, 98, 1, 100, 1, 99, 2, 101, 1, 100, 1, 99, 2, 101, 1, 100, 1,
    99, 2, 105, 2, 101, 2, 105, 2, 101, 2, 6, 2, 101, 2, 6, 2, 101, 255,
];

#[allow(dead_code)]
pub static CHANNELS: [&[&[u8]; 3]; 13] = [
    &[&CHANNEL_0, &CHANNEL_1, &CHANNEL_2],
    &[&CHANNEL_3, &CHANNEL_4, &CHANNEL_5],
    &[&CHANNEL_6, &CHANNEL_7, &CHANNEL_8],
    &[&CHANNEL_9, &CHANNEL_10, &CHANNEL_11],
    &[&CHANNEL_12, &CHANNEL_13, &CHANNEL_14],
    &[&CHANNEL_15, &CHANNEL_16, &CHANNEL_17],
    &[&CHANNEL_18, &CHANNEL_19, &CHANNEL_20],
    &[&CHANNEL_21, &CHANNEL_22, &CHANNEL_23],
    &[&CHANNEL_24, &CHANNEL_25, &CHANNEL_26],
    &[&CHANNEL_27, &CHANNEL_28, &CHANNEL_29],
    &[&CHANNEL_30, &CHANNEL_31, &CHANNEL_32],
    &[&CHANNEL_33, &CHANNEL_34, &CHANNEL_35],
    &[&CHANNEL_36, &CHANNEL_37, &CHANNEL_38],
];

#[allow(dead_code)]
pub static TRACKS: [&[u8]; 109] = [
    &TRACK_0, &TRACK_1, &TRACK_2, &TRACK_3, &TRACK_4, &TRACK_5, &TRACK_6, &TRACK_7, &TRACK_8,
    &TRACK_9, &TRACK_10, &TRACK_11, &TRACK_12, &TRACK_13, &TRACK_14, &TRACK_15, &TRACK_16,
    &TRACK_17, &TRACK_18, &TRACK_19, &TRACK_20, &TRACK_21, &TRACK_22, &TRACK_23, &TRACK_24,
    &TRACK_25, &TRACK_26, &TRACK_27, &TRACK_28, &TRACK_29, &TRACK_30, &TRACK_31, &TRACK_32,
    &TRACK_33, &TRACK_34, &TRACK_35, &TRACK_36, &TRACK_37, &TRACK_38, &TRACK_39, &TRACK_40,
    &TRACK_41, &TRACK_42, &TRACK_43, &TRACK_44, &TRACK_45, &TRACK_46, &TRACK_47, &TRACK_48,
    &TRACK_49, &TRACK_50, &TRACK_51, &TRACK_52, &TRACK_53, &TRACK_54, &TRACK_55, &TRACK_56,
    &TRACK_57, &TRACK_58, &TRACK_59, &TRACK_60, &TRACK_61, &TRACK_62, &TRACK_63, &TRACK_64,
    &TRACK_65, &TRACK_66, &TRACK_67, &TRACK_68, &TRACK_69, &TRACK_70, &TRACK_71, &TRACK_72,
    &TRACK_73, &TRACK_74, &TRACK_75, &TRACK_76, &TRACK_77, &TRACK_78, &TRACK_79, &TRACK_80,
    &TRACK_81, &TRACK_82, &TRACK_83, &TRACK_84, &TRACK_85, &TRACK_86, &TRACK_87, &TRACK_88,
    &TRACK_89, &TRACK_90, &TRACK_91, &TRACK_92, &TRACK_93, &TRACK_94, &TRACK_95, &TRACK_96,
    &TRACK_97, &TRACK_98, &TRACK_99, &TRACK_100, &TRACK_101, &TRACK_102, &TRACK_103, &TRACK_104,
    &TRACK_105, &TRACK_106, &TRACK_107, &TRACK_108,
];
#[allow(dead_code)]
pub static TRACK_0: [u8; 4] = [0x5F, 0x5F, 0x5F, 0xFF];
#[allow(dead_code)]
pub static TRACK_1: [u8; 13] = [
    0x1, 0xB4, 0x1, 0xB2, 0x1, 0xB4, 0x1, 0xAF, 0x1, 0xAD, 0x1, 0xAF, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_2: [u8; 11] = [0x1, 0xB2, 0x1, 0xB4, 0x1, 0xAF, 0x1, 0xAD, 0x1, 0xAF, 0xFF];
#[allow(dead_code)]
pub static TRACK_3: [u8; 4] = [0x81, 0x0, 0x34, 0xFF];
#[allow(dead_code)]
pub static TRACK_4: [u8; 4] = [0x81, 0x1, 0x34, 0xFF];
#[allow(dead_code)]
pub static TRACK_5: [u8; 4] = [0x81, 0x2, 0x34, 0xFF];
#[allow(dead_code)]
pub static TRACK_6: [u8; 20] = [
    0x8B, 0xF, 0x15, 0x85, 0x11, 0x30, 0x85, 0xF, 0x15, 0x5, 0x20, 0x5, 0x21, 0x85, 0x11, 0x30,
    0x85, 0xF, 0x1C, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_7: [u8; 18] = [
    0xBF, 0x4, 0x1C, 0x3F, 0x9C, 0x3F, 0x9C, 0x37, 0x9A, 0x37, 0x9A, 0x37, 0x9A, 0x97, 0x80, 0x3,
    0x9A, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_8: [u8; 14] = [
    0x9F, 0x5, 0x17, 0x1F, 0x17, 0x1F, 0x17, 0x1F, 0x17, 0x1F, 0x17, 0x1F, 0x17, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_9: [u8; 26] = [
    0x9F, 0x4, 0x17, 0x3F, 0x17, 0x1F, 0x17, 0x9F, 0xC0, 0x1, 0x9C, 0x7F, 0x7F, 0x7F, 0x7F, 0x1F,
    0x15, 0x3F, 0x15, 0x9F, 0x80, 0x1, 0x15, 0x1F, 0x17, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_10: [u8; 25] = [
    0x9F, 0x4, 0x17, 0x3F, 0x17, 0x9F, 0xC0, 0x2, 0xA1, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x1F, 0x15,
    0x3F, 0x15, 0x9F, 0x80, 0x1, 0x15, 0x1F, 0x17, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_11: [u8; 13] = [
    0x1, 0x3B, 0x1, 0x37, 0x1, 0x39, 0x1, 0x37, 0x1, 0x34, 0x1, 0x39, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_12: [u8; 6] = [0x8B, 0x7, 0x30, 0xB, 0x3C, 0xFF];
#[allow(dead_code)]
pub static TRACK_13: [u8; 24] = [
    0x9F, 0x4, 0x17, 0x9F, 0x80, 0x1, 0x93, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x1F, 0x15, 0x3F,
    0x15, 0x9F, 0x80, 0x1, 0x15, 0x1F, 0x17, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_14: [u8; 19] = [
    0x9F, 0x4, 0x17, 0x9F, 0x80, 0x2, 0x9A, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F,
    0x1F, 0x28, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_15: [u8; 19] = [
    0x3F, 0x28, 0x3F, 0x28, 0x3F, 0x28, 0x3F, 0x28, 0x3F, 0x28, 0x3F, 0x28, 0x5F, 0x5F, 0x5F, 0x5F,
    0x5F, 0x5F, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_16: [u8; 36] = [
    0x9F, 0x7, 0x18, 0x3F, 0x98, 0x1F, 0x98, 0x37, 0x21, 0x37, 0x1F, 0x37, 0x1E, 0x37, 0x9E, 0x3F,
    0x9E, 0x3F, 0x9E, 0x1F, 0x9E, 0x37, 0x1C, 0x37, 0x98, 0x37, 0x1C, 0x37, 0x9C, 0x3F, 0x9C, 0x3F,
    0x9C, 0x1F, 0x9C, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_17: [u8; 36] = [
    0x9F, 0x7, 0x1A, 0x3F, 0x9A, 0x1F, 0x9A, 0x37, 0x1F, 0x37, 0x1E, 0x37, 0x1C, 0x37, 0x9C, 0x3F,
    0x9C, 0x3F, 0x9C, 0x1F, 0x9C, 0x37, 0x1A, 0x37, 0x98, 0x37, 0x17, 0x37, 0x97, 0x3F, 0x97, 0x3F,
    0x97, 0x1F, 0x97, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_18: [u8; 4] = [0x81, 0x6, 0x3B, 0xFF];
#[allow(dead_code)]
pub static TRACK_19: [u8; 11] = [0x1, 0x37, 0x1, 0x39, 0x1, 0x37, 0x1, 0x34, 0x1, 0x39, 0xFF];
#[allow(dead_code)]
pub static TRACK_20: [u8; 4] = [0x81, 0x8, 0x3B, 0xFF];
#[allow(dead_code)]
pub static TRACK_21: [u8; 14] = [
    0x83, 0x9, 0x46, 0x3, 0x49, 0x3, 0x4D, 0x3, 0x52, 0x3, 0x4D, 0x3, 0x49, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_22: [u8; 14] = [
    0x83, 0x3, 0x22, 0x3, 0xA5, 0x3, 0xAE, 0x3, 0xA5, 0x3, 0xA2, 0x3, 0xAE, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_23: [u8; 12] = [
    0x87, 0xA, 0x35, 0x3, 0x35, 0x3, 0x35, 0x3, 0x35, 0x3, 0x35, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_24: [u8; 11] = [0x7, 0x36, 0x3, 0x36, 0x3, 0x36, 0x3, 0x36, 0x3, 0x36, 0xFF];
#[allow(dead_code)]
pub static TRACK_25: [u8; 11] = [0x7, 0x37, 0x3, 0x37, 0x3, 0x37, 0x3, 0x37, 0x3, 0x37, 0xFF];
#[allow(dead_code)]
pub static TRACK_26: [u8; 11] = [0x7, 0x34, 0x3, 0x34, 0x3, 0x34, 0x3, 0x34, 0x3, 0x34, 0xFF];
#[allow(dead_code)]
pub static TRACK_27: [u8; 13] = [
    0x3, 0x45, 0x3, 0x49, 0x3, 0x4C, 0x3, 0x51, 0x3, 0x4C, 0x3, 0x49, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_28: [u8; 13] = [
    0x3, 0xA1, 0x3, 0xA5, 0x3, 0xA8, 0x3, 0xAD, 0x3, 0xA8, 0x3, 0xA5, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_29: [u8; 13] = [
    0x3, 0x9F, 0x3, 0xA5, 0x3, 0xA8, 0x3, 0xAB, 0x3, 0xA8, 0x3, 0xA5, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_30: [u8; 14] = [
    0x81, 0x9, 0x3A, 0x1, 0x3D, 0x1, 0x49, 0x1, 0x46, 0x1, 0x49, 0x1, 0x3D, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_31: [u8; 14] = [
    0x81, 0x1, 0xA2, 0x1, 0xA5, 0x1, 0xAE, 0x1, 0xA5, 0x1, 0xA2, 0x1, 0xAE, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_32: [u8; 23] = [
    0x9F, 0xB, 0x35, 0x6F, 0x1F, 0x36, 0x6F, 0x1F, 0x35, 0x7F, 0x7F, 0x1F, 0x36, 0x6F, 0x17, 0x37,
    0x17, 0x34, 0x1F, 0x35, 0x7F, 0x5F, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_33: [u8; 13] = [
    0x1, 0x45, 0x1, 0x49, 0x1, 0x4C, 0x1, 0x51, 0x1, 0x4C, 0x1, 0x49, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_34: [u8; 13] = [
    0x1, 0xA1, 0x1, 0xA5, 0x1, 0xA8, 0x1, 0xAD, 0x1, 0xA8, 0x1, 0xA5, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_35: [u8; 13] = [
    0x1, 0x9F, 0x1, 0xA5, 0x1, 0xA8, 0x1, 0xAB, 0x1, 0xA8, 0x1, 0xA5, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_36: [u8; 12] = [
    0x9F, 0x7, 0x2D, 0x6F, 0x1F, 0x2B, 0x6F, 0x1F, 0x2E, 0x7F, 0x5F, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_37: [u8; 13] = [
    0x1, 0x39, 0x1, 0x3E, 0x1, 0x40, 0x1, 0x42, 0x1, 0x40, 0x1, 0x3E, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_38: [u8; 13] = [
    0x1, 0xA1, 0x1, 0xA6, 0x1, 0xA8, 0x1, 0xAA, 0x1, 0xA8, 0x1, 0xA6, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_39: [u8; 13] = [
    0x1, 0x3B, 0x1, 0x40, 0x1, 0x42, 0x1, 0x44, 0x1, 0x42, 0x1, 0x40, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_40: [u8; 13] = [
    0x1, 0xA3, 0x1, 0xA8, 0x1, 0xAA, 0x1, 0xAC, 0x1, 0xAA, 0x1, 0xA8, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_41: [u8; 13] = [
    0x1, 0x3A, 0x1, 0x3D, 0x1, 0x41, 0x1, 0x42, 0x1, 0x41, 0x1, 0x3D, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_42: [u8; 30] = [
    0x9F, 0xB, 0x41, 0x7F, 0x7F, 0x3F, 0x44, 0x7F, 0x7F, 0x3F, 0x42, 0x7F, 0x7F, 0x37, 0x41, 0x77,
    0x37, 0x3A, 0x57, 0x17, 0x39, 0x77, 0x37, 0x3B, 0x77, 0x3F, 0x3A, 0x7F, 0x5F, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_43: [u8; 13] = [
    0x1, 0xBA, 0x1, 0xBD, 0x1, 0xC9, 0x1, 0xC6, 0x1, 0xC9, 0x1, 0xBD, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_44: [u8; 9] = [0x3F, 0x45, 0x7F, 0x7F, 0x3F, 0x43, 0x7F, 0x7F, 0xFF];
#[allow(dead_code)]
pub static TRACK_45: [u8; 10] = [0x37, 0x3E, 0x77, 0x37, 0x40, 0x77, 0x7F, 0x7F, 0x5F, 0xFF];
#[allow(dead_code)]
pub static TRACK_46: [u8; 13] = [
    0x1, 0x9A, 0x1, 0xA6, 0x1, 0xAD, 0x1, 0xB2, 0x1, 0xAD, 0x1, 0xA6, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_47: [u8; 13] = [
    0x1, 0x9C, 0x1, 0xA8, 0x1, 0xAF, 0x1, 0xB4, 0x1, 0xAF, 0x1, 0xA8, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_48: [u8; 26] = [
    0x82, 0x9, 0x34, 0x2, 0x3E, 0x2, 0x40, 0x2, 0x3E, 0x2, 0x37, 0x2, 0x36, 0x2, 0x34, 0x2, 0x3E,
    0x2, 0x40, 0x2, 0x3E, 0x2, 0x37, 0x2, 0x36, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_49: [u8; 8] = [0x8A, 0x1, 0x1C, 0xA0, 0x8A, 0x3, 0xA8, 0xFF];
#[allow(dead_code)]
pub static TRACK_50: [u8; 48] = [
    0x97, 0xB, 0x2F, 0x77, 0x37, 0x32, 0x77, 0x37, 0x30, 0x77, 0x37, 0x2F, 0x17, 0x28, 0x37, 0x28,
    0x77, 0x37, 0x26, 0x77, 0x37, 0x24, 0x77, 0x37, 0x23, 0x57, 0xB, 0x34, 0xB, 0x3C, 0xB, 0x3B,
    0xB, 0x39, 0x17, 0x37, 0x17, 0x36, 0x17, 0x34, 0xB, 0x2D, 0xB, 0x2F, 0x1F, 0x30, 0x4F, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_51: [u8; 14] = [
    0x83, 0x9, 0x34, 0x3, 0xBE, 0x3, 0xC0, 0x3, 0xBE, 0x3, 0xB7, 0x3, 0xB6, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_52: [u8; 8] = [0x8A, 0x1, 0x10, 0xA0, 0x83, 0x3, 0xA8, 0xFF];
#[allow(dead_code)]
pub static TRACK_53: [u8; 27] = [
    0x97, 0x0, 0x2F, 0x77, 0x37, 0x32, 0x77, 0x37, 0x30, 0x77, 0x37, 0x2F, 0x17, 0x28, 0x37, 0x28,
    0x77, 0x37, 0x26, 0x77, 0x37, 0x24, 0x77, 0x37, 0x23, 0x57, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_54: [u8; 14] = [
    0x83, 0x0, 0x34, 0x3, 0xBE, 0x3, 0xC0, 0x3, 0xBE, 0x3, 0xB7, 0x3, 0xB6, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_55: [u8; 29] = [
    0xBF, 0xC0, 0x1, 0x28, 0x7F, 0x1F, 0x28, 0xBF, 0xC0, 0x1, 0x26, 0x7F, 0x1F, 0x26, 0xBF, 0xC0,
    0x1, 0x24, 0x7F, 0x1F, 0x24, 0xBF, 0xC0, 0x1, 0x23, 0x7F, 0x1F, 0x23, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_56: [u8; 21] = [
    0x81, 0xC, 0x43, 0x1, 0x43, 0x3, 0x3E, 0x3, 0x41, 0x1, 0x3C, 0x3, 0x3E, 0x3, 0x3A, 0x1, 0x35,
    0xF, 0x37, 0x4F, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_57: [u8; 11] = [0x87, 0x9, 0x2B, 0x7, 0x29, 0x7, 0x27, 0xF, 0x26, 0x4F, 0xFF];
#[allow(dead_code)]
pub static TRACK_58: [u8; 7] = [0x97, 0x7, 0x1F, 0xF, 0x1F, 0x4F, 0xFF];
#[allow(dead_code)]
pub static TRACK_59: [u8; 39] = [
    0x81, 0xC, 0x3A, 0x1, 0x3A, 0x3, 0x3C, 0x3, 0x46, 0x1, 0x3C, 0x1, 0x46, 0x1, 0x48, 0x1, 0x46,
    0x3, 0x3C, 0x1, 0x46, 0x1, 0x48, 0x3, 0x41, 0x3, 0x43, 0x1, 0x43, 0x1, 0x41, 0x3, 0x3A, 0x3,
    0x41, 0xF, 0x43, 0x47, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_60: [u8; 17] = [
    0x87, 0x9, 0x18, 0x7, 0x1A, 0x7, 0x1B, 0x7, 0x1D, 0x7, 0x1F, 0x7, 0x1D, 0xF, 0x1F, 0x47, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_61: [u8; 11] = [0x97, 0x9, 0x2B, 0x7, 0x30, 0xF, 0x32, 0xF, 0x32, 0x47, 0xFF];
#[allow(dead_code)]
pub static TRACK_62: [u8; 13] = [
    0x97, 0xB, 0x37, 0x7, 0x32, 0xF, 0x31, 0xF, 0x2E, 0x1F, 0x2F, 0x47, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_63: [u8; 13] = [
    0x97, 0xB, 0x32, 0x7, 0x2F, 0xF, 0x2E, 0xF, 0x2B, 0x1F, 0x2B, 0x47, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_64: [u8; 21] = [
    0x87, 0x9, 0x1F, 0x7, 0x1E, 0x7, 0x1D, 0x7, 0x1C, 0x7, 0x1B, 0x7, 0x19, 0x7, 0x18, 0x7, 0x19,
    0x1F, 0x13, 0x47, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_65: [u8; 17] = [
    0x87, 0xB, 0x2F, 0x7, 0x38, 0x7, 0x37, 0x7, 0x34, 0xF, 0x33, 0xF, 0x32, 0x1F, 0x32, 0x47, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_66: [u8; 17] = [
    0x87, 0xB, 0x2B, 0x7, 0x35, 0x7, 0x34, 0x7, 0x31, 0xF, 0x30, 0xF, 0x30, 0x1F, 0x2F, 0x47, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_67: [u8; 17] = [
    0x8F, 0x9, 0x1F, 0xF, 0x21, 0x7, 0x22, 0x7, 0x24, 0x7, 0x26, 0x7, 0x27, 0x1F, 0x13, 0x47, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_68: [u8; 23] = [
    0x8F, 0x7, 0x3A, 0x1, 0x37, 0x1, 0x3A, 0x1, 0x37, 0x1, 0x3A, 0xF, 0x3C, 0x1, 0x37, 0x1, 0x3A,
    0x3, 0x3C, 0x17, 0x3D, 0x47, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_69: [u8; 11] = [
    0x8F, 0xB, 0x37, 0x7, 0x33, 0x17, 0x34, 0x17, 0x35, 0x47, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_70: [u8; 17] = [
    0x87, 0x9, 0x27, 0x7, 0x26, 0x7, 0x25, 0x7, 0x24, 0x7, 0x22, 0x7, 0x21, 0x17, 0x16, 0x47, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_71: [u8; 29] = [
    0x85, 0xC, 0x2F, 0x1, 0x34, 0x5, 0x37, 0x1, 0x3B, 0x5, 0x40, 0x1, 0x3B, 0x5, 0x37, 0x1, 0x34,
    0x5, 0x2F, 0x1, 0x34, 0x5, 0x37, 0x1, 0x3B, 0xF, 0x3A, 0x47, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_72: [u8; 17] = [
    0x87, 0xD, 0x10, 0x7, 0x2F, 0x7, 0x13, 0x7, 0x2F, 0x7, 0x10, 0x7, 0x2F, 0xF, 0x32, 0x47, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_73: [u8; 17] = [
    0x87, 0xE, 0x10, 0x7, 0x2B, 0x7, 0x13, 0x7, 0x2B, 0x7, 0x10, 0x7, 0x2B, 0xF, 0x13, 0x47, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_74: [u8; 25] = [
    0x85, 0xC, 0x3E, 0x1, 0x3F, 0x5, 0x3E, 0x7, 0x3D, 0x1, 0x3F, 0x5, 0x3E, 0x7, 0x3D, 0x1, 0x43,
    0x7, 0x42, 0xF, 0x3B, 0x1F, 0x30, 0x47, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_75: [u8; 21] = [
    0x87, 0xD, 0x29, 0x7, 0x2F, 0x7, 0x28, 0x7, 0x2E, 0x7, 0x28, 0x7, 0x2D, 0x7, 0x27, 0x7, 0x2D,
    0x1F, 0x2C, 0x47, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_76: [u8; 13] = [
    0x8F, 0xE, 0x26, 0xF, 0x25, 0xF, 0x24, 0xF, 0x23, 0x1F, 0x1D, 0x47, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_77: [u8; 61] = [
    0x85, 0xC, 0x45, 0x1, 0x41, 0x5, 0x3E, 0x1, 0x41, 0x5, 0x45, 0x1, 0x41, 0x5, 0x3E, 0x1, 0x41,
    0x5, 0x45, 0x1, 0x41, 0x5, 0x3E, 0x1, 0x41, 0x5, 0x45, 0x1, 0x41, 0x5, 0x3E, 0x1, 0x41, 0x5,
    0x45, 0x1, 0x41, 0x5, 0x3E, 0x1, 0x41, 0x5, 0x40, 0x1, 0x3C, 0x5, 0x39, 0x1, 0x34, 0x5, 0x31,
    0x1, 0x36, 0x5, 0x39, 0x1, 0x3D, 0xF, 0x45, 0x47, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_78: [u8; 41] = [
    0x87, 0xB, 0x26, 0x7, 0x26, 0x7, 0x26, 0x3, 0x26, 0x3, 0x28, 0x3, 0x29, 0x3, 0x2B, 0x3, 0x2D,
    0x3, 0x2E, 0x7, 0x2D, 0x3, 0x2D, 0x3, 0x2E, 0x3, 0x30, 0x3, 0x32, 0x3, 0x34, 0x3, 0x35, 0x7,
    0x34, 0x7, 0x30, 0x1F, 0x31, 0x47, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_79: [u8; 17] = [
    0x8F, 0xE, 0x26, 0xF, 0x25, 0xF, 0x24, 0xF, 0x23, 0xF, 0x22, 0xF, 0x21, 0x1F, 0x1E, 0x47, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_80: [u8; 23] = [
    0x87, 0x0, 0x3B, 0x7, 0x3C, 0x7, 0x3E, 0x7, 0x3F, 0x7, 0x3D, 0x7, 0x3B, 0x7, 0x3D, 0x7, 0x3B,
    0x7, 0x38, 0x1F, 0x3A, 0x4F, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_81: [u8; 23] = [
    0x87, 0x6, 0x3C, 0x7, 0x45, 0x7, 0x46, 0x7, 0x47, 0x7, 0x46, 0x7, 0x44, 0x7, 0x46, 0x7, 0x44,
    0x7, 0x3F, 0x1F, 0x3E, 0x4F, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_82: [u8; 23] = [
    0x87, 0x9, 0x15, 0x7, 0x9C, 0x7, 0x9C, 0x7, 0x94, 0x7, 0x9B, 0x7, 0x9B, 0x7, 0x91, 0x7, 0x94,
    0x7, 0x97, 0x1F, 0x96, 0x4F, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_83: [u8; 29] = [
    0x87, 0x0, 0x3B, 0x7, 0x3C, 0x7, 0x3E, 0x7, 0x3F, 0x7, 0x3D, 0x7, 0x3B, 0x7, 0x34, 0x7, 0x32,
    0x7, 0x33, 0xF, 0x34, 0x9F, 0xC0, 0x8, 0x34, 0x7F, 0x7F, 0x4F, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_84: [u8; 42] = [
    0x83, 0xC, 0x3B, 0x3, 0x40, 0x3, 0x47, 0x3, 0x4C, 0x3, 0x4A, 0x3, 0x4C, 0x3, 0x47, 0x3, 0x4B,
    0x3, 0x46, 0x3, 0x3F, 0x3, 0x3B, 0x3, 0x3F, 0x7, 0x3C, 0x7, 0x3A, 0x7, 0x39, 0x8F, 0x0, 0x34,
    0x9F, 0x84, 0x12, 0x34, 0x7F, 0x7F, 0x4F, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_85: [u8; 29] = [
    0x87, 0x9, 0x15, 0x7, 0x9C, 0x7, 0x9C, 0x7, 0x94, 0x7, 0x9B, 0x7, 0x9B, 0x7, 0x92, 0x7, 0x9A,
    0x7, 0x97, 0xF, 0x1C, 0x9F, 0xC0, 0x10, 0x10, 0x7F, 0x7F, 0x4F, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_86: [u8; 130] = [
    0x83, 0xF, 0x1B, 0x3, 0x1B, 0x3, 0x27, 0x3, 0x1B, 0x3, 0x27, 0x3, 0x1B, 0x3, 0x25, 0x3, 0x27,
    0x3, 0x1B, 0x3, 0x1B, 0x3, 0x27, 0x3, 0x1B, 0x3, 0x22, 0x3, 0x20, 0x3, 0x1E, 0x3, 0x1D, 0x3,
    0x19, 0x3, 0x19, 0x3, 0x25, 0x3, 0x19, 0x3, 0x25, 0x3, 0x19, 0x3, 0x23, 0x3, 0x25, 0x3, 0x19,
    0x3, 0x19, 0x3, 0x25, 0x3, 0x19, 0x3, 0x22, 0x3, 0x20, 0x3, 0x1E, 0x3, 0x1D, 0x3, 0x17, 0x3,
    0x17, 0x3, 0x23, 0x3, 0x17, 0x3, 0x23, 0x3, 0x17, 0x3, 0x22, 0x3, 0x23, 0x3, 0x17, 0x3, 0x17,
    0x3, 0x23, 0x3, 0x17, 0x3, 0x22, 0x3, 0x20, 0x3, 0x1E, 0x3, 0x1D, 0x3, 0x1E, 0x3, 0x1E, 0x3,
    0x2A, 0x3, 0x1E, 0x3, 0x1D, 0x3, 0x1D, 0x3, 0x29, 0x3, 0x1D, 0x3, 0x1B, 0x3, 0x1B, 0x3, 0x27,
    0x3, 0x1B, 0x3, 0x16, 0x3, 0x16, 0x3, 0x22, 0x3, 0x16, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_87: [u8; 130] = [
    0x83, 0x10, 0x3F, 0x3, 0x3A, 0x3, 0x3F, 0x3, 0x42, 0x3, 0x46, 0x3, 0x42, 0x3, 0x3F, 0x3, 0x3A,
    0x3, 0x3F, 0x3, 0x3A, 0x3, 0x3F, 0x3, 0x42, 0x3, 0x46, 0x3, 0x44, 0x3, 0x42, 0x3, 0x41, 0x3,
    0x41, 0x3, 0x3A, 0x3, 0x3D, 0x3, 0x41, 0x3, 0x46, 0x3, 0x41, 0x3, 0x3D, 0x3, 0x3A, 0x3, 0x41,
    0x3, 0x3A, 0x3, 0x3D, 0x3, 0x41, 0x3, 0x46, 0x3, 0x44, 0x3, 0x42, 0x3, 0x41, 0x3, 0x3F, 0x3,
    0x38, 0x3, 0x3B, 0x3, 0x3F, 0x3, 0x44, 0x3, 0x42, 0x3, 0x41, 0x3, 0x3F, 0x3, 0x42, 0x3, 0x3B,
    0x3, 0x3F, 0x3, 0x42, 0x3, 0x46, 0x3, 0x44, 0x3, 0x42, 0x3, 0x44, 0x3, 0x46, 0x3, 0x47, 0x3,
    0x46, 0x3, 0x47, 0x3, 0x46, 0x3, 0x47, 0x3, 0x46, 0x3, 0x47, 0x3, 0x46, 0x3, 0x47, 0x3, 0x46,
    0x3, 0x47, 0x3, 0x46, 0x3, 0x44, 0x3, 0x42, 0x3, 0x41, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_88: [u8; 18] = [
    0x81, 0x12, 0x33, 0x1, 0x3F, 0x1, 0x4B, 0x1, 0x3F, 0x83, 0x11, 0x30, 0x81, 0x12, 0x33, 0x1,
    0x3F, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_89: [u8; 36] = [
    0x87, 0xB, 0x3F, 0x7, 0x3A, 0x1F, 0x3F, 0xF, 0x46, 0x7, 0x41, 0x7, 0x3A, 0x1F, 0x41, 0xF, 0x46,
    0x7, 0x3F, 0x7, 0x38, 0x1F, 0x3F, 0xF, 0x47, 0xF, 0x46, 0xF, 0x47, 0x17, 0x46, 0x3, 0x42, 0x3,
    0x41, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_90: [u8; 17] = [
    0x3, 0x14, 0x3, 0x14, 0x3, 0x20, 0x3, 0x14, 0x3, 0x20, 0x3, 0x14, 0x3, 0x1E, 0x3, 0x20, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_91: [u8; 17] = [
    0x3, 0x19, 0x3, 0x19, 0x3, 0x25, 0x3, 0x19, 0x3, 0x25, 0x3, 0x19, 0x3, 0x23, 0x3, 0x25, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_92: [u8; 17] = [
    0x3, 0x1C, 0x3, 0x1C, 0x3, 0x28, 0x3, 0x1C, 0x3, 0x28, 0x3, 0x1C, 0x3, 0x27, 0x3, 0x28, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_93: [u8; 17] = [
    0x3, 0x17, 0x3, 0x17, 0x3, 0x23, 0x3, 0x17, 0x3, 0x23, 0x3, 0x17, 0x3, 0x22, 0x3, 0x23, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_94: [u8; 17] = [
    0x3, 0x12, 0x3, 0x12, 0x3, 0x1E, 0x3, 0x12, 0x3, 0x1E, 0x3, 0x12, 0x3, 0x1D, 0x3, 0x1E, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_95: [u8; 74] = [
    0x8B, 0x13, 0x3F, 0xB, 0x38, 0x3, 0x3A, 0x3, 0x3B, 0xB, 0x3D, 0xB, 0x40, 0x7, 0x3D, 0x3, 0x40,
    0x7, 0x40, 0xB, 0x40, 0x3, 0x3F, 0x3, 0x3D, 0x1F, 0x3F, 0xB, 0x3F, 0xB, 0x38, 0x3, 0x3A, 0x3,
    0x3B, 0xB, 0x3D, 0xB, 0x40, 0x7, 0x44, 0x3, 0x44, 0x3, 0x45, 0x3, 0x44, 0x3, 0x45, 0x3, 0x44,
    0x3, 0x42, 0x3, 0x40, 0x3, 0x42, 0x3, 0x46, 0x3, 0x47, 0x3, 0x46, 0x3, 0x47, 0x3, 0x44, 0x3,
    0x42, 0x3, 0x40, 0x3, 0x3F, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_96: [u8; 105] = [
    0x87, 0xB, 0x3D, 0x7, 0x38, 0x17, 0x44, 0xA2, 0x13, 0x38, 0x21, 0xBD, 0x22, 0xC4, 0x22, 0xB8,
    0x21, 0xBD, 0x22, 0xC4, 0x22, 0xB8, 0x21, 0xBD, 0x2, 0xC4, 0x87, 0xB, 0x40, 0x7, 0x3B, 0x17,
    0x47, 0xA2, 0x13, 0x3B, 0x21, 0xC0, 0x22, 0xC7, 0x22, 0xBB, 0x21, 0xC0, 0x22, 0xC7, 0x22, 0xBB,
    0x21, 0xC0, 0x2, 0xC7, 0x87, 0xB, 0x42, 0x7, 0x3B, 0x17, 0x47, 0xA2, 0x13, 0x3B, 0x21, 0xC2,
    0x22, 0xC7, 0x22, 0xBB, 0x21, 0xC2, 0x22, 0xC7, 0x22, 0xBB, 0x21, 0xC2, 0x2, 0xC7, 0x87, 0xB,
    0x42, 0x7, 0x3D, 0x17, 0x49, 0xA2, 0x13, 0x3D, 0x21, 0xC2, 0x22, 0xC9, 0x22, 0xBD, 0x21, 0xC2,
    0x22, 0xC9, 0x22, 0xBD, 0x21, 0xC2, 0x2, 0xC9, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_97: [u8; 18] = [
    0x81, 0x12, 0x38, 0x1, 0x44, 0x1, 0x38, 0x1, 0x3F, 0x83, 0x11, 0x30, 0x81, 0x12, 0x38, 0x1,
    0x44, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_98: [u8; 20] = [
    0x8B, 0xF, 0x13, 0x85, 0x11, 0x30, 0x85, 0xF, 0x13, 0x5, 0x1E, 0x5, 0x1F, 0x85, 0x11, 0x30,
    0x85, 0xF, 0x1C, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_99: [u8; 20] = [
    0x8B, 0xF, 0x1C, 0x85, 0x11, 0x30, 0x85, 0xF, 0x1C, 0x5, 0x26, 0x5, 0x28, 0x85, 0x11, 0x30,
    0x85, 0xF, 0x23, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_100: [u8; 20] = [
    0x8B, 0xF, 0x1A, 0x85, 0x11, 0x30, 0x85, 0xF, 0x1A, 0x5, 0x25, 0x5, 0x26, 0x85, 0x11, 0x30,
    0x85, 0xF, 0x1A, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_101: [u8; 20] = [
    0x8B, 0xF, 0x1E, 0x85, 0x11, 0x30, 0x85, 0xF, 0x1E, 0x5, 0x1D, 0x5, 0x1E, 0x85, 0x11, 0x30,
    0x85, 0xF, 0x19, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_102: [u8; 14] = [
    0x81, 0x14, 0x39, 0x1, 0x3D, 0x1, 0x40, 0x1, 0x45, 0x1, 0x40, 0x1, 0x3D, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_103: [u8; 40] = [
    0x85, 0x10, 0x39, 0x5, 0x3B, 0x5, 0x39, 0x17, 0x40, 0x5, 0x39, 0x5, 0x39, 0x5, 0x3B, 0x5, 0x39,
    0x17, 0x45, 0x5, 0x39, 0x5, 0x39, 0x5, 0x3B, 0x5, 0x39, 0x17, 0x40, 0x5, 0x39, 0xB, 0x3B, 0xB,
    0x40, 0xB, 0x42, 0xB, 0x3B, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_104: [u8; 54] = [
    0x17, 0x3D, 0xB, 0x3A, 0xB, 0x41, 0x37, 0x44, 0x6B, 0x5, 0x42, 0x5, 0x41, 0x17, 0x40, 0xB,
    0x3E, 0xB, 0x3D, 0x37, 0x3B, 0x6B, 0x5, 0x3A, 0x5, 0x3B, 0x17, 0x3A, 0xB, 0x41, 0xB, 0x3D,
    0x37, 0x44, 0x6B, 0x5, 0x42, 0x5, 0x41, 0xB, 0x40, 0x17, 0x3D, 0x5, 0x38, 0x5, 0x39, 0xB, 0x3B,
    0xB, 0x40, 0x17, 0x42, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_105: [u8; 20] = [
    0x8B, 0xF, 0x20, 0x85, 0x11, 0x30, 0x85, 0xF, 0x20, 0x5, 0x1F, 0x5, 0x20, 0x85, 0x11, 0x30,
    0x85, 0xF, 0x1B, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_106: [u8; 49] = [
    0xB, 0x43, 0x5, 0x46, 0x5, 0x44, 0x37, 0x43, 0x77, 0x17, 0x3F, 0xB, 0x41, 0x5, 0x44, 0x5, 0x42,
    0x37, 0x41, 0x77, 0x4B, 0x5, 0x3C, 0x5, 0x3D, 0xB, 0x3F, 0x5, 0x46, 0x5, 0x44, 0x37, 0x43,
    0x77, 0x17, 0x3F, 0x17, 0x41, 0x17, 0x3D, 0x17, 0x38, 0xB, 0x35, 0x5, 0x36, 0x5, 0x38, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_107: [u8; 13] = [
    0x1, 0x3A, 0x1, 0x3C, 0x1, 0x3F, 0x1, 0x43, 0x1, 0x3F, 0x1, 0x3C, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_108: [u8; 18] = [
    0x17, 0x39, 0x17, 0x38, 0x17, 0x36, 0x17, 0x34, 0x37, 0x35, 0x77, 0x77, 0x4B, 0x5, 0x36, 0x5,
    0x38, 0xFF,
];

#[allow(dead_code)]
pub static INSTRUMENTS: [Instrument; 22] = [
    Instrument {
        pulse_width: 512,
        ctrl_register: 0b01000011,
        attack_and_decay: 0x00,
        sustain_and_release: 0xDC,
        vibrato_depth: 0,
        pulse_speed: 0x01,
        fx: 0b10100000,
    },
    Instrument {
        pulse_width: 512,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x00,
        sustain_and_release: 0xBC,
        vibrato_depth: 0,
        pulse_speed: 0x02,
        fx: 0b10100000,
    },
    Instrument {
        pulse_width: 256,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x00,
        sustain_and_release: 0xBC,
        vibrato_depth: 0,
        pulse_speed: 0x04,
        fx: 0b10100000,
    },
    Instrument {
        pulse_width: 64,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x00,
        sustain_and_release: 0xB9,
        vibrato_depth: 0,
        pulse_speed: 0x08,
        fx: 0b10100000,
    },
    Instrument {
        pulse_width: 512,
        ctrl_register: 0b01000001,
        attack_and_decay: 0xDE,
        sustain_and_release: 0x7A,
        vibrato_depth: 34,
        pulse_speed: 0x21,
        fx: 0b00000000,
    },
    Instrument {
        pulse_width: 2112,
        ctrl_register: 0b00000000,
        attack_and_decay: 0x0A,
        sustain_and_release: 0x08,
        vibrato_depth: 0,
        pulse_speed: 0x00,
        fx: 0b00000000,
    },
    Instrument {
        pulse_width: 176,
        ctrl_register: 0b00010001,
        attack_and_decay: 0x08,
        sustain_and_release: 0x99,
        vibrato_depth: 0,
        pulse_speed: 0x00,
        fx: 0b01000100,
    },
    Instrument {
        pulse_width: 128,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x0E,
        sustain_and_release: 0xA7,
        vibrato_depth: 42,
        pulse_speed: 0x40,
        fx: 0b00000000,
    },
    Instrument {
        pulse_width: 0,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x08,
        sustain_and_release: 0x99,
        vibrato_depth: 0,
        pulse_speed: 0x81,
        fx: 0b01000100,
    },
    Instrument {
        pulse_width: 608,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x08,
        sustain_and_release: 0x99,
        vibrato_depth: 0,
        pulse_speed: 0x00,
        fx: 0b01000100,
    },
    Instrument {
        pulse_width: 320,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x0A,
        sustain_and_release: 0x89,
        vibrato_depth: 0,
        pulse_speed: 0x60,
        fx: 0b00000100,
    },
    Instrument {
        pulse_width: 176,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x09,
        sustain_and_release: 0xAB,
        vibrato_depth: 35,
        pulse_speed: 0x30,
        fx: 0b00000000,
    },
    Instrument {
        pulse_width: 688,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x49,
        sustain_and_release: 0x79,
        vibrato_depth: 27,
        pulse_speed: 0x60,
        fx: 0b00000100,
    },
    Instrument {
        pulse_width: 288,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x57,
        sustain_and_release: 0x8A,
        vibrato_depth: 35,
        pulse_speed: 0x30,
        fx: 0b00000100,
    },
    Instrument {
        pulse_width: 1232,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x57,
        sustain_and_release: 0x8A,
        vibrato_depth: 0,
        pulse_speed: 0x10,
        fx: 0b00000000,
    },
    Instrument {
        pulse_width: 0,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x0C,
        sustain_and_release: 0xC9,
        vibrato_depth: 0,
        pulse_speed: 0x50,
        fx: 0b01000100,
    },
    Instrument {
        pulse_width: 320,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x0C,
        sustain_and_release: 0x99,
        vibrato_depth: 35,
        pulse_speed: 0x40,
        fx: 0b00000000,
    },
    Instrument {
        pulse_width: 192,
        ctrl_register: 0b00010001,
        attack_and_decay: 0x0F,
        sustain_and_release: 0x09,
        vibrato_depth: 0,
        pulse_speed: 0x00,
        fx: 0b00001011,
    },
    Instrument {
        pulse_width: 2176,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x08,
        sustain_and_release: 0x08,
        vibrato_depth: 0,
        pulse_speed: 0x50,
        fx: 0b00000100,
    },
    Instrument {
        pulse_width: 2176,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x3D,
        sustain_and_release: 0x00,
        vibrato_depth: 18,
        pulse_speed: 0x30,
        fx: 0b00000100,
    },
    Instrument {
        pulse_width: 2816,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x0D,
        sustain_and_release: 0xFC,
        vibrato_depth: 0,
        pulse_speed: 0xF0,
        fx: 0b10100000,
    },
    Instrument {
        pulse_width: 448,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x0A,
        sustain_and_release: 0x9A,
        vibrato_depth: 0,
        pulse_speed: 0xE0,
        fx: 0b01100100,
    },
];

#[allow(dead_code)]
pub static SOUNDFX: [SoundFx; 0] = [];

#[allow(dead_code)]
pub static INSTRFX: [InstrFx; 22] = [
    InstrFx {
        vibdepth_note: 10,
        arpt: 0b00000001,
        skydive: 0b10000001,
        arpt_counter: 2,
        notenum: 0,
        pw_minmax: 0x50,
        resfilt: 0b11110001,
        fchi: 0b00001101,
    },
    InstrFx {
        vibdepth_note: 3,
        arpt: 0b11111111,
        skydive: 0b10000001,
        arpt_counter: 2,
        notenum: 0,
        pw_minmax: 0x30,
        resfilt: 0b11110001,
        fchi: 0b11101000,
    },
    InstrFx {
        vibdepth_note: 3,
        arpt: 0b11111111,
        skydive: 0b10000001,
        arpt_counter: 2,
        notenum: 0,
        pw_minmax: 0x2f,
        resfilt: 0b11110010,
        fchi: 0b11101000,
    },
    InstrFx {
        vibdepth_note: 4,
        arpt: 0b11111101,
        skydive: 0b10000001,
        arpt_counter: 2,
        notenum: 0,
        pw_minmax: 0x30,
        resfilt: 0b11110001,
        fchi: 0b11110111,
    },
    InstrFx {
        vibdepth_note: 0,
        arpt: 0b10000001,
        skydive: 0b00000000,
        arpt_counter: 2,
        notenum: 0,
        pw_minmax: 0x30,
        resfilt: 0b11110010,
        fchi: 0b00011000,
    },
    InstrFx {
        vibdepth_note: 0,
        arpt: 0b00010001,
        skydive: 0b10000001,
        arpt_counter: 8,
        notenum: 51,
        pw_minmax: 0xf8,
        resfilt: 0b00000000,
        fchi: 0b00000000,
    },
    InstrFx {
        vibdepth_note: 101,
        arpt: 0b10000001,
        skydive: 0b00010001,
        arpt_counter: 2,
        notenum: 0,
        pw_minmax: 0xfc,
        resfilt: 0b00000000,
        fchi: 0b00000000,
    },
    InstrFx {
        vibdepth_note: 2,
        arpt: 0b00100001,
        skydive: 0b10000001,
        arpt_counter: 3,
        notenum: 61,
        pw_minmax: 0xfa,
        resfilt: 0b00000000,
        fchi: 0b00000000,
    },
    InstrFx {
        vibdepth_note: 101,
        arpt: 0b10000001,
        skydive: 0b10000001,
        arpt_counter: 4,
        notenum: 0,
        pw_minmax: 0xfc,
        resfilt: 0b00000000,
        fchi: 0b00000000,
    },
    InstrFx {
        vibdepth_note: 80,
        arpt: 0b00010101,
        skydive: 0b00000000,
        arpt_counter: 2,
        notenum: 0,
        pw_minmax: 0xfc,
        resfilt: 0b00000000,
        fchi: 0b00000000,
    },
    InstrFx {
        vibdepth_note: 65,
        arpt: 0b00010001,
        skydive: 0b00000000,
        arpt_counter: 1,
        notenum: 0,
        pw_minmax: 0xfb,
        resfilt: 0b00000000,
        fchi: 0b00000000,
    },
    InstrFx {
        vibdepth_note: 0,
        arpt: 0b10000001,
        skydive: 0b10000001,
        arpt_counter: 2,
        notenum: 64,
        pw_minmax: 0xfc,
        resfilt: 0b00000000,
        fchi: 0b00000000,
    },
    InstrFx {
        vibdepth_note: 1,
        arpt: 0b01000011,
        skydive: 0b10000001,
        arpt_counter: 2,
        notenum: 64,
        pw_minmax: 0xfc,
        resfilt: 0b00000000,
        fchi: 0b00000000,
    },
    InstrFx {
        vibdepth_note: 0,
        arpt: 0b00100001,
        skydive: 0b10000001,
        arpt_counter: 3,
        notenum: 51,
        pw_minmax: 0xf8,
        resfilt: 0b00000000,
        fchi: 0b00000000,
    },
    InstrFx {
        vibdepth_note: 0,
        arpt: 0b01000001,
        skydive: 0b00000000,
        arpt_counter: 240,
        notenum: 0,
        pw_minmax: 0xeb,
        resfilt: 0b00000000,
        fchi: 0b00000000,
    },
    InstrFx {
        vibdepth_note: 66,
        arpt: 0b10000001,
        skydive: 0b00000000,
        arpt_counter: 2,
        notenum: 0,
        pw_minmax: 0xeb,
        resfilt: 0b00000000,
        fchi: 0b00000000,
    },
    InstrFx {
        vibdepth_note: 67,
        arpt: 0b10000001,
        skydive: 0b00000000,
        arpt_counter: 2,
        notenum: 0,
        pw_minmax: 0xfb,
        resfilt: 0b00000000,
        fchi: 0b00000000,
    },
    InstrFx {
        vibdepth_note: 0,
        arpt: 0b01000001,
        skydive: 0b10000001,
        arpt_counter: 4,
        notenum: 61,
        pw_minmax: 0x30,
        resfilt: 0b00000000,
        fchi: 0b00000000,
    },
    InstrFx {
        vibdepth_note: 0,
        arpt: 0b10000001,
        skydive: 0b00010101,
        arpt_counter: 1,
        notenum: 64,
        pw_minmax: 0x00,
        resfilt: 0b00000000,
        fchi: 0b00000000,
    },
    InstrFx {
        vibdepth_note: 0,
        arpt: 0b01000011,
        skydive: 0b00010111,
        arpt_counter: 2,
        notenum: 79,
        pw_minmax: 0x00,
        resfilt: 0b11110010,
        fchi: 0b00001111,
    },
    InstrFx {
        vibdepth_note: 8,
        arpt: 0b00000100,
        skydive: 0b10000001,
        arpt_counter: 1,
        notenum: 44,
        pw_minmax: 0x00,
        resfilt: 0b11110010,
        fchi: 0b00000010,
    },
    InstrFx {
        vibdepth_note: 72,
        arpt: 0b00010111,
        skydive: 0b00000000,
        arpt_counter: 2,
        notenum: 0,
        pw_minmax: 0x2f,
        resfilt: 0b11110010,
        fchi: 0b00100000,
    },
];
