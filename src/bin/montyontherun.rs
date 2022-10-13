use std::collections::HashMap;

pub static PSID_HEADER_SIZE: usize = 126;

pub static PSID_DATA_LOAD_ADDR: usize = 0x8000; // *124

pub static TRACKS_LIST_OFFSET:usize = 0x856C;
pub static TRACKS_LIST_QTY:usize = 3;

pub static PATTPTL_OFFSET:usize = 0x857E;
pub static PATTPTH_OFFSET:usize = 0x85CB;
pub static PATT_QTY:usize = 77;

pub static INSTR_OFFSET: usize = 0x93B4;
pub static INSTR_QTY: usize = 20;
pub static INSTR_STRUCT_SIZE: usize = 8;

pub static SOUNDFX_OFFSET: usize = 0x9454;
pub static SOUNDFX_QTY: usize = 16;
pub static SOUNDFX_STRUCT_SIZE: usize = 16;

fn main() {
    let sidfile = include_bytes!("Monty_on_the_Run.sid");
    let mut songs: Vec<usize> = vec![];

    println!("// Monty on the Run - Rob Hubbard - 1985 Gremlin Graphics\n");
    println!("use super::rhplayer::{{ RhSongs, Instrument, SidT, SoundFx }};");
    println!(r###"#[allow(dead_code)]
pub static RHSONGS: RhSongs = RhSongs {{
    total: TOTAL_SONGS,
    tracks: &TRACKS,
    patterns: &PATTERNS,
    instruments: &INSTRUMENTS,
    soundfx: &SOUNDFX
}};"###);

    // ********* TOTAL_SONGS
    println!("\n\n#[allow(dead_code)]");
    println!("pub static TOTAL_SONGS: usize = {};",TRACKS_LIST_QTY + SOUNDFX_QTY);

    // ********* PREPARE SONGS
    for i in 0..TRACKS_LIST_QTY {
        let offset = PSID_HEADER_SIZE+TRACKS_LIST_OFFSET-PSID_DATA_LOAD_ADDR+i*6;
        let voice0: usize = sidfile[offset] as usize | (sidfile[offset+3] as usize) << 8;
        let voice1: usize = sidfile[offset+1] as usize | (sidfile[offset+3+1] as usize) << 8;
        let voice2: usize = sidfile[offset+2] as usize | (sidfile[offset+3+2] as usize) << 8;
        songs.push(voice0);songs.push(voice1);songs.push(voice2);
    }
    // println!("songs={:?}", songs);

    // ********* TRACKS
    let mut tracks: HashMap<usize, usize> = HashMap::new();
    for (i, offset) in songs.iter().enumerate() {
        let file_offset = *offset as usize + PSID_HEADER_SIZE - PSID_DATA_LOAD_ADDR;
        let mut track: Vec<u8> = vec![];
        let mut j = 0;
        tracks.insert(file_offset, i);
        loop {
            track.push(sidfile[file_offset+j]);
            if sidfile[file_offset+j] == 0xfe || sidfile[file_offset+j] == 0xff {
                break;
            }
            j += 1;
        }
        println!("\n\n#[allow(dead_code)]");
        println!("pub static TRACK_{}: [u8; {}] = {:?};",i,j+1, track);
    }
    // println!("tracks={:?}", tracks);

    // ********* PRINT TRACKS
    println!("\n\n#[allow(dead_code)]");
    println!("pub static TRACKS: [ &[&[u8];3]; {}] = [", TRACKS_LIST_QTY);
    for i in 0..TRACKS_LIST_QTY {
        let voice0 = songs[i*3]+PSID_HEADER_SIZE-PSID_DATA_LOAD_ADDR;
        let voice1 = songs[i*3+1]+PSID_HEADER_SIZE-PSID_DATA_LOAD_ADDR;
        let voice2 = songs[i*3+2]+PSID_HEADER_SIZE-PSID_DATA_LOAD_ADDR;
        // println!("{} {} {}", voice0, voice1, voice2);
        println!("    &[&TRACK_{},&TRACK_{},&TRACK_{}],", tracks[&voice0], tracks[&voice1], tracks[&voice2]);
    }
    println!("];");


    // ********* PATTERNS
    let mut patterns: Vec<u16> = vec![];
    for i in 0..PATT_QTY {
        let offset_low_file = PSID_HEADER_SIZE+PATTPTL_OFFSET-PSID_DATA_LOAD_ADDR;
        let offset_high_file = PSID_HEADER_SIZE+PATTPTH_OFFSET-PSID_DATA_LOAD_ADDR;
        let offset: u16 = sidfile[offset_low_file+i] as u16 | (sidfile[offset_high_file+i] as u16) << 8;
        patterns.push(offset);
    }
    // println!("{:04X?}",patterns);
    println!("\n\n#[allow(dead_code)]");
    println!("pub static PATTERNS: [&[u8]; {}] = [", PATT_QTY);
    for i in 0..PATT_QTY {
        print!("&PATTERN_{},", i);
    }
    println!("];");

    for i in 0..PATT_QTY {
        let file_offset = PSID_HEADER_SIZE + patterns[i] as usize - PSID_DATA_LOAD_ADDR;
        let mut pattern: Vec<u8> = vec![];
        let mut j = 0;
        loop {
            pattern.push(sidfile[file_offset+j]);
            if sidfile[file_offset+j] == 0xff {
                break;
            }
            j += 1;
        }
        // HACK debug an original note bug
        if i == 18 {
            if pattern[2] == 0x68 {
                println!("\n\n// Found pitch bug or rip bug in original song? 0x68 -> 0x48");
                pattern[2] = 0x48;  // 0x68 == 0b0110_1000. 0x48=0b0100_1000 only one bit difference and sound good.
            }
        }
        // END HACK
        println!("#[allow(dead_code)]");
        println!("pub static PATTERN_{}: [u8; {}] = {:#X?};",i,pattern.len(), pattern);
    }

    // ********* INSTRUMENTS
    println!("\n\n#[allow(dead_code)]");
    // println!("pub static INSTRUMENTS: [ [u8; {}]; {}] = [", INSTR_STRUCT_SIZE, INSTR_QTY);
    println!("pub static INSTRUMENTS: [ Instrument; {}] = [", INSTR_QTY);
    for i in 0..INSTR_QTY {
        let file_offset = PSID_HEADER_SIZE + INSTR_OFFSET - PSID_DATA_LOAD_ADDR;
        let start = file_offset+i*INSTR_STRUCT_SIZE;
        // let end = start + INSTR_STRUCT_SIZE;
        // println!("    {:#X?},",&sidfile[start..end]);
        println!("    Instrument {{ pulse_width:{}, ctrl_register:0b{:08b}, attack_and_decay:0x{:02X}, sustain_and_release:0x{:02X}, vibrato_depth:{}, pulse_speed:{}, fx:0b{:08b} }},",
                sidfile[start] as u16|(sidfile[start+1] as u16)<<8,sidfile[start+2],sidfile[start+3],sidfile[start+4],sidfile[start+5],sidfile[start+6],sidfile[start+7]);
    }
    println!("];");

    // ********* SOUNDFX
    println!("\n\n#[allow(dead_code)]");
    println!("pub static SOUNDFX: [ SoundFx; {}] = [", SOUNDFX_QTY);
    for i in 0..SOUNDFX_QTY {
        let file_offset = PSID_HEADER_SIZE + SOUNDFX_OFFSET - PSID_DATA_LOAD_ADDR;
        let start = file_offset+i*SOUNDFX_STRUCT_SIZE;
        // let end = start + SOUNDFX_STRUCT_SIZE;
        // println!("    {:#X?},",&sidfile[start..end]);
        println!("    SoundFx {{
            incdec: 0b{:08b},
            voice0: SidT {{
                freq: 0x{:04X},
                pulse_width: {},
                ctrl: {},
                attack_and_decay_len: {},
                sustain_vol_and_release_len: {},
            }},
            voice1: SidT {{
                freq: 0x{:04X},
                pulse_width: {},
                ctrl: {},
                attack_and_decay_len: {},
                sustain_vol_and_release_len: {},
            }},
            sfx_note_dest: 0x{:02X},
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



}
