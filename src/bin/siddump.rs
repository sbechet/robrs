use clap::Parser;
use std::collections::HashMap;
// use std::fs::File;
// use std::io::prelude::*;
use std::path::Path;

pub struct OriginalSong<'a> {
    pub filename: &'a str,
    pub name: &'a str,
    pub author: &'a str,
    pub copyright: &'a str,
    pub compression_version: usize,
    pub load_adress: usize,
    pub song_track_qty: usize,
    pub song_list_offset: usize,
    pub song_list_qty: usize,
    pub patt_ptl_offset: usize,
    pub patt_pth_offset: usize,
    pub patt_qty: usize,
    pub instr_offset: usize,
    pub instr_qty: usize,
    pub fx_v1_offset: usize,
    pub fx_v1_qty: usize,
    pub fx_v2_offset: usize,
    pub fx_v2_qty: usize,
    pub resetspd: usize,
    pub skydive_v1_when: usize, // When length > skydive_v1_when
    pub skydive_v1_add: isize,  // XXX if freq+skydive_v1_add<0x10000
}

impl<'a> OriginalSong<'a> {
    pub fn get_list() -> Vec<&'a str> {
        let mut list: Vec<&'a str> = vec![];
        for i in 0..DATABASE.len() {
            list.push(DATABASE[i].name);
        }
        return list;
    }

    pub fn get_song(name: &str) -> Option<&OriginalSong> {
        for i in 0..DATABASE.len() {
            if DATABASE[i].name == name {
                return Some(&DATABASE[i]);
            }
        }
        return None;
    }

    fn extract(&self) {
        let spath = format!("sid/{}", &self.filename);
        let path = Path::new(&spath);
        let display = path.display();
        let sidfile = match std::fs::read(path) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(file) => file,
        };

        let mut songs: Vec<usize> = vec![];

        println!("// {} - {} - {}\n", self.name, self.author, self.copyright);
        println!(
            "use crate::rhplayer::rhsongs::{{ RhSongs, Instrument, SidT, SoundFx, InstrFx }};"
        );
        println!(
            r###"#[allow(dead_code)]
                pub static RHSONGS: RhSongs = RhSongs {{
                    version: {},
                    total: {},
                    channels: &CHANNELS,
                    tracks: &TRACKS,
                    instruments: &INSTRUMENTS,
                    soundfx: &SOUNDFX,
                    instrfx: &INSTRFX,
                    resetspd: {},
                    skydive_v1_when: {},
                    skydive_v1_add: {},
                }};"###,
            self.compression_version,
            self.song_list_qty + self.fx_v1_qty,
            self.resetspd,
            self.skydive_v1_when,
            self.skydive_v1_add
        );

        // ********* PREPARE SONGS
        for i in 0..self.song_list_qty {
            let offset =
                126 + self.song_list_offset - self.load_adress + i * 2 * self.song_track_qty;
            for j in 0..self.song_track_qty {
                songs.push(
                    sidfile[offset + j] as usize
                        | (sidfile[offset + j + self.song_track_qty] as usize) << 8,
                );
            }
        }
        // println!("XXX songs={:?}", songs);

        // ********* CHANNELS
        let mut channels: HashMap<usize, usize> = HashMap::new();
        for (i, offset) in songs.iter().enumerate() {
            let file_offset = *offset as usize + 126 - self.load_adress;
            let mut track: Vec<u8> = vec![];
            let mut j = 0;
            channels.insert(file_offset, i);
            loop {
                track.push(sidfile[file_offset + j]);
                if sidfile[file_offset + j] == 0xfe || sidfile[file_offset + j] == 0xff {
                    break;
                }
                j += 1;
            }
            println!("\n\n#[allow(dead_code)]");
            println!("pub static CHANNEL_{}: [u8; {}] = {:?};", i, j + 1, track);
        }
        // println!("channels={:?}", channels);

        if self.song_track_qty == 2 {
            println!("\n\n#[allow(dead_code)]");
            println!("pub static CHANNEL_EMPTY: [u8; 1] = [255];");
        }

        // ********* PRINT CHANNELS
        println!("\n\n#[allow(dead_code)]");
        println!(
            "pub static CHANNELS: [ &[&[u8];3]; {}] = [",
            self.song_list_qty
        );
        for i in 0..self.song_list_qty {
            // TODO: One day cleanup?
            match self.song_track_qty {
                2 => {
                    let voice0 = songs[i * self.song_track_qty] + 126 - self.load_adress;
                    let voice1 = songs[i * self.song_track_qty + 1] + 126 - self.load_adress;
                    println!(
                        "    &[&CHANNEL_{},&CHANNEL_{},&CHANNEL_EMPTY],",
                        channels[&voice0], channels[&voice1]
                    );
                }
                _ => {
                    let voice0 = songs[i * self.song_track_qty] + 126 - self.load_adress;
                    let voice1 = songs[i * self.song_track_qty + 1] + 126 - self.load_adress;
                    let voice2 = songs[i * self.song_track_qty + 2] + 126 - self.load_adress;
                    println!(
                        "    &[&CHANNEL_{},&CHANNEL_{},&CHANNEL_{}],",
                        channels[&voice0], channels[&voice1], channels[&voice2]
                    );
                }
            }
        }
        println!("];");

        // ********* TRACKS
        let mut tracks: Vec<u16> = vec![];
        for i in 0..self.patt_qty {
            let offset_low_file = 126 + self.patt_ptl_offset - self.load_adress;
            let offset_high_file = 126 + self.patt_pth_offset - self.load_adress;
            let offset: u16 =
                sidfile[offset_low_file + i] as u16 | (sidfile[offset_high_file + i] as u16) << 8;
            tracks.push(offset);
        }
        // println!("{:04X?}",tracks);
        println!("\n\n#[allow(dead_code)]");
        println!("pub static TRACKS: [&[u8]; {}] = [", self.patt_qty);
        for i in 0..self.patt_qty {
            print!("&TRACK_{},", i);
        }
        println!("];");

        // println!("pub static DEBUG_TRACKS_OFFSET: [usize; {}] = {:#X?};", tracks.len(), tracks);
        for i in 0..self.patt_qty {
            let file_offset = 126 + tracks[i] as usize - self.load_adress;
            let mut pattern: Vec<u8> = vec![];
            let mut j = 0;
            loop {
                pattern.push(sidfile[file_offset + j]);
                if sidfile[file_offset + j] == 0xff {
                    break; // TODO: Sometimes test bug, see international karate
                }
                j += 1;
            }
            println!("#[allow(dead_code)]");
            println!(
                "pub static TRACK_{}: [u8; {}] = {:#X?};",
                i,
                pattern.len(),
                pattern
            );
        }

        // ********* INSTRUMENTS
        println!("\n\n#[allow(dead_code)]");
        println!(
            "pub static INSTRUMENTS: [ Instrument; {}] = [",
            self.instr_qty
        );
        for i in 0..self.instr_qty {
            let file_offset = 126 + self.instr_offset - self.load_adress;
            let start = file_offset + i * 8;
            println!("    Instrument {{ pulse_width:{}, ctrl_register:0b{:08b}, attack_and_decay:0x{:02X}, sustain_and_release:0x{:02X}, vibrato_depth:{}, pulse_speed:0x{:02X}, fx:0b{:08b} }},",
                    sidfile[start] as u16|(sidfile[start+1] as u16)<<8,sidfile[start+2],sidfile[start+3],sidfile[start+4],sidfile[start+5],sidfile[start+6],sidfile[start+7]);
        }
        println!("];");

        // ********* SOUNDFX
        println!("\n\n#[allow(dead_code)]");
        println!("pub static SOUNDFX: [ SoundFx; {}] = [", self.fx_v1_qty);
        for i in 0..self.fx_v1_qty {
            let file_offset = 126 + self.fx_v1_offset - self.load_adress;
            let start = file_offset + i * 16;
            println!("    SoundFx {{
                incdec: 0b{:08b},
                voice0: SidT {{
                    freq: 0x{:04X},     // REAL: lower part is used as start note. 
                    pulse_width: {},
                    ctrl: 0b{:08b},
                    attack_and_decay_len: 0x{:02X},
                    sustain_vol_and_release_len: 0x{:02X},
                }},
                voice1: SidT {{
                    freq: 0x{:04X},     // REAL: value & 0b0011_1111 = note_delta ; sometime if value&0b1_0000000: flip_flop voice0 ctrl ; if value&0b0_1_000000: flip_flop voice1 ctrl
                    pulse_width: {},
                    ctrl: 0b{:08b},
                    attack_and_decay_len: 0x{:02X},
                    sustain_vol_and_release_len: 0x{:02X},
                }},
                sfx_note_dest: 0x{:02X},    // REAL: end note
            }}, ",
            sidfile[start],

            sidfile[start+1] as u16|(sidfile[start+2] as u16)<<8,
            sidfile[start+3] as u16|(sidfile[start+4] as u16)<<8,
            sidfile[start+5],
            sidfile[start+6],
            sidfile[start+7],

            sidfile[start+8] as u16|(sidfile[start+9] as u16)<<8,
            sidfile[start+10] as u16|(sidfile[start+11] as u16)<<8,
            sidfile[start+12],
            sidfile[start+13],
            sidfile[start+14],

            sidfile[start+15],
            );
        }
        println!("];");

        // ********* INSTRFX
        println!("\n\n#[allow(dead_code)]");
        println!("pub static INSTRFX: [ InstrFx; {}] = [", self.fx_v2_qty);
        for i in 0..self.fx_v2_qty {
            let file_offset = 126 + self.fx_v2_offset - self.load_adress;
            let start = file_offset + i * 8;
            println!(
                "    InstrFx {{
                vibdepth_note: {},
                arpt: 0b{:08b},
                skydive: 0b{:08b},
                arpt_counter: {},
                notenum: {},
                pw_minmax: 0x{:02x},
                resfilt: 0b{:08b},
                fchi: 0b{:08b},
            }}, ",
                sidfile[start],
                sidfile[start + 1],
                sidfile[start + 2],
                sidfile[start + 3],
                sidfile[start + 4],
                sidfile[start + 5],
                sidfile[start + 6],
                sidfile[start + 7],
            );
        }
        println!("];");
    }
}

pub static DATABASE: [OriginalSong; 14] = [
    OriginalSong {
        filename: "ACE_II.sid",
        name: "ACE II",
        author: "Rob Hubbard",
        copyright: "1987 Arcade",
        compression_version: 20,
        load_adress: 0xE000,
        song_track_qty: 3,
        song_list_offset: 0xE67C,
        song_list_qty: 1,
        patt_ptl_offset: 0xE682,
        patt_pth_offset: 0xE6A8,
        patt_qty: 38,
        instr_offset: 0xE5CB,
        instr_qty: 10,
        fx_v1_offset: 0,
        fx_v1_qty: 0,
        fx_v2_offset: 0,
        fx_v2_qty: 0,
        resetspd: 1,
        skydive_v1_when: 0,
        skydive_v1_add: 0,
    },
    OriginalSong {
        filename: "Commando.sid",
        name: "Commando",
        author: "Rob Hubbard",
        copyright: "1985 Elite",
        compression_version: 10,
        load_adress: 0x5000,
        song_track_qty: 3,
        song_list_offset: 0x56FF,
        song_list_qty: 3,
        patt_ptl_offset: 0x5711,
        patt_pth_offset: 0x573E,
        patt_qty: 45,
        instr_offset: 0x5591,
        instr_qty: 13,
        fx_v1_offset: 0x55F9,
        fx_v1_qty: 16,
        fx_v2_offset: 0,
        fx_v2_qty: 0,
        resetspd: 2, // [2, 3, 2]
        skydive_v1_when: 2,
        skydive_v1_add: 512,
    },
    OriginalSong {
        filename: "Crazy_Comets.sid",
        name: "Crazy Comets",
        author: "Rob Hubbard",
        copyright: "1985 Martech",
        compression_version: 10,
        load_adress: 0x5000,
        song_track_qty: 3,
        song_list_offset: 0x5732,
        song_list_qty: 2,
        patt_ptl_offset: 0x573E,
        patt_pth_offset: 0x5773,
        patt_qty: 53,
        instr_offset: 0x5574,
        instr_qty: 23,
        fx_v1_offset: 0x562C,
        fx_v1_qty: 1,
        fx_v2_offset: 0,
        fx_v2_qty: 0,
        resetspd: 2,
        skydive_v1_when: 16,
        skydive_v1_add: -256,
    },
    OriginalSong {
        filename: "Delta.sid",
        name: "Delta",
        author: "Rob Hubbard",
        copyright: "1987 Thalamus",
        compression_version: 30, // Compression _and_ pattern loop in channels
        load_adress: 0xBC00,
        song_track_qty: 3,
        song_list_offset: 0xC4F4,
        song_list_qty: 13,
        patt_ptl_offset: 0xC542,
        patt_pth_offset: 0xC5AF,
        patt_qty: 109,
        instr_offset: 0xC38E,
        instr_qty: 22,
        fx_v1_offset: 0,
        fx_v1_qty: 0,
        fx_v2_offset: 0xC43E,
        fx_v2_qty: 22,
        resetspd: 1,
        skydive_v1_when: 0,
        skydive_v1_add: 0,
    },
    OriginalSong {
        filename: "Human_Race.sid",
        name: "The Human Race",
        author: "Rob Hubbard",
        copyright: "1985 Mastertronic",
        compression_version: 20,
        load_adress: 0x0980,
        song_track_qty: 2,
        song_list_offset: 0x0E9F,
        song_list_qty: 5,
        patt_ptl_offset: 0x0EB3,
        patt_pth_offset: 0x0F02,
        patt_qty: 79,
        instr_offset: 0x0DE3,
        instr_qty: 23,
        fx_v1_offset: 0,
        fx_v1_qty: 0,
        fx_v2_offset: 0,
        fx_v2_qty: 0,
        resetspd: 3,
        skydive_v1_when: 0,
        skydive_v1_add: -256,
    },
    OriginalSong {
        filename: "International_Karate.sid",
        name: "International Karate",
        author: "Rob Hubbard",
        copyright: "1986 System 3",
        compression_version: 20,
        load_adress: 0xAE00,
        song_track_qty: 3,
        song_list_offset: 0xB3B0,
        song_list_qty: 1,
        patt_ptl_offset: 0xB3B6,
        patt_pth_offset: 0xB3EB,
        patt_qty: 53,
        instr_offset: 0xB308,
        instr_qty: 20,
        fx_v1_offset: 0,
        fx_v1_qty: 0,
        fx_v2_offset: 0,
        fx_v2_qty: 0,
        resetspd: 2,
        skydive_v1_when: 0,
        skydive_v1_add: 0,
    },
    OriginalSong {
        filename: "Last_V8.sid",
        name: "The Last V8",
        author: "Rob Hubbard",
        copyright: "1985 MAD/Mastertronic",
        compression_version: 10,
        load_adress: 0x8010,
        song_track_qty: 3,
        song_list_offset: 0x8797,
        song_list_qty: 3,
        patt_ptl_offset: 0x87A9,
        patt_pth_offset: 0x87C6,
        patt_qty: 29,
        instr_offset: 0x85A1,
        instr_qty: 19,
        fx_v1_offset: 0x8699,
        fx_v1_qty: 12,
        fx_v2_offset: 0,
        fx_v2_qty: 0,
        resetspd: 1,
        skydive_v1_when: 0,
        skydive_v1_add: -256,
    },
    OriginalSong {
        filename: "Lightforce.sid",
        name: "Lightforce",
        author: "Rob Hubbard",
        copyright: "1986 Faster Than Light (FTL)",
        compression_version: 20,
        load_adress: 0xF000,
        song_track_qty: 3,
        song_list_offset: 0xF778,
        song_list_qty: 1,
        patt_ptl_offset: 0xF77E,
        patt_pth_offset: 0xF79D,
        patt_qty: 31,
        instr_offset: 0xF618,
        instr_qty: 22,
        fx_v1_offset: 0,
        fx_v1_qty: 0,
        fx_v2_offset: 0xF6C8,
        fx_v2_qty: 22,
        resetspd: 2,
        skydive_v1_when: 0,
        skydive_v1_add: 0,
    },
    OriginalSong {
        filename: "Monty_on_the_Run.sid",
        name: "Monty on the Run",
        author: "Rob Hubbard",
        copyright: "1985 Gremlin Graphics",
        compression_version: 10,
        load_adress: 0x8000,
        song_track_qty: 3,
        song_list_offset: 0x856C,
        song_list_qty: 3,
        patt_ptl_offset: 0x857E,
        patt_pth_offset: 0x85CB,
        patt_qty: 77,
        instr_offset: 0x93B4,
        instr_qty: 20,
        fx_v1_offset: 0x9454,
        fx_v1_qty: 16,
        fx_v2_offset: 0,
        fx_v2_qty: 0,
        resetspd: 1,
        skydive_v1_when: 0,
        skydive_v1_add: -256,
    },
    OriginalSong {
        filename: "Sanxion.sid",
        name: "Sanxion Song 1",
        author: "Rob Hubbard",
        copyright: "1986 Thalamus",
        compression_version: 20,
        load_adress: 0xB000,
        song_track_qty: 3,
        song_list_offset: 0xB73C,
        song_list_qty: 1,
        patt_ptl_offset: 0xB742,
        patt_pth_offset: 0xB75D,
        patt_qty: 27,
        instr_offset: 0xB56C,
        instr_qty: 29,
        fx_v1_offset: 0,
        fx_v1_qty: 0,
        fx_v2_offset: 0xB654,
        fx_v2_qty: 29,
        resetspd: 2,
        skydive_v1_when: 0,
        skydive_v1_add: 0,
    },
    OriginalSong {
        filename: "Sanxion.sid",
        name: "Sanxion Song 2",
        author: "Rob Hubbard",
        copyright: "1986 Thalamus",
        compression_version: 20,
        load_adress: 0xB000,
        song_track_qty: 3,
        song_list_offset: 0xC5F5,
        song_list_qty: 1,
        patt_ptl_offset: 0xC5FB,
        patt_pth_offset: 0xC633,
        patt_qty: 56,
        instr_offset: 0xC4B5,
        instr_qty: 20,
        fx_v1_offset: 0,
        fx_v1_qty: 0,
        fx_v2_offset: 0xC5F5,
        fx_v2_qty: 20,
        resetspd: 2,
        skydive_v1_when: 0,
        skydive_v1_add: 0,
    },
    OriginalSong {
        filename: "Spellbound.sid",
        name: "Spellbound",
        author: "Rob Hubbard",
        copyright: "1986 MAD/Mastertronic",
        compression_version: 15, // XXX WARN: 0xE0E5 second byte & 0b1000_0000 is instrnr: not a vibrato! no vibrato. Using version 15 -- check soundfx too
        load_adress: 0xE000,
        song_track_qty: 3,
        song_list_offset: 0xE6B6,
        song_list_qty: 3,
        patt_ptl_offset: 0xE6C8,
        patt_pth_offset: 0xE6F2,
        patt_qty: 42,
        instr_offset: 0xE548,
        instr_qty: 25,
        fx_v1_offset: 0xE610,
        fx_v1_qty: 16,
        fx_v2_offset: 0,
        fx_v2_qty: 0,
        resetspd: 1,
        skydive_v1_when: 0,
        skydive_v1_add: 256,
    },
    OriginalSong {
        filename: "Thing_on_a_Spring.sid",
        name: "Thing on a Spring",
        author: "Rob Hubbard",
        copyright: "1985 Gremlin Graphics",
        compression_version: 10,
        load_adress: 0xC000,
        song_track_qty: 3,
        song_list_offset: 0xC509,
        song_list_qty: 1,
        patt_ptl_offset: 0xC50F,
        patt_pth_offset: 0xC533,
        patt_qty: 36,
        instr_offset: 0xCD2A,
        instr_qty: 45,
        fx_v1_offset: 0xCE92,
        fx_v1_qty: 1,
        fx_v2_offset: 0,
        fx_v2_qty: 0,
        resetspd: 1,
        skydive_v1_when: 0,
        skydive_v1_add: 0,
    },
    OriginalSong {
        filename: "Zoids.sid",
        name: "Zoids",
        author: "Rob Hubbard",
        copyright: "1986 Martech",
        compression_version: 10,
        load_adress: 0x1000,
        song_track_qty: 3,
        song_list_offset: 0x14FC,
        song_list_qty: 3,
        patt_ptl_offset: 0x150E,
        patt_pth_offset: 0x152D,
        patt_qty: 31,
        instr_offset: 0x147E,
        instr_qty: 15,
        fx_v1_offset: 0,
        fx_v1_qty: 0,
        fx_v2_offset: 0,
        fx_v2_qty: 0,
        resetspd: 1,
        skydive_v1_when: 0,
        skydive_v1_add: 0,
    },
];

#[derive(Parser)]
struct Cli {
    name: String,
}

fn main() {
    let list = OriginalSong::get_list();
    let cli = Cli::parse();

    match OriginalSong::get_song(&*cli.name) {
        Some(song) => {
            song.extract();
        }
        _ => {
            for i in 0..list.len() {
                println!("{}", list[i]);
            }
        }
    }
}
