use ahash::{AHashMap, AHasher, RandomState};
use std::collections::HashMap;
use std::hash::BuildHasher;
use std::sync::Arc;

use std::fs::File;
use std::io::Write;

use super::channel_iterator::ChannelIterator;

use xmrs::module::Module;
use xmrs::module::Pattern;
use xmrs::prelude::*;
use xmrs::xm::xp_pattern::XpPattern;
use xmrs::xm::xt_track::XtTrack;

use xmrs::instr_default::InstrDefault;
use xmrs::instr_robsid::InstrRobSid;
use xmrs::instr_sid::InstrSid;
use xmrs::instrument::{Instrument, InstrumentType};

use xmrs::patternslot::PatternSlot;
use xmrs::xm::xmpatternslot::XmPatternSlot;

use crate::rhplayer::patternrh::PatternRh;
use crate::rhsongs::RhSongs;

pub struct Convert;

impl Convert {
    fn get_instrument(song: &RhSongs, number: usize) -> Instrument {
        // 7 : *noise*
        // 6 : *pulse*
        // 5 : *saw*tooth
        // 4 : *tri*angle
        // 3 : *test*
        // 2 : *ring* modulation with voice 3
        // 1 : *sync*hronize with voice 3
        // 0 : gate
        let name = if song.instruments[number].ctrl_register & 0b1000_0000 == 0b1000_0000 {
            "noise"
        } else if song.instruments[number].ctrl_register & 0b0100_0000 == 0b0100_0000 {
            "pulse"
        } else if song.instruments[number].ctrl_register & 0b0010_0000 == 0b0010_0000 {
            "sawtooth"
        } else if song.instruments[number].ctrl_register & 0b0001_0000 == 0b0001_0000 {
            "triangle"
        } else {
            "nothing"
        };

        // 12 bits -> 0-4095 with 2048 for center
        let pw = song.instruments[number].pulse_width;

        let mask_lo: u8 = if let 15 = song.version {
            0b0000_1111
        } else {
            0b000_11111
        };
        let mask_hi: u8 = if let 15 = song.version {
            0b1111_0000
        } else {
            0b111_00000
        };
        /*
        Each pulse_delay,
            1. increment pw from pulsespeed until >=3584 ( = 2048 + 3/4 * 2048 = center + 3/4)
            2. then decrement pw from pulsespeed until <=2048
            3. loop
        tick = 1/50 s = 0,02s: real_delay = pulse_delay*0,02

        more long time: pw=0, pulse_delay=31, pulsespeed=1
        pw=0,pd:31,ps:1 -> 31*(3584-0)/1 = 111104 ticks -> 111104 * 0.02 = 2222.08 seconds = 37 minutes
        ping_pong with loop_start to 2048*F, un loop_end to 3584*F

        monty music:
        instr 8  pw:2688,pd:31,ps:64 -> 31*(3584-2688)/64=434 ticks -> 434 * 0,02 = 8,68 seconds
        instr 13 pw:2048,pd:8,ps:224 -> 8*(3584-2048)/224=54 ticks 54 * 0,02 = 1,1 seconds

        -> 10 seconds samples * 48000 = 480KiB, yes we can...

         */
        let pulse_delay = song.instruments[number].pulse_speed & mask_lo; // 0..31
        let pulsespeed = song.instruments[number].pulse_speed & mask_hi; // 0, 32, 64, 96, 128, 160, 192, 224

        /*
           For the PAL version the C64 has a 17.734472MHz clock, this is than divided by 18 to get the ~985kHz SID clock
           SIDFreq = 985248.444444 Hz
           1.10^9 ns / 985248.444444 = 1014.97242207 ns per tick
        */
        // ns
        let attack =
            (1014.97242207 * (song.instruments[number].attack_and_decay >> 4) as f64) as u16;
        let decay =
            (1014.97242207 * (song.instruments[number].attack_and_decay & 0x0F) as f64) as u16;
        let sustain =
            (1014.97242207 * (song.instruments[number].sustain_and_release >> 4) as f64) as u16;
        let release =
            (1014.97242207 * (song.instruments[number].sustain_and_release & 0xF) as f64) as u16;

        /*
           first version (=10):
               F0=F(note)
               F1=F(note+1)
               vibdepth=0..15

               osc=0..3 then 3..0=if tick&7 > 3 { tick&7 ^ 7 } else { tick&7 };
               delta=(F1-F0)/2^vibdepth
               F = F0 + osc*delta

           next versions:
               F0=F(note-1)
               F1=F(note)
               vibdepth = 0b002222_111 with d1=111 (0-7), d2=2222 (0-15)

               osc=0 if no current note or 0..d2..0
               delta=(F1-F0)/2^d1
               F = F1 + delta * (osc - d2/2)

            see oscillator.rs for helper
        */
        let vibdepth = song.instruments[number].vibrato_depth;

        /*
           TODO: effects
           drum_fx (bit0), noise then for each tick Freq -=256
               La frequence défini la vitesse à la quelle le LFSR (Linear Feedback Shift Register) décale ses bits vers la droite.
               SID6581 : x22+x17+x5+x4+1x22+x17+x5+x4+X0
               SID8580 : x23+x22+x21+x20+x18+x17+x16+x13+x11+x9+x6+x5+x4+1x23+x22+x21+x20+x18+x17+x16+x13+x11+x9+x6+x5+x4+x0.
               SID 6581: Utilise un LFSR de 22 bits.
               SID 8580: Utilise un LFSR de 23 bits.
               le bit à gauche est calculé en faisant le XOR des bits du polynome.

           portamento down (bit1)
           version 10
               freq = if note && freq>256 { freq-256 } else { freq };
                   de façon plus générale :
               freq += songs.skydive_v1_add si note > songs.skydive_v1_when && 0<cur_freq+songs.skydive_v1_add<65536
           version 15
               freq = F(++note)

           octave arpeggio (bit3)
           version 10
               even : F(note)
               odd : F(note+12)
           version 15
               data(fx) = fx>>4
               si data(fx)==12, on bascule tous les deux ticks vers l'octave inferieur (utilisation du bit 2 du counter)
               sinon, on bascule, chaque tick vers la valeur de data(fx)  (utilisation du bit 1 du counter)

        */

        let comment = format!(
            "(pw:{},pd:{},ps:{},a:{},d:{},s:{},r:{},vd:{})",
            pw, pulse_delay, pulsespeed, attack, decay, sustain, release, vibdepth
        );

        Instrument {
            name: format!("{} {}", name, comment),
            instr_type: InstrumentType::Empty,
            muted: false,
        }
    }

    fn write_track(idx: usize, track: &Vec<PatternSlot>) {
        let data = XtTrack::save(track);
        let mut file = match File::create(format!("track{:02}_{:02}.xt", idx, track.len())) {
            Ok(file) => file,
            Err(e) => {
                panic!("Can't open file: {}", e);
            }
        };
        match file.write_all(&data) {
            Ok(_) => {}
            Err(e) => {
                // Gestion de l'erreur si l'écriture échoue
                panic!("Error writing file: {}", e);
            }
        }
    }

    fn serialize_xp_pattern(pattern: &Vec<Vec<PatternSlot>>) -> Option<(u64, Vec<u8>)> {
        let hash_builder = RandomState::with_seed(42);
        let data = XpPattern::save(pattern);
        match data {
            Some(d) => {
                let hash = hash_builder.hash_one(&d);
                Some((hash, d))
            }
            _ => None,
        }
    }

    /// return uniq hash value
    fn write_xp_pattern(idx: usize, data: &Vec<u8>) {
        let mut file = match File::create(format!("pattern{:03}.xp", idx)) {
            Ok(file) => file,
            Err(e) => {
                panic!("Can't open file: {}", e);
            }
        };
        match file.write_all(&data) {
            Ok(_) => {}
            Err(e) => {
                // Gestion de l'erreur si l'écriture échoue
                panic!("Error writing file: {}", e);
            }
        }
    }

    pub fn convert(name: String, editor: String, song: &RhSongs, number: usize) -> Module {
        let ci = ChannelIterator::new(song, number);

        for (idx, track) in ci.tracks.clone().into_iter().enumerate() {
            Self::write_track(idx, &track);
        }

        let mut pattern: Vec<Arc<Vec<Vec<PatternSlot>>>> = vec![];

        // Step #1: map(hash,pattern)
        let mut map_hash_to_pattern: AHashMap<u64, Vec<Vec<PatternSlot>>> = AHashMap::new();
        for (idx, one_pattern) in ci.into_iter().enumerate() {
            let (hash, data) = Self::serialize_xp_pattern(&one_pattern).unwrap();
            map_hash_to_pattern.insert(hash, one_pattern);
            // println!("{} {} {:#?}", hash, idx, one_pattern);
        }

        // Step #2: store uniq pattern AND map(hash, index)
        let mut map_hash_to_pattern_index: AHashMap<u64, usize> = AHashMap::new();
        for (i, p) in map_hash_to_pattern.into_iter().enumerate() {
            pattern.push(Arc::new(p.1));
            map_hash_to_pattern_index.insert(p.0, i);
        }

        // Step #3: create pattern order
        let mut pattern_order: Vec<u8> = vec![];
        let ci = ChannelIterator::new(song, number);
        for one_pattern in ci.into_iter() {
            let (hash, data) = Self::serialize_xp_pattern(&one_pattern).unwrap();
            let index = map_hash_to_pattern_index.get(&hash).unwrap();
            Self::write_xp_pattern(*index, &data);
            pattern_order.push(*index as u8);
        }

        let mut instrument: Vec<Arc<Instrument>> = vec![];
        let qty = song.instruments.len();
        for i in 0..qty {
            let instr = Arc::new(Self::get_instrument(song, i));
            println!("{} : {}", i, instr.name);
            instrument.push(instr);
        }

        Module {
            name: name,
            comment: editor,
            frequency_type: FrequencyType::LinearFrequencies,
            restart_position: 0,
            default_tempo: 6,
            default_bpm: 125,
            pattern_order: pattern_order,
            pattern: pattern,
            instrument: instrument,
        }
    }
}
