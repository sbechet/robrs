/*
  Rh2Midi Sebastien Bechet
*/

use std::{io, io::Write};
use midly::{num, Header, Smf, Format, Timing, Fps, MetaMessage, Track, TrackEvent, TrackEventKind, MidiMessage };
// use std::ops::{BitAnd, BitOr};
use super::rhplayer::note::Note;
// use super::rhplayer::noterh::NoteRh;
use super::rhplayer::patternrh::PatternRh;


#[repr(C, packed)]
pub struct SidT {
  pub freq: u16,
  pub pulse_width: u16,
  pub ctrl: u8,
  pub attack_and_decay_len: u8,
  pub sustain_vol_and_release_len: u8,
}

pub struct SoundFx {
  /*
  incdec: 0bAABB_CCCC
  AA : 0b00 : play current music freq at end of CCCC counter
  BB : 0b10 : INC else DEC voice0 freq low part
  CCCC : IncDec counter
  */
  pub incdec: u8,
  pub voice0: SidT, // freq low part is used as note
  pub voice1: SidT, // freq low part is 0bA_B_CCCCCC with A:voice0_ctrl, B:voice1_ctrl, CCCCCC:note_delta
  pub sfx_note_dest: u8
}

pub struct Instrument {
  pub pulse_width: u16,
  pub ctrl_register: u8,
  pub attack_and_decay: u8,
  pub sustain_and_release: u8,
  pub vibrato_depth: u8,
  pub pulse_speed: u8,  // 0bAAA_BBBBB with A:speed and B:delay
  pub fx: u8, // ABCD_EFGH : H:drum, G:skydive, F:octave_arpeggio, 3-5: fx_use -- commando:E=0:modify default pulse-width effect
}

pub enum MusicPlayer {
  Commando,
  CrazyComets,
  MontyOnTheRun,
  SpellBound,
}

pub struct RhSongs<'a> {
  pub musicplayer: MusicPlayer,
  pub total: usize,
  pub tracks: &'a [&'a[&'a [u8]; 3]],
  pub patterns: &'a [&'a [u8]],
  pub instruments: &'a [Instrument],
  pub soundfx: &'a [SoundFx],
  pub resetspd: u8,

}

impl<'a> RhSongs<'a> {

  pub fn get_pattern(&self, song_idx: usize, channel: usize, pattern_list_idx: usize) -> Option<Vec<Note>> {
    let song = self.tracks[song_idx];
    let pattern_list = song[channel];
    let pattern_num = pattern_list[pattern_list_idx] as usize;
    let pattern = self.patterns[pattern_num];
    let pattern = Vec::uncompress(pattern);
    return pattern;
  }

  pub fn get_track_len(&self, song_idx: usize, channel: usize) -> usize {
    let song = self.tracks[song_idx];
    return song[channel].len() - 1;
  }

  pub fn is_track_restart(&self, song_idx: usize, channel: usize) -> bool {
    let len = self.get_track_len(song_idx, channel);
    let song = self.tracks[song_idx];
    let pattern_list = song[channel];
    let state = pattern_list[len] as usize;
    // In track:
    // 254: Pattern stop (finish song)
    // 255: Pattern restart
    return state == 255
  }

  pub fn get_song_channel(&self, song_idx: usize, channel: usize) -> Vec<Option<Vec<Note>>> {
    let mut patterns: Vec<Option<Vec<Note>>> = vec![];
    let len = self.get_track_len(song_idx, channel);
    for i in 0..len {
      patterns.push(self.get_pattern(song_idx, channel, i));
    }
    return patterns;
  }

  pub fn get_song(&self, song_idx: usize) -> [Vec<Option<Vec<Note>>>; 3] {
    let mut patterns: [Vec<Option<Vec<Note>>>; 3] = [vec![], vec![], vec![]];
    for channel in 0..3 {
      let mut p = self.get_song_channel(song_idx, channel);
      patterns[channel].append(&mut p);
    }
    return patterns;
  }

  pub fn print_pattern(&self, song_idx: usize, channel: usize, pattern_list_idx: usize) {
    let pattern = self.get_pattern(song_idx, channel, pattern_list_idx);
    println!("song:{}, ch:{}, idx:{} = {:?}",song_idx, channel, pattern_list_idx, pattern);
  }

  pub fn print_song(&self, song_idx: usize) {
    let patterns = self.get_song(song_idx);
    println!("{:#?}", patterns);
  }

  // try to detect
  pub fn instr2gm(&self, instrnum: u8) -> u8 {
    let instr = &self.instruments[instrnum as usize];
    let pw = instr.pulse_width;
    let noise = instr.ctrl_register & 0b1000_0000 == 0b1000_0000;
    let pulse = instr.ctrl_register & 0b0100_0000 == 0b0100_0000;
    let sawtooth = instr.ctrl_register & 0b0010_0000 == 0b0010_0000;
    let triangle = instr.ctrl_register & 0b0001_0000 == 0b0001_0000;
    let attack = instr.attack_and_decay >> 4;
    let decay = instr.attack_and_decay & 0b0000_1111;
    let sustain = instr.sustain_and_release >> 4;
    let release = instr.sustain_and_release & 0b0000_1111;

  
    if noise {
      println!("noise:{:2}, a:{:2},d:{:2},s:{:2},r:{:2},pw:{}",instrnum,attack,decay,sustain,release,pw);
      // return 119; // Synth Drum. XXX to channel 10 ?
    } else if pulse {
      println!("pulse:{:2}, a:{:2},d:{:2},s:{:2},r:{:2},pw:{}",instrnum,attack,decay,sustain,release,pw);
      // return 1; // Acoustic Grand Piano
    } else if sawtooth {
      println!("sawtooth:{:2}, a:{:2},d:{:2},s:{:2},r:{:2},pw:{}",instrnum,attack,decay,sustain,release,pw);
      // return 82; // Lead 2 (sawtooth wave)
    } else {
      println!("triangle:{:2}, a:{:2},d:{:2},s:{:2},r:{:2},pw:{}",instrnum,attack,decay,sustain,release,pw);
      // return 3; // Electric Grand Piano
    }

    /*
                                                  monty main theme:
    440 noise: 1, a: 0,d: 2,s: 0,r: 8,pw:2048  -- ch  1              1
     71 pulse: 2, a: 0,d: 9,s: 8,r: 0,pw:672   -- ch    2            2
    395 noise: 3, a: 0,d: 9,s: 0,r: 9,pw:512   -- ch  1,2
      5 pulse: 4, a: 0,d: 8,s: 5,r: 0,pw:2048  -- ch0,  2
      2 pulse: 5, a: 3,d:15,s:12,r: 0,pw:256   -- ch0,  2
      5 pulse: 6, a: 0,d: 4,s: 4,r: 0,pw:2048  -- ch0,               0

      1 pulse: 8, a: 0,d: 9,s: 7,r: 0,pw:2688  -- ch  1,2
      8 pulse: 9, a: 4,d:10,s: 6,r: 9,pw:2176  -- ch0,               0

      8 noise:11, a: 0,d:10,s: 0,r:10,pw:1920  -- ch0,1
     13 pulse:14, a: 0,d: 6,s: 0,r:10,pw:2048  -- ch0,1
      1 pulse:15, a: 1,d: 9,s: 7,r: 0,pw:2304  -- ch0                0

    */

    // if instrnum == 6 || instrnum == 9 || instrnum == 15 {
    //   return 3; // Acoustic Grand Piano
    // } else if instrnum == 1 || instrnum == 3 || instrnum == 8 || instrnum == 11 || instrnum == 14 {
    //   return 3; // Electric Grand Piano
    // } else {
    //   return 3; // Synth Drum. XXX to channel 10 ?
    // }

    return instrnum;

  }

  pub fn fixe_note(note: u8) -> u8 {
    // let mut rnote = note & 0b0_1111111;
    let mut rnote = note;
    // while rnote > 8*12 {
    //   println!("rnote too high: {}", rnote);
    //   rnote -= 12;
    // }
    return rnote;
  }

  // return track and delta length
  pub fn get_track_midly<'b>(&self, delta_start: u32, dest_chan: u8, p:&Vec<Option<Vec<Note>>>) -> (Track<'b>, u32) {
    let mut delta: u32 = delta_start;
    let mut track: Track = vec![];

    for sn in p {
      if let Some(notes) = sn {
        let mut instr_num = 0;
        let mut oldn: &Note = &Note::new();

        for n in notes {

          // TEST:remove note too distant
          // if oldn.value == 0 {
          //   oldn = n; // init / first pass
          // }
          // if i32::abs(oldn.value as i32 - n.value as i32) > 12 {  // 2 octaves
          //   continue;
          // }
          // end remove

          if n.instr != 0 {
            instr_num = n.instr;

            let mi = TrackEventKind::Midi {
              channel: num::u4::new(dest_chan),
              message: MidiMessage::ProgramChange { program: num::u7::new(self.instr2gm(instr_num)) },
            };
            let te = TrackEvent { delta: num::u28::new(delta), kind: mi };
            track.push(te);
          }

          // ----------- ON
          let mi = TrackEventKind::Midi {
            channel: num::u4::new(dest_chan),
            message: MidiMessage::NoteOn {
              key: num::u7::new(Self::fixe_note(n.value)),
              vel: num::u7::new(100),
            },
          };
          let te = TrackEvent { delta: num::u28::new(delta), kind: mi };
          track.push(te);

          delta = n.length as u32;

          // if n.release {
            // ----------- OFF
            let mi = TrackEventKind::Midi {
              channel: num::u4::new(dest_chan),
              message: MidiMessage::NoteOff {
                key: num::u7::new(Self::fixe_note(n.value)),
                vel: num::u7::new(80),
              },
            };
            let te = TrackEvent { delta: num::u28::new(delta), kind: mi };
            track.push(te);
          // }

        }
      }
    }
    return (track, delta);
  }

  pub fn get_song_midly<'b>(&self, song_idx: usize) -> Vec<Track<'b>> {
    let patterns = self.get_song(song_idx);
    let mut tracks: Vec<Track<'b>> = vec![ vec![], vec![], vec![] ];

    let mut delta_start = 0;
    for (ch, p) in patterns.iter().enumerate() {
      let (mut track, delta_next) = self.get_track_midly(delta_start, ch as u8, p);
      delta_start = delta_next;
      tracks[ch].append(&mut track);
      tracks[ch].push(TrackEvent { delta: num::u28::new(0), kind: TrackEventKind::Meta(MetaMessage::EndOfTrack) });
    }
    return tracks;
  }

  pub fn write_midi(&self, song_idx: usize) {
    let patterns = self.get_song_midly(song_idx);
    let header = Header {
      format: Format::Parallel,
      // timing: Timing::Metrical(num::u15::new(480)),
      timing: Timing::Timecode(Fps::Fps25, 4),
    };
    let mut smf = Smf::new(header);
    // let mut track0: Track = vec![];
    // let te0 = TrackEvent { delta: num::u28::new(0), kind: TrackEventKind::Meta(MetaMessage::Text(b"Music"))};
    // let te1 = TrackEvent { delta: num::u28::new(0), kind: TrackEventKind::Meta(MetaMessage::Copyright(b"Rob Hubbard"))};
    // track0.push(te0);
    // track0.push(te1);
    // smf.tracks.push(track0);

    for i in 0..3 {
      let te = &patterns[i];
      smf.tracks.push(te.to_vec());
    }
    smf.save("all.mid").unwrap();
  }

  pub fn write_all_patterns(&self) {
    let header = Header {
      format: Format::Parallel,
      // timing: Timing::Metrical(num::u15::new(480)),
      timing: Timing::Timecode(Fps::Fps25, 4),
    };
    for i in 0..self.patterns.len() {
      let p = vec![Vec::uncompress(self.patterns[i])];
      let (mut track, delta_next) = self.get_track_midly(0, 0, &p);
      track.push(TrackEvent { delta: num::u28::new(0), kind: TrackEventKind::Meta(MetaMessage::EndOfTrack) });
      let mut smf = Smf::new(header);
      smf.tracks.push(track);
      println!("### Pattern_{:03} : ",i);
      println!("{:#?}", p);
      smf.save(format!("pattern_{:03}.mid", i)).unwrap();
    }
  }

  pub fn write_channel_patterns(&self, song_idx: usize, channel: usize) {
    let header = Header {
      format: Format::SingleTrack,
      timing: Timing::Timecode(Fps::Fps25, 4),
    };
    let mut smf = Smf::new(header);

    let p = self.get_song_channel(song_idx, channel);
    let (mut track, delta_next) = self.get_track_midly(0, channel as u8, &p);
    track.push(TrackEvent { delta: num::u28::new(0), kind: TrackEventKind::Meta(MetaMessage::EndOfTrack) });
    let mut smf = Smf::new(header);
    smf.tracks.push(track);
    smf.save(format!("channel_{}.mid", channel)).unwrap();
  }

}


