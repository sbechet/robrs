// Zoids - Rob Hubbard - 1986 Martech

use super::rhplayer::{ RhSongs, Instrument, MusicPlayer, SidT, SoundFx };
#[allow(dead_code)]
pub static RHSONGS: RhSongs = RhSongs {
    musicplayer: MusicPlayer::MontyOnTheRun,
    total: TOTAL_SONGS,
    tracks: &TRACKS,
    patterns: &PATTERNS,
    instruments: &INSTRUMENTS,
    soundfx: &SOUNDFX,
    resetspd: 1,
};


#[allow(dead_code)]
pub static TOTAL_SONGS: usize = 3;


#[allow(dead_code)]
pub static TRACK_0: [u8; 49] = [24, 24, 24, 24, 2, 4, 6, 7, 6, 8, 9, 6, 7, 6, 10, 6, 7, 6, 11, 16, 16, 20, 21, 20, 22, 23, 2, 4, 6, 7, 6, 8, 24, 24, 24, 24, 24, 24, 24, 24, 0, 0, 0, 0, 0, 0, 0, 0, 255];


#[allow(dead_code)]
pub static TRACK_1: [u8; 84] = [25, 25, 25, 25, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 14, 15, 3, 14, 3, 3, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 3, 14, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 25, 25, 25, 25, 25, 25, 25, 25, 0, 0, 0, 0, 0, 0, 0, 0, 255];


#[allow(dead_code)]
pub static TRACK_2: [u8; 85] = [26, 26, 26, 26, 1, 1, 1, 1, 1, 1, 1, 1, 5, 1, 5, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5, 1, 5, 1, 5, 1, 5, 12, 13, 17, 5, 12, 1, 1, 13, 13, 18, 13, 13, 13, 18, 13, 18, 13, 18, 13, 13, 18, 5, 12, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5, 1, 5, 1, 26, 26, 26, 26, 26, 26, 26, 26, 0, 0, 0, 0, 0, 0, 0, 0, 255];


#[allow(dead_code)]
pub static TRACK_3: [u8; 3] = [24, 27, 254];


#[allow(dead_code)]
pub static TRACK_4: [u8; 3] = [25, 28, 254];


#[allow(dead_code)]
pub static TRACK_5: [u8; 3] = [29, 29, 254];


#[allow(dead_code)]
pub static TRACK_6: [u8; 3] = [30, 27, 254];


#[allow(dead_code)]
pub static TRACK_7: [u8; 5] = [19, 19, 19, 28, 254];


#[allow(dead_code)]
pub static TRACK_8: [u8; 5] = [13, 18, 13, 29, 254];


#[allow(dead_code)]
pub static TRACKS: [ &[&[u8];3]; 3] = [
    &[&TRACK_0,&TRACK_1,&TRACK_2],
    &[&TRACK_3,&TRACK_4,&TRACK_5],
    &[&TRACK_6,&TRACK_7,&TRACK_8],
];


#[allow(dead_code)]
pub static PATTERNS: [&[u8]; 31] = [
&PATTERN_0,&PATTERN_1,&PATTERN_2,&PATTERN_3,&PATTERN_4,&PATTERN_5,&PATTERN_6,&PATTERN_7,&PATTERN_8,&PATTERN_9,&PATTERN_10,&PATTERN_11,&PATTERN_12,&PATTERN_13,&PATTERN_14,&PATTERN_15,&PATTERN_16,&PATTERN_17,&PATTERN_18,&PATTERN_19,&PATTERN_20,&PATTERN_21,&PATTERN_22,&PATTERN_23,&PATTERN_24,&PATTERN_25,&PATTERN_26,&PATTERN_27,&PATTERN_28,&PATTERN_29,&PATTERN_30,];
#[allow(dead_code)]
pub static PATTERN_0: [u8; 3] = [
    0x5F,
    0x5F,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_1: [u8; 21] = [
    0x87,
    0x2,
    0x13,
    0x7,
    0x13,
    0x87,
    0x3,
    0x30,
    0x87,
    0x2,
    0x1F,
    0x7,
    0x1F,
    0x7,
    0x13,
    0x87,
    0x3,
    0x30,
    0x7,
    0x2C,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_2: [u8; 42] = [
    0x8F,
    0x0,
    0x2F,
    0xB,
    0x2F,
    0x3,
    0x2D,
    0xF,
    0x2F,
    0xB,
    0x2F,
    0x3,
    0x2D,
    0xF,
    0x2F,
    0x1F,
    0x32,
    0x47,
    0x3,
    0x30,
    0x3,
    0x32,
    0x1F,
    0x33,
    0x1F,
    0x30,
    0x3,
    0x2F,
    0x3,
    0x30,
    0x1F,
    0x32,
    0x83,
    0xC,
    0x2F,
    0x3,
    0x2F,
    0x7,
    0x2C,
    0x7,
    0x2C,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_3: [u8; 26] = [
    0x8F,
    0x1,
    0x40,
    0x83,
    0x4,
    0x2B,
    0x3,
    0x2B,
    0x3,
    0x26,
    0x3,
    0x2B,
    0x8F,
    0x1,
    0x40,
    0x83,
    0x4,
    0x2B,
    0x3,
    0x2B,
    0x3,
    0x26,
    0x83,
    0x2,
    0x1F,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_4: [u8; 42] = [
    0x8F,
    0x0,
    0x2F,
    0xB,
    0x2F,
    0x3,
    0x2D,
    0xF,
    0x2F,
    0xB,
    0x2F,
    0x3,
    0x2D,
    0xF,
    0x2F,
    0x1F,
    0x32,
    0x47,
    0x3,
    0x30,
    0x3,
    0x32,
    0x1F,
    0x33,
    0x1F,
    0x37,
    0x1F,
    0x32,
    0x47,
    0x83,
    0xC,
    0x2F,
    0x3,
    0x2F,
    0x7,
    0x2C,
    0x83,
    0x0,
    0x30,
    0x3,
    0x32,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_5: [u8; 21] = [
    0x87,
    0x2,
    0x18,
    0x7,
    0x18,
    0x87,
    0x3,
    0x30,
    0x87,
    0x2,
    0x24,
    0x7,
    0x24,
    0x7,
    0x18,
    0x87,
    0x3,
    0x30,
    0x7,
    0x2C,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_6: [u8; 49] = [
    0x2,
    0x33,
    0x1,
    0x30,
    0x2,
    0x33,
    0x2,
    0x30,
    0x1,
    0x33,
    0x2,
    0x30,
    0x2,
    0x37,
    0x1,
    0x33,
    0x2,
    0x37,
    0x2,
    0x33,
    0x1,
    0x37,
    0x2,
    0x33,
    0x2,
    0x3C,
    0x1,
    0x37,
    0x2,
    0x3C,
    0x2,
    0x37,
    0x1,
    0x3C,
    0x2,
    0x37,
    0x2,
    0x3F,
    0x1,
    0x3C,
    0x2,
    0x3F,
    0x2,
    0x3C,
    0x1,
    0x3F,
    0x2,
    0x3C,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_7: [u8; 19] = [
    0xF,
    0x3C,
    0xF,
    0x3A,
    0x87,
    0xC,
    0x2F,
    0x7,
    0x2F,
    0x3,
    0x2C,
    0x3,
    0x2C,
    0x83,
    0x0,
    0x30,
    0x3,
    0x32,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_8: [u8; 14] = [
    0x1F,
    0x3E,
    0x87,
    0xC,
    0x2F,
    0x7,
    0x2F,
    0x3,
    0x2C,
    0x3,
    0x2C,
    0x7,
    0x2C,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_9: [u8; 74] = [
    0x8F,
    0x5,
    0x2F,
    0xB,
    0x2F,
    0x3,
    0x2D,
    0xF,
    0x2F,
    0xB,
    0x2F,
    0x3,
    0x2D,
    0xF,
    0x2F,
    0x1F,
    0x32,
    0x47,
    0x3,
    0x2B,
    0x3,
    0x2D,
    0x1F,
    0x2E,
    0x1F,
    0x31,
    0x1F,
    0x2F,
    0x83,
    0xC,
    0x2F,
    0x7,
    0x2F,
    0x3,
    0x2C,
    0x7,
    0x2C,
    0x7,
    0x2C,
    0x8F,
    0x5,
    0x2F,
    0xB,
    0x2F,
    0x3,
    0x2D,
    0xF,
    0x2F,
    0xB,
    0x2F,
    0x3,
    0x2D,
    0xF,
    0x2F,
    0x1F,
    0x32,
    0x47,
    0x3,
    0x2F,
    0x3,
    0x30,
    0x1F,
    0x31,
    0x1F,
    0x34,
    0x1F,
    0x32,
    0x17,
    0x35,
    0x3,
    0x30,
    0x3,
    0x32,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_10: [u8; 17] = [
    0x1F,
    0x3E,
    0x87,
    0xC,
    0x2F,
    0x7,
    0x2F,
    0x3,
    0x2C,
    0x3,
    0x2C,
    0x83,
    0x5,
    0x30,
    0x3,
    0x32,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_11: [u8; 144] = [
    0x9F,
    0x6,
    0x3F,
    0x7,
    0x3C,
    0x7,
    0x3E,
    0x21,
    0x3F,
    0xA5,
    0xC0,
    0x3F,
    0x7,
    0x41,
    0x37,
    0x3F,
    0xA7,
    0xA7,
    0x3F,
    0xF,
    0x3E,
    0x7,
    0x3F,
    0x7,
    0x41,
    0x1F,
    0x42,
    0x7,
    0x3F,
    0x7,
    0x41,
    0x21,
    0x42,
    0xA5,
    0xCA,
    0x42,
    0x7,
    0x44,
    0x37,
    0x42,
    0xA7,
    0xAB,
    0x42,
    0xF,
    0x41,
    0x7,
    0x42,
    0x7,
    0x44,
    0x27,
    0x47,
    0xA7,
    0xB7,
    0x47,
    0xF,
    0x46,
    0xF,
    0x44,
    0x27,
    0x46,
    0xA7,
    0xD1,
    0x46,
    0x37,
    0x44,
    0xA7,
    0xAB,
    0x44,
    0xF,
    0x43,
    0x7,
    0x44,
    0x7,
    0x46,
    0xA7,
    0xD0,
    0x46,
    0x17,
    0x48,
    0x83,
    0xC,
    0x2F,
    0x7,
    0x2F,
    0x3,
    0x2C,
    0x7,
    0x2C,
    0x87,
    0x6,
    0x48,
    0xA7,
    0xFE,
    0x48,
    0x17,
    0x4B,
    0x83,
    0xC,
    0x2F,
    0x7,
    0x2F,
    0x3,
    0x2C,
    0x7,
    0x2C,
    0x87,
    0x5,
    0x3F,
    0xF,
    0x3E,
    0x87,
    0xC,
    0x2F,
    0x7,
    0x2F,
    0x87,
    0x7,
    0x4F,
    0x7,
    0x43,
    0x87,
    0xC,
    0x2C,
    0x7,
    0x2C,
    0x8F,
    0x7,
    0x43,
    0x87,
    0xC,
    0x2F,
    0x7,
    0x2F,
    0x87,
    0x7,
    0x4F,
    0x7,
    0x43,
    0x87,
    0xC,
    0x2C,
    0x7,
    0x2C,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_12: [u8; 21] = [
    0x87,
    0x2,
    0x14,
    0x7,
    0x14,
    0x87,
    0x3,
    0x30,
    0x87,
    0x2,
    0x20,
    0x7,
    0x20,
    0x7,
    0x14,
    0x87,
    0x3,
    0x30,
    0x7,
    0x2C,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_13: [u8; 21] = [
    0x87,
    0x2,
    0x16,
    0x7,
    0x16,
    0x87,
    0x3,
    0x30,
    0x87,
    0x2,
    0x22,
    0x7,
    0x22,
    0x7,
    0x16,
    0x87,
    0x3,
    0x30,
    0x7,
    0x2C,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_14: [u8; 26] = [
    0x8F,
    0x1,
    0x40,
    0x83,
    0x4,
    0x2C,
    0x3,
    0x2C,
    0x3,
    0x27,
    0x3,
    0x2C,
    0x8F,
    0x1,
    0x40,
    0x83,
    0x4,
    0x2C,
    0x3,
    0x2C,
    0x3,
    0x27,
    0x83,
    0xC,
    0x2F,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_15: [u8; 126] = [
    0x8F,
    0x1,
    0x40,
    0x83,
    0x4,
    0x2E,
    0x3,
    0x2E,
    0x3,
    0x29,
    0x3,
    0x2E,
    0x8F,
    0x1,
    0x40,
    0x83,
    0x4,
    0x2E,
    0x3,
    0x2E,
    0x3,
    0x29,
    0x83,
    0xC,
    0x2F,
    0x8F,
    0x1,
    0x40,
    0x83,
    0x4,
    0x2F,
    0x3,
    0x2F,
    0x3,
    0x2A,
    0x3,
    0x2F,
    0x8F,
    0x1,
    0x40,
    0x83,
    0x4,
    0x2F,
    0x3,
    0x2F,
    0x3,
    0x2A,
    0x83,
    0xC,
    0x2F,
    0x8F,
    0x1,
    0x40,
    0x83,
    0x4,
    0x31,
    0x3,
    0x31,
    0x3,
    0x2C,
    0x3,
    0x31,
    0x8F,
    0x1,
    0x40,
    0x83,
    0x4,
    0x31,
    0x3,
    0x31,
    0x3,
    0x2C,
    0x83,
    0xC,
    0x2F,
    0x8F,
    0x1,
    0x40,
    0x83,
    0x4,
    0x34,
    0x3,
    0x34,
    0x3,
    0x2F,
    0x3,
    0x34,
    0x8F,
    0x1,
    0x40,
    0x83,
    0x4,
    0x34,
    0x3,
    0x34,
    0x3,
    0x2F,
    0x83,
    0xC,
    0x2F,
    0x8F,
    0x1,
    0x40,
    0x83,
    0x4,
    0x33,
    0x3,
    0x33,
    0x3,
    0x2E,
    0x3,
    0x33,
    0x8F,
    0x1,
    0x40,
    0x83,
    0x4,
    0x33,
    0x3,
    0x33,
    0x3,
    0x2E,
    0x83,
    0xC,
    0x2F,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_16: [u8; 101] = [
    0x8F,
    0x8,
    0x4A,
    0xB,
    0x4A,
    0x3,
    0x48,
    0x1,
    0x4A,
    0x1,
    0x4B,
    0x1,
    0x4A,
    0x1,
    0x48,
    0x7,
    0x4D,
    0xB,
    0x4A,
    0x3,
    0x48,
    0x8F,
    0x8,
    0x4A,
    0xB,
    0x4A,
    0x3,
    0x48,
    0x1,
    0x4A,
    0x1,
    0x4B,
    0x1,
    0x4A,
    0x1,
    0x48,
    0x7,
    0x4D,
    0x7,
    0x4A,
    0x3,
    0x4B,
    0x3,
    0x4D,
    0xF,
    0x4E,
    0x3,
    0x52,
    0x3,
    0x53,
    0x7,
    0x52,
    0x1,
    0x50,
    0x1,
    0x52,
    0x1,
    0x50,
    0x1,
    0x4E,
    0x1,
    0x4D,
    0x1,
    0x4E,
    0x1,
    0x4D,
    0x1,
    0x4B,
    0x1,
    0x4D,
    0x1,
    0x4E,
    0x1,
    0x4D,
    0x1,
    0x4B,
    0x1,
    0x4D,
    0x1,
    0x4E,
    0x1,
    0x4D,
    0x1,
    0x4B,
    0x3,
    0x4A,
    0x3,
    0x4B,
    0x17,
    0x4D,
    0x87,
    0x7,
    0x52,
    0x7,
    0x46,
    0x87,
    0xC,
    0x2F,
    0x7,
    0x2C,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_17: [u8; 81] = [
    0x87,
    0x2,
    0x17,
    0x7,
    0x17,
    0x87,
    0x3,
    0x30,
    0x87,
    0x2,
    0x23,
    0x7,
    0x23,
    0x7,
    0x17,
    0x87,
    0x3,
    0x30,
    0x7,
    0x2C,
    0x87,
    0x2,
    0x19,
    0x7,
    0x19,
    0x87,
    0x3,
    0x30,
    0x87,
    0x2,
    0x25,
    0x7,
    0x25,
    0x7,
    0x19,
    0x87,
    0x3,
    0x30,
    0x7,
    0x2C,
    0x87,
    0x2,
    0x1C,
    0x7,
    0x1C,
    0x87,
    0x3,
    0x30,
    0x87,
    0x2,
    0x28,
    0x7,
    0x28,
    0x7,
    0x1C,
    0x87,
    0x3,
    0x30,
    0x7,
    0x2C,
    0x87,
    0x2,
    0x1B,
    0x7,
    0x1B,
    0x87,
    0x3,
    0x30,
    0x87,
    0x2,
    0x27,
    0x7,
    0x27,
    0x7,
    0x1B,
    0x87,
    0x3,
    0x30,
    0x7,
    0x2C,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_18: [u8; 21] = [
    0x87,
    0x2,
    0x1B,
    0x7,
    0x1B,
    0x87,
    0x3,
    0x30,
    0x87,
    0x2,
    0x27,
    0x7,
    0x27,
    0x7,
    0x1B,
    0x87,
    0x3,
    0x30,
    0x7,
    0x2C,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_19: [u8; 29] = [
    0x87,
    0x7,
    0x3A,
    0x7,
    0x2E,
    0x83,
    0x9,
    0x52,
    0x3,
    0x46,
    0x3,
    0x3A,
    0x3,
    0x2E,
    0x87,
    0x7,
    0x3A,
    0x7,
    0x2E,
    0x83,
    0x9,
    0x2E,
    0x3,
    0x3A,
    0x3,
    0x46,
    0x3,
    0x52,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_20: [u8; 66] = [
    0x81,
    0x8,
    0x33,
    0x1,
    0x2A,
    0x1,
    0x2E,
    0x1,
    0x33,
    0x1,
    0x36,
    0x1,
    0x2E,
    0x1,
    0x33,
    0x1,
    0x36,
    0x1,
    0x3A,
    0x1,
    0x33,
    0x1,
    0x36,
    0x1,
    0x3A,
    0x1,
    0x3F,
    0x1,
    0x36,
    0x1,
    0x3A,
    0x1,
    0x3F,
    0x1,
    0x42,
    0x1,
    0x3A,
    0x1,
    0x3F,
    0x1,
    0x42,
    0x1,
    0x46,
    0x1,
    0x3F,
    0x1,
    0x42,
    0x1,
    0x46,
    0x1,
    0x4B,
    0x1,
    0x42,
    0x1,
    0x46,
    0x1,
    0x4B,
    0x1,
    0x4E,
    0x1,
    0x46,
    0x1,
    0x4B,
    0x1,
    0x4E,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_21: [u8; 15] = [
    0xF,
    0x4B,
    0xF,
    0x49,
    0x87,
    0x7,
    0x52,
    0x7,
    0x46,
    0x87,
    0xC,
    0x2F,
    0x7,
    0x2C,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_22: [u8; 16] = [
    0x1F,
    0x4D,
    0x87,
    0x7,
    0x52,
    0x7,
    0x46,
    0x87,
    0xC,
    0x2F,
    0x83,
    0xA,
    0x42,
    0x3,
    0x44,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_23: [u8; 95] = [
    0x27,
    0x47,
    0xA7,
    0xB7,
    0x47,
    0xF,
    0x46,
    0xF,
    0x44,
    0x27,
    0x46,
    0xA7,
    0xD1,
    0x46,
    0x37,
    0x44,
    0xA7,
    0xAB,
    0x44,
    0xF,
    0x43,
    0x7,
    0x44,
    0x7,
    0x46,
    0xA7,
    0xD0,
    0x46,
    0x17,
    0x48,
    0x83,
    0xC,
    0x2F,
    0x7,
    0x2F,
    0x3,
    0x2C,
    0x7,
    0x2C,
    0x87,
    0x6,
    0x48,
    0xA7,
    0xFE,
    0x48,
    0x17,
    0x4B,
    0x83,
    0xC,
    0x2F,
    0x7,
    0x2F,
    0x3,
    0x2C,
    0x7,
    0x2C,
    0x87,
    0x5,
    0x3F,
    0xF,
    0x3E,
    0x87,
    0xC,
    0x2F,
    0x7,
    0x2F,
    0x87,
    0x7,
    0x4F,
    0x7,
    0x43,
    0x87,
    0xC,
    0x2C,
    0x7,
    0x2C,
    0x8F,
    0x7,
    0x43,
    0x87,
    0xC,
    0x2F,
    0x7,
    0x2F,
    0x87,
    0x7,
    0x4F,
    0x7,
    0x43,
    0x87,
    0xC,
    0x2C,
    0x7,
    0x2C,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_24: [u8; 19] = [
    0x8F,
    0x7,
    0x43,
    0x87,
    0xC,
    0x2F,
    0x7,
    0x2F,
    0x87,
    0x7,
    0x4F,
    0x7,
    0x43,
    0x87,
    0xC,
    0x2C,
    0x7,
    0x2C,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_25: [u8; 15] = [
    0x8F,
    0x1,
    0x40,
    0x8F,
    0x3,
    0x30,
    0x8F,
    0x1,
    0x40,
    0x87,
    0x3,
    0x30,
    0x7,
    0x2C,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_26: [u8; 6] = [
    0xBF,
    0x2,
    0x13,
    0x1F,
    0x13,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_27: [u8; 6] = [
    0xBF,
    0x7,
    0x43,
    0x1F,
    0x43,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_28: [u8; 6] = [
    0xBF,
    0x3,
    0x30,
    0x1F,
    0x30,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_29: [u8; 6] = [
    0xBF,
    0x2,
    0x16,
    0x1F,
    0x16,
    0xFF,
];
#[allow(dead_code)]
pub static PATTERN_30: [u8; 80] = [
    0x8F,
    0x8,
    0x4A,
    0xB,
    0x4A,
    0x3,
    0x48,
    0x1,
    0x4A,
    0x1,
    0x4B,
    0x1,
    0x4A,
    0x1,
    0x48,
    0x7,
    0x4D,
    0x7,
    0x4A,
    0x3,
    0x4B,
    0x3,
    0x4D,
    0xF,
    0x4E,
    0x3,
    0x52,
    0x3,
    0x53,
    0x7,
    0x52,
    0x1,
    0x50,
    0x1,
    0x52,
    0x1,
    0x50,
    0x1,
    0x4E,
    0x1,
    0x4D,
    0x1,
    0x4E,
    0x1,
    0x4D,
    0x1,
    0x4B,
    0x1,
    0x4D,
    0x1,
    0x4E,
    0x1,
    0x4D,
    0x1,
    0x4B,
    0x1,
    0x4D,
    0x1,
    0x4E,
    0x1,
    0x4D,
    0x1,
    0x4B,
    0x3,
    0x4A,
    0x3,
    0x4B,
    0x17,
    0x4D,
    0x87,
    0x7,
    0x52,
    0x7,
    0x46,
    0x87,
    0xC,
    0x2F,
    0x7,
    0x2C,
    0xFF,
];


#[allow(dead_code)]
pub static INSTRUMENTS: [ Instrument; 15] = [
    Instrument { pulse_width:2304, ctrl_register:0b01000001, attack_and_decay:0x59, sustain_and_release:0xAF, vibrato_depth:2, pulse_speed:224, fx:0b00000000 },
    Instrument { pulse_width:2176, ctrl_register:0b10000001, attack_and_decay:0x09, sustain_and_release:0x0A, vibrato_depth:1, pulse_speed:0, fx:0b00000101 },
    Instrument { pulse_width:415, ctrl_register:0b01000001, attack_and_decay:0x0C, sustain_and_release:0xCD, vibrato_depth:0, pulse_speed:1, fx:0b00001101 },
    Instrument { pulse_width:512, ctrl_register:0b10000001, attack_and_decay:0x0B, sustain_and_release:0x0C, vibrato_depth:0, pulse_speed:0, fx:0b00000101 },
    Instrument { pulse_width:640, ctrl_register:0b01000001, attack_and_decay:0x07, sustain_and_release:0x0A, vibrato_depth:0, pulse_speed:0, fx:0b00000101 },
    Instrument { pulse_width:512, ctrl_register:0b01000001, attack_and_decay:0x0C, sustain_and_release:0xAF, vibrato_depth:0, pulse_speed:120, fx:0b00001100 },
    Instrument { pulse_width:2176, ctrl_register:0b01000001, attack_and_decay:0x0D, sustain_and_release:0x9F, vibrato_depth:2, pulse_speed:153, fx:0b00001000 },
    Instrument { pulse_width:384, ctrl_register:0b00010101, attack_and_decay:0x0A, sustain_and_release:0x0C, vibrato_depth:1, pulse_speed:0, fx:0b00000101 },
    Instrument { pulse_width:2048, ctrl_register:0b01000001, attack_and_decay:0x6C, sustain_and_release:0x0A, vibrato_depth:1, pulse_speed:0, fx:0b00000010 },
    Instrument { pulse_width:2304, ctrl_register:0b00010001, attack_and_decay:0x0F, sustain_and_release:0xFF, vibrato_depth:0, pulse_speed:0, fx:0b00000100 },
    Instrument { pulse_width:2048, ctrl_register:0b01000001, attack_and_decay:0x6C, sustain_and_release:0x70, vibrato_depth:1, pulse_speed:0, fx:0b00000010 },
    Instrument { pulse_width:768, ctrl_register:0b00010001, attack_and_decay:0x0F, sustain_and_release:0xFF, vibrato_depth:2, pulse_speed:0, fx:0b00000100 },
    Instrument { pulse_width:2048, ctrl_register:0b01000001, attack_and_decay:0x0A, sustain_and_release:0x0A, vibrato_depth:0, pulse_speed:0, fx:0b00000001 },
    Instrument { pulse_width:512, ctrl_register:0b01000001, attack_and_decay:0x39, sustain_and_release:0x8F, vibrato_depth:2, pulse_speed:4, fx:0b00001000 },
    Instrument { pulse_width:2048, ctrl_register:0b01000001, attack_and_decay:0x09, sustain_and_release:0x8F, vibrato_depth:2, pulse_speed:224, fx:0b00000000 },
];


#[allow(dead_code)]
pub static SOUNDFX: [ SoundFx; 0] = [];