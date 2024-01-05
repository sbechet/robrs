use crate::rhplayer::patternrh::PatternRh;
use crate::rhsongs::RhSongs;

use xmrs::module::Pattern;
use xmrs::patternslot::PatternSlot;

pub struct ChannelIterator<'a> {
    pub ch0: &'a [u8],
    pub ch1: &'a [u8],
    pub ch2: &'a [u8],
    pub tracks: Vec<Vec<PatternSlot>>,
    index: usize,
}

impl<'a> ChannelIterator<'a> {
    pub fn new(song: &'a RhSongs, number: usize) -> Self {
        let (ch0, ch1, ch2) = (
            song.tracks[number][0],
            song.tracks[number][1],
            song.tracks[number][2],
        );

        ChannelIterator {
            ch0,
            ch1,
            ch2,
            tracks: Self::convert_tracks(song),
            index: 0,
        }
    }

    pub fn reset(&mut self) {
        self.index = 0;
    }

    pub fn convert_tracks(song: &RhSongs) -> Vec<Vec<PatternSlot>> {
        let mut pss: Vec<Vec<PatternSlot>> = vec![];

        for (idx, pattern) in song.patterns.into_iter().enumerate() {
            let r = Vec::uncompress(pattern, song.version);
            match r {
                Some(d) => {
                    // print!("==========\ntrack {}: ", idx);
                    // println!("{:#?}",d);
                    // print!("track={}, ", idx);
                    let mut track: Vec<PatternSlot> = Self::note2rows(&d);
                    // println!("idx={}, len={}",idx, track.len());
                    // println!("{:#?}", pattern[0]);
                    pss.push(track.clone());
                    // Self::write_track(idx, &mut pattern[0]);
                }
                _ => {
                    println!("convert empty");
                }
            }
        }
        pss
    }

    pub fn max3(a: usize, b: usize, c: usize) -> usize {
        if a >= b && a >= c {
            a
        } else if b >= a && b >= c {
            b
        } else {
            c
        }
    }

    fn get_min_max_len(d: &Vec<crate::rhplayer::note::Note>) -> (u16, u16) {
        let mut min_delay = 65535;
        let mut max_delay = 0;

        for n in d {
            if 1 + n.length < min_delay {
                min_delay = 1 + n.length;
            }
            if 1 + n.length > max_delay {
                max_delay = 1 + n.length;
            }
        }

        (min_delay, max_delay)
    }

    fn fill(vps: &mut Vec<PatternSlot>, len: usize) {
        let emptyps = PatternSlot::default();
        let good_len = len - vps.len();
        for _ in 0..good_len {
            vps.push(emptyps.clone());
        }
    }

    fn note2rows(d: &Vec<crate::rhplayer::note::Note>) -> Vec<PatternSlot> {
        let mut total_len = 0;
        let (min_delay, max_delay) = Self::get_min_max_len(&d);
        let mut r: Vec<PatternSlot> = vec![];
        let psd = PatternSlot::default();

        for n in d {
            /* effects : portamento or release */
            let (et, mut ep) = if n.portamento != 0 {
                if n.portamento < 0 {
                    (2 as u8, -n.portamento as u8)
                } else {
                    (1 as u8, n.portamento as u8)
                }
            } else if n.release {
                //   (0x14 as u8, 1+n.length as u8)
                (0 as u8, 0 as u8)
            } else {
                (0 as u8, 0 as u8)
            };

            ep = (ep as u16 / min_delay) as u8;

            let ps = PatternSlot {
                note: xmrs::note::Note::try_from(1 + n.value).unwrap(),
                instrument: n.instr,
                volume: 0,
                effect_type: et,
                effect_parameter: ep,
            };
            r.push(ps);
            let delay = (n.length + 1) / min_delay;
            total_len += delay;
            for _i in 0..(delay - 1) {
                r.push(psd);
            }
        }
        // let warn = if total_len > 256 { '*' } else { ' ' };
        // println!("total_len = {}{}, nnote={}, min={}, max={}, div={}", total_len, warn, r.len(), min_delay, max_delay, total_len as f32 / min_delay as f32);
        r
    }
}

impl<'a> Iterator for ChannelIterator<'a> {
    type Item = Pattern;

    fn next(&mut self) -> Option<Self::Item> {
        let mut vps: Vec<Vec<PatternSlot>> = vec![];

        for ch in [self.ch0, self.ch1, self.ch2] {
            let cur_vps = if self.index < ch.len() {
                if ch[self.index] as usize != 255 {
                    self.tracks[ch[self.index] as usize].clone()
                } else {
                    vec![]
                }
            } else {
                vec![]
            };
            vps.push(cur_vps);
        }

        if vps[0].len() == 0 && vps[1].len() == 0 && vps[2].len() == 0 {
            return None;
        }

        let maxlen = Self::max3(vps[0].len(), vps[1].len(), vps[2].len());
        Self::fill(&mut vps[0], maxlen);
        Self::fill(&mut vps[1], maxlen);
        Self::fill(&mut vps[2], maxlen);

        /* Now reorder to create a real 32 channels pattern, not track after track */
        let mut good_pattern: Vec<Vec<PatternSlot>> = Vec::with_capacity(maxlen);
        for i in 0..maxlen {
            let mut row: Vec<PatternSlot> = vec![];
            row.push(vps[0][i]);
            row.push(vps[1][i]);
            row.push(vps[2][i]);
            let fill_len = 32 - 3;
            for _ in 0..fill_len {
                row.push(PatternSlot::default());
            }
            good_pattern.push(row);
        }

        let item = good_pattern;
        self.index += 1;
        return Some(item);
    }
}
