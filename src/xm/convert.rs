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

use xmrs::patternslot::PatternSlot;
use xmrs::xm::xmpatternslot::XmPatternSlot;

use crate::rhplayer::patternrh::PatternRh;
use crate::rhsongs::RhSongs;

pub struct Convert;

impl Convert {
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

        Module {
            name: name,
            comment: editor,
            frequency_type: FrequencyType::LinearFrequencies,
            restart_position: 0,
            default_tempo: 6,
            default_bpm: 125,
            pattern_order: pattern_order,
            pattern: pattern,
            instrument: vec![],
        }
    }
}
