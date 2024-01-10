// The Last V8 - Rob Hubbard - 1985 MAD/Mastertronic

use crate::rhplayer::rhsongs::{InstrFx, Instrument, RhSongs, SidT, SoundFx};
#[allow(dead_code)]
pub static RHSONGS: RhSongs = RhSongs {
    version: 10,
    total: 15,
    channels: &CHANNELS,
    tracks: &TRACKS,
    instruments: &INSTRUMENTS,
    soundfx: &SOUNDFX,
    instrfx: &INSTRFX,
    resetspd: 1,
    skydive_v1_when: 0,
    skydive_v1_add: -256,
};

#[allow(dead_code)]
pub static CHANNEL_0: [u8; 38] = [
    0, 0, 0, 0, 7, 7, 7, 7, 8, 8, 8, 8, 0, 0, 0, 0, 0, 0, 0, 0, 4, 4, 10, 4, 11, 4, 15, 15, 16, 16,
    4, 4, 5, 5, 4, 4, 18, 255,
];

#[allow(dead_code)]
pub static CHANNEL_1: [u8; 55] = [
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 17, 17, 17, 17,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 255,
];

#[allow(dead_code)]
pub static CHANNEL_2: [u8; 88] = [
    0, 0, 0, 0, 6, 9, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 12, 12, 12, 12, 13, 13, 13, 13, 14, 14, 14, 14, 3, 3, 3, 3, 1, 1, 1, 1, 1, 1,
    1, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 1, 1, 1, 1, 1, 1, 1, 1, 19, 255,
];

#[allow(dead_code)]
pub static CHANNEL_3: [u8; 2] = [23, 254];

#[allow(dead_code)]
pub static CHANNEL_4: [u8; 6] = [20, 20, 20, 20, 21, 254];

#[allow(dead_code)]
pub static CHANNEL_5: [u8; 2] = [22, 254];

#[allow(dead_code)]
pub static CHANNEL_6: [u8; 2] = [25, 254];

#[allow(dead_code)]
pub static CHANNEL_7: [u8; 8] = [27, 27, 27, 27, 27, 27, 26, 254];

#[allow(dead_code)]
pub static CHANNEL_8: [u8; 2] = [24, 254];

#[allow(dead_code)]
pub static CHANNELS: [&[&[u8]; 3]; 3] = [
    &[&CHANNEL_0, &CHANNEL_1, &CHANNEL_2],
    &[&CHANNEL_3, &CHANNEL_4, &CHANNEL_5],
    &[&CHANNEL_6, &CHANNEL_7, &CHANNEL_8],
];

#[allow(dead_code)]
pub static TRACKS: [&[u8]; 29] = [
    &TRACK_0, &TRACK_1, &TRACK_2, &TRACK_3, &TRACK_4, &TRACK_5, &TRACK_6, &TRACK_7, &TRACK_8,
    &TRACK_9, &TRACK_10, &TRACK_11, &TRACK_12, &TRACK_13, &TRACK_14, &TRACK_15, &TRACK_16,
    &TRACK_17, &TRACK_18, &TRACK_19, &TRACK_20, &TRACK_21, &TRACK_22, &TRACK_23, &TRACK_24,
    &TRACK_25, &TRACK_26, &TRACK_27, &TRACK_28,
];
#[allow(dead_code)]
pub static TRACK_0: [u8; 2] = [0x5F, 0xFF];
#[allow(dead_code)]
pub static TRACK_1: [u8; 22] = [
    0x83, 0x2, 0x15, 0x3, 0x21, 0x83, 0x3, 0x2F, 0x83, 0x2, 0x15, 0x3, 0x21, 0x3, 0x2D, 0x83, 0x3,
    0x2F, 0x83, 0x2, 0x21, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_2: [u8; 45] = [
    0x83, 0x0, 0x45, 0x83, 0x1, 0x47, 0x3, 0x4C, 0x83, 0x0, 0x40, 0x83, 0x1, 0x4C, 0x3, 0x47, 0x83,
    0x0, 0x39, 0x83, 0x1, 0x4C, 0x83, 0x0, 0x51, 0x83, 0x1, 0x47, 0x83, 0x0, 0x45, 0x83, 0x1, 0x4C,
    0x83, 0x0, 0x39, 0x83, 0x1, 0x47, 0x3, 0x40, 0x3, 0x47, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_3: [u8; 22] = [
    0x83, 0x2, 0x1C, 0x3, 0x28, 0x83, 0x3, 0x2F, 0x83, 0x2, 0x1C, 0x3, 0x28, 0x3, 0x34, 0x83, 0x3,
    0x2F, 0x83, 0x2, 0x28, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_4: [u8; 46] = [
    0x83, 0x6, 0x45, 0x3, 0x45, 0x7, 0x48, 0x3, 0x47, 0x7, 0x43, 0x7, 0x45, 0xF, 0x40, 0x3, 0x40,
    0x3, 0x43, 0x3, 0x44, 0x83, 0x6, 0x45, 0x3, 0x45, 0x7, 0x48, 0x3, 0x47, 0x7, 0x43, 0x3, 0x45,
    0x43, 0x87, 0x9, 0x32, 0x7, 0x32, 0x3, 0x2F, 0x87, 0xA, 0x38, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_5: [u8; 89] = [
    0x83, 0x4, 0x3E, 0x3, 0x3E, 0x7, 0x3B, 0x7, 0x3E, 0x3, 0x3B, 0x7, 0x3E, 0x3, 0x3E, 0x7, 0x3B,
    0xA3, 0xB0, 0x3E, 0xB, 0x40, 0x83, 0x4, 0x3E, 0x3, 0x3E, 0x7, 0x3B, 0x7, 0x3E, 0x3, 0x3B, 0x7,
    0x3E, 0x3, 0x3E, 0x7, 0x3B, 0xA3, 0xA7, 0x3A, 0xB, 0x39, 0x83, 0x4, 0x3E, 0x3, 0x3E, 0x7, 0x3B,
    0x7, 0x3E, 0x3, 0x3B, 0x7, 0x3E, 0x3, 0x3E, 0x7, 0x3B, 0xA3, 0xB0, 0x3E, 0xB, 0x40, 0x83, 0x4,
    0x3E, 0x3, 0x3E, 0x7, 0x3B, 0x7, 0x3E, 0x3, 0x3B, 0x7, 0x3E, 0x3, 0x3E, 0x7, 0x40, 0xA3, 0xC0,
    0x43, 0xB, 0x44, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_6: [u8; 42] = [
    0xBF, 0x8, 0x15, 0xB, 0x15, 0x3, 0x17, 0x7, 0x18, 0x7, 0x17, 0x3F, 0x13, 0xB, 0x13, 0x3, 0x17,
    0x7, 0x18, 0x7, 0x17, 0x3F, 0x12, 0xB, 0x12, 0x3, 0x17, 0x7, 0x18, 0x7, 0x17, 0x3F, 0x11, 0xB,
    0x11, 0x3, 0x17, 0x7, 0x18, 0x7, 0x17, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_7: [u8; 11] = [
    0x4F, 0xA3, 0x7, 0x40, 0x7, 0x45, 0x3, 0x4D, 0x3F, 0x4C, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_8: [u8; 11] = [
    0x4F, 0xA3, 0x7, 0x45, 0x7, 0x4C, 0x3, 0x54, 0x3F, 0x53, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_9: [u8; 41] = [
    0x3F, 0x1A, 0xB, 0x1A, 0x3, 0x1C, 0x7, 0x1D, 0x7, 0x1C, 0x3F, 0x18, 0xB, 0x18, 0x3, 0x1C, 0x7,
    0x1D, 0x7, 0x1C, 0x3F, 0x17, 0xB, 0x17, 0x3, 0x1C, 0x7, 0x1D, 0x7, 0x1C, 0x3F, 0x10, 0xB, 0x10,
    0x3, 0x1C, 0x7, 0x1D, 0x7, 0x1C, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_10: [u8; 42] = [
    0x8B, 0x5, 0x34, 0x3, 0x32, 0x3, 0x34, 0x3, 0x34, 0x3, 0x32, 0x7, 0x37, 0x3, 0x37, 0x7, 0x36,
    0x3, 0x32, 0x7, 0x34, 0x3, 0x32, 0x3, 0x34, 0x3, 0x34, 0x7, 0x34, 0x3, 0x34, 0x7, 0x30, 0x3,
    0x2D, 0x83, 0xB, 0x0, 0x9B, 0xF0, 0x0, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_11: [u8; 42] = [
    0x8B, 0x5, 0x34, 0x3, 0x32, 0x3, 0x34, 0x3, 0x34, 0x3, 0x32, 0x7, 0x37, 0x3, 0x37, 0x7, 0x36,
    0x3, 0x32, 0x7, 0x34, 0x3, 0x32, 0x3, 0x34, 0x3, 0x34, 0x7, 0x39, 0x3, 0x45, 0x7, 0x40, 0x3,
    0x39, 0x83, 0xB, 0x0, 0x9B, 0xF1, 0x30, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_12: [u8; 22] = [
    0x83, 0x2, 0x1A, 0x3, 0x26, 0x83, 0x3, 0x2F, 0x83, 0x2, 0x1A, 0x3, 0x26, 0x3, 0x32, 0x83, 0x3,
    0x2F, 0x83, 0x2, 0x26, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_13: [u8; 22] = [
    0x83, 0x2, 0x13, 0x3, 0x1F, 0x83, 0x3, 0x2F, 0x83, 0x2, 0x13, 0x3, 0x1F, 0x3, 0x2B, 0x83, 0x3,
    0x2F, 0x83, 0x2, 0x1F, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_14: [u8; 22] = [
    0x83, 0x2, 0x17, 0x3, 0x23, 0x83, 0x3, 0x2F, 0x83, 0x2, 0x17, 0x3, 0x23, 0x3, 0x2F, 0x83, 0x3,
    0x2F, 0x83, 0x2, 0x23, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_15: [u8; 39] = [
    0x83, 0x5, 0x32, 0x3, 0x32, 0x7, 0x35, 0x7, 0x3C, 0x3, 0x37, 0x7, 0x3B, 0x7, 0x3B, 0x3, 0x37,
    0x3, 0x3B, 0x3, 0x37, 0x7, 0x32, 0x3, 0x32, 0x3, 0x32, 0x7, 0x35, 0x7, 0x3C, 0x3, 0x37, 0x3,
    0x3B, 0x9F, 0xF, 0x4A, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_16: [u8; 49] = [
    0x83, 0x5, 0x3B, 0x3, 0x3B, 0x3, 0x3E, 0x3, 0x3B, 0x3, 0x45, 0x3, 0x3E, 0x3, 0x3B, 0x7, 0x3B,
    0x3, 0x3B, 0x3, 0x3E, 0x3, 0x3B, 0x3, 0x44, 0x3, 0x40, 0x3, 0x3B, 0x7, 0x3B, 0x3, 0x3B, 0x3,
    0x3E, 0x3, 0x3B, 0x3, 0x45, 0x3, 0x40, 0x3, 0x3B, 0x3, 0x44, 0x9F, 0xF, 0x4C, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_17: [u8; 45] = [
    0x83, 0x0, 0x45, 0x83, 0x1, 0x45, 0x3, 0x4A, 0x83, 0x0, 0x40, 0x83, 0x1, 0x4A, 0x3, 0x45, 0x83,
    0x0, 0x39, 0x83, 0x1, 0x4A, 0x83, 0x0, 0x51, 0x83, 0x1, 0x45, 0x83, 0x0, 0x45, 0x83, 0x1, 0x4A,
    0x83, 0x0, 0x39, 0x83, 0x1, 0x45, 0x3, 0x3E, 0x3, 0x45, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_18: [u8; 11] = [
    0xBF, 0xD, 0x7C, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_19: [u8; 11] = [
    0xBF, 0xE, 0x21, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_20: [u8; 34] = [
    0x82, 0x10, 0x30, 0x2, 0x32, 0x2, 0x37, 0x2, 0x3C, 0x2, 0x3E, 0x2, 0x43, 0x2, 0x48, 0x2, 0x4A,
    0x2, 0x4F, 0x2, 0x4A, 0x2, 0x48, 0x2, 0x43, 0x2, 0x3E, 0x2, 0x3C, 0x2, 0x37, 0x2, 0x34, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_21: [u8; 11] = [
    0xBF, 0x4, 0x39, 0xBF, 0xFE, 0x39, 0x3F, 0x3C, 0x1F, 0x3C, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_22: [u8; 45] = [
    0x85, 0x9, 0x32, 0xB, 0x32, 0xB, 0x2F, 0x5, 0x2F, 0xB, 0x2C, 0xAB, 0x8, 0x1F, 0x25, 0x22, 0xA5,
    0x85, 0x22, 0x25, 0x21, 0x2B, 0x1D, 0x2B, 0x1F, 0x2B, 0x1B, 0x5, 0x1A, 0x37, 0x18, 0x37, 0x13,
    0x37, 0x13, 0x3F, 0x13, 0xBF, 0x88, 0x13, 0x3F, 0x18, 0x1F, 0x18, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_23: [u8; 34] = [
    0x57, 0x57, 0xB7, 0xA, 0x32, 0x37, 0x32, 0x37, 0x32, 0x57, 0x8B, 0x4, 0x3C, 0x25, 0x3F, 0xA5,
    0x97, 0x3F, 0x5, 0x3E, 0xB, 0x3A, 0x5, 0x37, 0x3F, 0x3C, 0xBF, 0xFE, 0x3C, 0x3F, 0x3F, 0x1F,
    0x3F, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_24: [u8; 31] = [
    0x8B, 0x8, 0x1C, 0x25, 0x1F, 0xA5, 0x85, 0x1F, 0x25, 0x1E, 0x2B, 0x1A, 0x25, 0x17, 0x37, 0x1C,
    0x37, 0x1C, 0x37, 0x21, 0x37, 0x21, 0x37, 0x1C, 0x37, 0x1C, 0x37, 0x1C, 0x17, 0x21, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_25: [u8; 72] = [
    0x57, 0x57, 0x85, 0x12, 0x39, 0x5, 0x39, 0xB, 0x3C, 0x5, 0x3B, 0xB, 0x37, 0xB, 0x39, 0x17,
    0x32, 0x5, 0x37, 0x5, 0x34, 0x5, 0x33, 0x5, 0x32, 0x2, 0x33, 0x2, 0x34, 0x2, 0x35, 0x2, 0x36,
    0x2, 0x37, 0x2, 0x38, 0x2, 0x39, 0x2, 0x3A, 0x2, 0x3B, 0x2, 0x3C, 0x2, 0x3D, 0x2, 0x3E, 0x2,
    0x3F, 0x2, 0x40, 0x2, 0x41, 0x2, 0x42, 0x2, 0x43, 0x2, 0x44, 0x2, 0x45, 0x2, 0x46, 0x2, 0x47,
    0x2, 0x48, 0x17, 0x49, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_26: [u8; 51] = [
    0x85, 0x4, 0x23, 0x2, 0x24, 0x2, 0x25, 0x2, 0x26, 0x2, 0x27, 0x2, 0x28, 0x2, 0x29, 0x2, 0x2A,
    0x2, 0x2B, 0x2, 0x2C, 0x2, 0x2D, 0x2, 0x2E, 0x2, 0x2F, 0x2, 0x30, 0x2, 0x31, 0x2, 0x32, 0x2,
    0x33, 0x2, 0x34, 0x2, 0x35, 0x2, 0x36, 0x2, 0x37, 0x2, 0x38, 0x2, 0x39, 0x97, 0x4, 0x39, 0xFF,
];
#[allow(dead_code)]
pub static TRACK_27: [u8; 18] = [
    0x82, 0x11, 0x39, 0x2, 0x40, 0x2, 0x45, 0x2, 0x4C, 0x2, 0x51, 0x2, 0x4C, 0x2, 0x45, 0x2, 0x40,
    0xFF,
];
#[allow(dead_code)]
pub static TRACK_28: [u8; 45] = [
    0x0, 0xA0, 0x0, 0xA, 0x8D, 0x17, 0x85, 0xA, 0x18, 0x6D, 0x17, 0x85, 0xAA, 0xBD, 0x97, 0x87,
    0x99, 0x91, 0x87, 0xE8, 0xC8, 0xC0, 0x6, 0xD0, 0xF4, 0xA9, 0x40, 0x8D, 0x29, 0x85, 0x60, 0xA9,
    0xC0, 0x8D, 0x29, 0x85, 0x60, 0xA9, 0x0, 0x8D, 0x36, 0x85, 0x60, 0xA9, 0xFF,
];

#[allow(dead_code)]
pub static INSTRUMENTS: [Instrument; 19] = [
    Instrument {
        pulse_width: 2048,
        ctrl_register: 0b00010001,
        attack_and_decay: 0x04,
        sustain_and_release: 0x0F,
        vibrato_depth: 2,
        pulse_speed: 0x00,
        fx: 0b00000001,
    },
    Instrument {
        pulse_width: 2464,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x04,
        sustain_and_release: 0x09,
        vibrato_depth: 0,
        pulse_speed: 0x79,
        fx: 0b00000101,
    },
    Instrument {
        pulse_width: 352,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x09,
        sustain_and_release: 0x70,
        vibrato_depth: 0,
        pulse_speed: 0x08,
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
        pulse_width: 768,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x09,
        sustain_and_release: 0x90,
        vibrato_depth: 2,
        pulse_speed: 0x00,
        fx: 0b00000000,
    },
    Instrument {
        pulse_width: 340,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x09,
        sustain_and_release: 0x00,
        vibrato_depth: 2,
        pulse_speed: 0x08,
        fx: 0b00001000,
    },
    Instrument {
        pulse_width: 2304,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x07,
        sustain_and_release: 0x00,
        vibrato_depth: 2,
        pulse_speed: 0x41,
        fx: 0b00000100,
    },
    Instrument {
        pulse_width: 2880,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x1B,
        sustain_and_release: 0x6F,
        vibrato_depth: 1,
        pulse_speed: 0xE1,
        fx: 0b00000000,
    },
    Instrument {
        pulse_width: 3328,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x2F,
        sustain_and_release: 0xFF,
        vibrato_depth: 0,
        pulse_speed: 0x81,
        fx: 0b00000000,
    },
    Instrument {
        pulse_width: 2048,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x08,
        sustain_and_release: 0x0A,
        vibrato_depth: 0,
        pulse_speed: 0x00,
        fx: 0b00000001,
    },
    Instrument {
        pulse_width: 2048,
        ctrl_register: 0b10000001,
        attack_and_decay: 0x0F,
        sustain_and_release: 0xFF,
        vibrato_depth: 0,
        pulse_speed: 0x00,
        fx: 0b00000001,
    },
    Instrument {
        pulse_width: 2048,
        ctrl_register: 0b10000001,
        attack_and_decay: 0x0F,
        sustain_and_release: 0xF8,
        vibrato_depth: 0,
        pulse_speed: 0x00,
        fx: 0b00000000,
    },
    Instrument {
        pulse_width: 2048,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x05,
        sustain_and_release: 0x0A,
        vibrato_depth: 0,
        pulse_speed: 0x00,
        fx: 0b00000001,
    },
    Instrument {
        pulse_width: 256,
        ctrl_register: 0b00010111,
        attack_and_decay: 0x0F,
        sustain_and_release: 0x0F,
        vibrato_depth: 1,
        pulse_speed: 0x00,
        fx: 0b00000010,
    },
    Instrument {
        pulse_width: 384,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x0F,
        sustain_and_release: 0x0F,
        vibrato_depth: 2,
        pulse_speed: 0x00,
        fx: 0b00000100,
    },
    Instrument {
        pulse_width: 0,
        ctrl_register: 0b00010001,
        attack_and_decay: 0x0F,
        sustain_and_release: 0xF0,
        vibrato_depth: 2,
        pulse_speed: 0x00,
        fx: 0b00000010,
    },
    Instrument {
        pulse_width: 0,
        ctrl_register: 0b00010101,
        attack_and_decay: 0x0F,
        sustain_and_release: 0x0F,
        vibrato_depth: 0,
        pulse_speed: 0x00,
        fx: 0b00000001,
    },
    Instrument {
        pulse_width: 2048,
        ctrl_register: 0b01000011,
        attack_and_decay: 0x06,
        sustain_and_release: 0x08,
        vibrato_depth: 0,
        pulse_speed: 0x00,
        fx: 0b00000001,
    },
    Instrument {
        pulse_width: 2048,
        ctrl_register: 0b01000001,
        attack_and_decay: 0x05,
        sustain_and_release: 0x89,
        vibrato_depth: 2,
        pulse_speed: 0x00,
        fx: 0b00000101,
    },
];

#[allow(dead_code)]
pub static SOUNDFX: [SoundFx; 12] = [
    SoundFx {
        incdec: 0b01001000,
        voice0: SidT {
            freq: 0x0123, // REAL: lower part is used as start note.
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
        sfx_note_dest: 0x00, // REAL: end note
    },
    SoundFx {
        incdec: 0b10100011,
        voice0: SidT {
            freq: 0x1455, // REAL: lower part is used as start note.
            pulse_width: 2048,
            ctrl: 0b10000001,
            attack_and_decay_len: 0x0B,
            sustain_vol_and_release_len: 0x00,
        },
        voice1: SidT {
            freq: 0x0804, // REAL: value & 0b0011_1111 = note_delta ; sometime if value&0b1_0000000: flip_flop voice0 ctrl ; if value&0b0_1_000000: flip_flop voice1 ctrl
            pulse_width: 2048,
            ctrl: 0b00010101,
            attack_and_decay_len: 0x0B,
            sustain_vol_and_release_len: 0x00,
        },
        sfx_note_dest: 0x67, // REAL: end note
    },
    SoundFx {
        incdec: 0b01010011,
        voice0: SidT {
            freq: 0x1055, // REAL: lower part is used as start note.
            pulse_width: 0,
            ctrl: 0b10000001,
            attack_and_decay_len: 0x0C,
            sustain_vol_and_release_len: 0x00,
        },
        voice1: SidT {
            freq: 0x0804, // REAL: value & 0b0011_1111 = note_delta ; sometime if value&0b1_0000000: flip_flop voice0 ctrl ; if value&0b0_1_000000: flip_flop voice1 ctrl
            pulse_width: 2048,
            ctrl: 0b10000001,
            attack_and_decay_len: 0x0C,
            sustain_vol_and_release_len: 0x00,
        },
        sfx_note_dest: 0x2B, // REAL: end note
    },
    SoundFx {
        incdec: 0b01100110,
        voice0: SidT {
            freq: 0x1B14, // REAL: lower part is used as start note.
            pulse_width: 512,
            ctrl: 0b10000001,
            attack_and_decay_len: 0x0D,
            sustain_vol_and_release_len: 0x00,
        },
        voice1: SidT {
            freq: 0x0000, // REAL: value & 0b0011_1111 = note_delta ; sometime if value&0b1_0000000: flip_flop voice0 ctrl ; if value&0b0_1_000000: flip_flop voice1 ctrl
            pulse_width: 0,
            ctrl: 0b00010101,
            attack_and_decay_len: 0x0F,
            sustain_vol_and_release_len: 0x00,
        },
        sfx_note_dest: 0x3A, // REAL: end note
    },
    SoundFx {
        incdec: 0b01010100,
        voice0: SidT {
            freq: 0x0B48, // REAL: lower part is used as start note.
            pulse_width: 512,
            ctrl: 0b01010001,
            attack_and_decay_len: 0x0F,
            sustain_vol_and_release_len: 0x00,
        },
        voice1: SidT {
            freq: 0x0000, // REAL: value & 0b0011_1111 = note_delta ; sometime if value&0b1_0000000: flip_flop voice0 ctrl ; if value&0b0_1_000000: flip_flop voice1 ctrl
            pulse_width: 2048,
            ctrl: 0b01000011,
            attack_and_decay_len: 0x0F,
            sustain_vol_and_release_len: 0x00,
        },
        sfx_note_dest: 0x0C, // REAL: end note
    },
    SoundFx {
        incdec: 0b10101001,
        voice0: SidT {
            freq: 0x0033, // REAL: lower part is used as start note.
            pulse_width: 2176,
            ctrl: 0b01010001,
            attack_and_decay_len: 0x0F,
            sustain_vol_and_release_len: 0xF0,
        },
        voice1: SidT {
            freq: 0x0100, // REAL: value & 0b0011_1111 = note_delta ; sometime if value&0b1_0000000: flip_flop voice0 ctrl ; if value&0b0_1_000000: flip_flop voice1 ctrl
            pulse_width: 0,
            ctrl: 0b01010111,
            attack_and_decay_len: 0x0F,
            sustain_vol_and_release_len: 0xF0,
        },
        sfx_note_dest: 0x5F, // REAL: end note
    },
    SoundFx {
        incdec: 0b10100110,
        voice0: SidT {
            freq: 0x300A, // REAL: lower part is used as start note.
            pulse_width: 1536,
            ctrl: 0b01000001,
            attack_and_decay_len: 0x58,
            sustain_vol_and_release_len: 0x00,
        },
        voice1: SidT {
            freq: 0x27C0, // REAL: value & 0b0011_1111 = note_delta ; sometime if value&0b1_0000000: flip_flop voice0 ctrl ; if value&0b0_1_000000: flip_flop voice1 ctrl
            pulse_width: 1088,
            ctrl: 0b01000001,
            attack_and_decay_len: 0x78,
            sustain_vol_and_release_len: 0x00,
        },
        sfx_note_dest: 0x30, // REAL: end note
    },
    SoundFx {
        incdec: 0b00010111,
        voice0: SidT {
            freq: 0x0020, // REAL: lower part is used as start note.
            pulse_width: 0,
            ctrl: 0b01010001,
            attack_and_decay_len: 0x0F,
            sustain_vol_and_release_len: 0x90,
        },
        voice1: SidT {
            freq: 0x2708, // REAL: value & 0b0011_1111 = note_delta ; sometime if value&0b1_0000000: flip_flop voice0 ctrl ; if value&0b0_1_000000: flip_flop voice1 ctrl
            pulse_width: 0,
            ctrl: 0b01010001,
            attack_and_decay_len: 0x0F,
            sustain_vol_and_release_len: 0x90,
        },
        sfx_note_dest: 0x00, // REAL: end note
    },
    SoundFx {
        incdec: 0b01010001,
        voice0: SidT {
            freq: 0x0D50, // REAL: lower part is used as start note.
            pulse_width: 0,
            ctrl: 0b01010001,
            attack_and_decay_len: 0x0C,
            sustain_vol_and_release_len: 0x00,
        },
        voice1: SidT {
            freq: 0x0002, // REAL: value & 0b0011_1111 = note_delta ; sometime if value&0b1_0000000: flip_flop voice0 ctrl ; if value&0b0_1_000000: flip_flop voice1 ctrl
            pulse_width: 1024,
            ctrl: 0b01000011,
            attack_and_decay_len: 0x0D,
            sustain_vol_and_release_len: 0x00,
        },
        sfx_note_dest: 0x00, // REAL: end note
    },
    SoundFx {
        incdec: 0b01100001,
        voice0: SidT {
            freq: 0x0D16, // REAL: lower part is used as start note.
            pulse_width: 0,
            ctrl: 0b01010001,
            attack_and_decay_len: 0x0C,
            sustain_vol_and_release_len: 0x00,
        },
        voice1: SidT {
            freq: 0x0002, // REAL: value & 0b0011_1111 = note_delta ; sometime if value&0b1_0000000: flip_flop voice0 ctrl ; if value&0b0_1_000000: flip_flop voice1 ctrl
            pulse_width: 1024,
            ctrl: 0b01000011,
            attack_and_decay_len: 0x0F,
            sustain_vol_and_release_len: 0x00,
        },
        sfx_note_dest: 0x5F, // REAL: end note
    },
    SoundFx {
        incdec: 0b01010100,
        voice0: SidT {
            freq: 0x1638, // REAL: lower part is used as start note.
            pulse_width: 128,
            ctrl: 0b01010001,
            attack_and_decay_len: 0x0D,
            sustain_vol_and_release_len: 0x00,
        },
        voice1: SidT {
            freq: 0x0000, // REAL: value & 0b0011_1111 = note_delta ; sometime if value&0b1_0000000: flip_flop voice0 ctrl ; if value&0b0_1_000000: flip_flop voice1 ctrl
            pulse_width: 1024,
            ctrl: 0b00010101,
            attack_and_decay_len: 0x8F,
            sustain_vol_and_release_len: 0x00,
        },
        sfx_note_dest: 0x00, // REAL: end note
    },
    SoundFx {
        incdec: 0b00100011,
        voice0: SidT {
            freq: 0xF92A, // REAL: lower part is used as start note.
            pulse_width: 128,
            ctrl: 0b10000001,
            attack_and_decay_len: 0x97,
            sustain_vol_and_release_len: 0x00,
        },
        voice1: SidT {
            freq: 0x0D28, // REAL: value & 0b0011_1111 = note_delta ; sometime if value&0b1_0000000: flip_flop voice0 ctrl ; if value&0b0_1_000000: flip_flop voice1 ctrl
            pulse_width: 1024,
            ctrl: 0b10000001,
            attack_and_decay_len: 0xAD,
            sustain_vol_and_release_len: 0x40,
        },
        sfx_note_dest: 0x5F, // REAL: end note
    },
];

#[allow(dead_code)]
pub static INSTRFX: [InstrFx; 0] = [];
