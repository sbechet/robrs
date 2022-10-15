/*
  Original 6510 code: Rob Hubbard
  Rust rewrite: Sebastien Bechet
*/

use resid::Sid;
use std::ops::{BitAnd, BitOr};
pub mod residext;
use residext::{ SidExt, NOTE_FREQ_HEX };

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
  MontyOnTheRun,
  Commando,
}

pub struct RhSongs<'a> {
  pub musicplayer: MusicPlayer,
  pub total: usize,
  pub tracks: &'a [&'a[&'a [u8]; 3]],
  pub patterns: &'a [&'a [u8]],
  pub instruments: &'a [Instrument],
  pub soundfx: &'a [SoundFx; 16],
  pub resetspd: u8,

}

#[repr(u8)]
#[derive(PartialEq)]
enum SongState {
  Play = 0,
  Config = 0x40,
  Stop = 0x80,
  ConfigStop = 0xC0,
}

impl BitAnd<SongState> for u8 {
  type Output = Self;
  fn bitand(self, rhs: SongState) -> Self::Output {
    self & rhs as u8
  }
}

#[repr(u8)]
enum Note1 {
  // NoteMask      = 0b0001_1111, // length of the note 0-31
  ReleaseMask = 0b0010_0000, // 0:no release
  AppendedMask  = 0b0100_0000, // signal appended note
  ChgIntrOrPorta        = 0b1000_0000, // new instrument or portamento coming up (2 or 3 bytes for note)
  EndOfPattern  = 0b1111_1111,
}

impl BitAnd<Note1> for u8 {
  type Output = Self;
  fn bitand(self, rhs: Note1) -> Self::Output {
      self & rhs as u8
  }
}

#[repr(u8)]
enum Note2 {
  TypeMask              = 0b1000_0000, // 1 = portamento, 0 = instrument
  // InstrNrMask           = 0b0111_1111, // for instrnr 0..128
  PortamentoMask        = 0b0111_1110, // for portamento
  PortamentoUpDownMask  = 0b0000_0001, // 0 for up, 1 for down
}

impl BitAnd<Note2> for u8 {
  type Output = Self;
  fn bitand(self, rhs: Note2) -> Self::Output {
      self & rhs as u8
  }
}

#[repr(u8)]
enum Fx {
  Stop = 0b1000_0000,
  Config = 0b0100_0000,
  IdxMask = 0b0000_1111,
  // IncDecMask = 0b0010_0000, // 1 = INC, 0 = DEC sfx_note
}

impl BitAnd<Fx> for u8 {
  type Output = Self;
  fn bitand(self, rhs: Fx) -> Self::Output {
      self & rhs as u8
  }
}

impl BitOr<Fx> for u8 {
  type Output = Self;
  fn bitor(self, rhs: Fx) -> Self::Output {
      self | rhs as u8
  }
}

pub struct RhPlayer<'a> {
  sid:        &'a mut Sid,
  songs:      &'a RhSongs<'a>,
  instr_pulse_width: Vec<u16>, // HACK
  current_tracks: &'a[&'a[u8]; 3],
  posoffset : [u8; 3],
  patoffset : [u8; 3],
  lengthleft: [u8; 3],  // length left (see first byte)
  savelnthcc: [u8; 3],  // first byte (full)
  voicectrl : [u8; 3],
  notenum   : [u8; 3],
  instrnr   : [u8; 3],
  appendfl  : u8,
  pulsedelay: [u8; 3],
  pulsedir  : [u8; 3],
  speed     : u8,
  resetspd  : u8,
  mstatus   : u8,
  savefreq  : [u16; 3],
  portaval  : [u8; 3],
  counter   : u8,

  engine_up: bool,
  engine_fx_play: u8, // 0x80 = end of soundfx, 0x40 = configure soundfx, low quartet = idx
  engine_song_playing: bool,
  sfx_note: u8,
  sfx_counter: u8,
  sfx_note_dest: u8,
  sfx_note_delta: u8,
  sfx_voices_ctrl: u8,
  sfx_voice0_ctrl: u8,
  sfx_voice1_ctrl: u8,
  /*
  0bABCC_DDDD
  A  : 0 : play current music freq at end of CCCC counter
  B
  CC : 0b10 : INC else DEC voice0 freq low part
  DDDD : IncDec counter
  */
  sfx_incdec: u8,
}


impl<'a> RhPlayer<'a> {
  pub fn new(sid: &'a mut Sid, songs: &'a RhSongs<'a>) -> Self {
    RhPlayer {
      sid:        sid,
      songs:      songs,
      instr_pulse_width: vec![0; songs.instruments.len()],  // HACK
      current_tracks: songs.tracks[0],
      posoffset : [0,0,0],
      patoffset : [0,0,0],
      lengthleft: [0,0,0],
      savelnthcc: [0,0,0],
      voicectrl : [0,0,0],
      notenum   : [0,0,0],
      instrnr   : [0,0,0],
      appendfl  : 0,  // 0xff: no append
      pulsedelay: [0,0,0],
      pulsedir  : [0,0,0],
      speed     : 0,
      resetspd  : songs.resetspd,
      mstatus   : SongState::Stop as u8,
      savefreq  : [0,0,0],
      portaval  : [0,0,0],
      counter   : 0,

      engine_up: true,
      engine_fx_play: 0xff,
      engine_song_playing: true,

      sfx_note: 0,
      sfx_counter: 0,
      sfx_note_dest: 0,
      sfx_note_delta: 0,
      sfx_voices_ctrl: 0,
      sfx_voice0_ctrl: 0,
      sfx_voice1_ctrl: 0,
      /*
      0bAABB_CCCC
      AA : 0b00 : play current music freq at end of CCCC counter
      BB : 0b10 : INC else DEC voice0 freq low part
      CCCC : IncDec counter
      */
      sfx_incdec: 0,
    }
  }

  fn assert_high_note(note: u8) -> u8 {
    let mut rnote= note;
    if note >= 8*12 {
      println!("note too high? {}", note);
    }
    // Stupid hack but what can i do?
    while rnote >= 8*12 {
      rnote -= 12;
    }
    return rnote;
  }

  fn get_new_note(&mut self, track_idx: usize) {
    // get the pattern number/cc from this position
    let current_track = self.current_tracks[track_idx];
    let mut patnum = current_track[self.posoffset[track_idx] as usize] as usize;
    if patnum == 0xff {
      // RESTART
      self.lengthleft[track_idx] = 0;
      self.posoffset[track_idx] = 0;
      self.patoffset[track_idx] = 0;
      patnum = current_track[0] as usize;
    } else if patnum == 0xfe {
      // STOP
      self.mstatus = SongState::ConfigStop as u8;
      return;
    }
    // GET NOTE DATA
    let current_pattern = self.songs.patterns[patnum];
    self.portaval[track_idx] = 0;
    let mut pattern_idx = self.patoffset[track_idx] as usize;
    self.appendfl = 0xff;
    // 1st byte is the length of the note 0-31
    // 0x20 bit5 signals no release (see sndwork)
    // 0x40 bit6 signals appended note
    // 0x80 bit7 signals a new instrument or portamento coming up
    let templnthcc = current_pattern[pattern_idx]; // GET
    self.savelnthcc[track_idx] = templnthcc;
    self.lengthleft[track_idx] = templnthcc & 0x1f;
    if templnthcc & Note1::AppendedMask == Note1::AppendedMask as u8 {
      self.appendfl -= 1; // append note
    } else {
      pattern_idx += 1;
      self.patoffset[track_idx] += 1;
      if templnthcc & Note1::ChgIntrOrPorta == Note1::ChgIntrOrPorta as u8 {
        // next byte is a new instrument or portamento coming up
        let instr_or_portamento = current_pattern[pattern_idx]; // GET
        if instr_or_portamento & Note2::TypeMask == Note2::TypeMask as u8 {
          self.portaval[track_idx] = instr_or_portamento;
        } else {
          self.instrnr[track_idx] = instr_or_portamento;
          /* HACK because we can't write original instrument */
          let instr = &self.songs.instruments[instr_or_portamento as usize];
          // Only first access
          if self.instr_pulse_width[instr_or_portamento as usize] == 0 {
            self.instr_pulse_width[instr_or_portamento as usize] = instr.pulse_width;
          }
          /* HACK END */
        }
        pattern_idx += 1;
        self.patoffset[track_idx] += 1;
      }
      // next byte is the note of the note: get the 'base frequency' here
      let mut note = RhPlayer::assert_high_note(current_pattern[pattern_idx]); // GET
      self.notenum[track_idx] = note;

      if self.engine_song_playing {
        let freq = NOTE_FREQ_HEX[note as usize];
        self.savefreq[track_idx] = freq;
        self.sid.set_freq(track_idx, freq);
      }
    }
    // fetch all the initial values from the instrument data structure
    let instr = &self.songs.instruments[self.instrnr[track_idx] as usize];
    if self.engine_song_playing {
      self.sid.set_ctrl(track_idx, instr.ctrl_register & self.appendfl);
      self.sid.set_pw(track_idx, self.instr_pulse_width[self.instrnr[track_idx] as usize]); // instr.pulse_width; // HACK: use self.instr_pulse_width[]
      self.sid.set_ad(track_idx, instr.attack_and_decay);
      self.sid.set_sr(track_idx, instr.sustain_and_release);

    }
    self.voicectrl[track_idx] = instr.ctrl_register;
    // last byte checks for the end of pattern if eop found, inc the position and reset patoffset for new pattern
    pattern_idx += 1;
    self.patoffset[track_idx] += 1;
    if current_pattern[pattern_idx] == Note1::EndOfPattern as u8 {
      self.patoffset[track_idx] = 0;
      self.posoffset[track_idx] += 1;
    }
  }

  fn release(&mut self, track_idx: usize) {
    // set off a release when the length of the note is exceeded
    // bit5 of the 1st note-byte can specify for no release
    if self.savelnthcc[track_idx] & Note1::ReleaseMask == 0 && self.lengthleft[track_idx] == 0 {
      // kill adsr
      self.sid.set_ctrl(track_idx, self.voicectrl[track_idx] & 0b1111_1110);
      self.sid.set_ad(track_idx, 0);
      self.sid.set_sr(track_idx, 0);
    }
  }

  fn vibrato(&mut self, track_idx: usize) -> u8 {
    let mut rvalue = 0;
    let instr = &self.songs.instruments[self.instrnr[track_idx] as usize];
    if instr.vibrato_depth == 0 {
      return rvalue;
    }
    // the counter's turned into an oscillating value (01233210)
    let mut oscilatval = self.counter & 7;
    if oscilatval > 3 {
      oscilatval ^= 7;
    }
    let mut note = RhPlayer::assert_high_note(self.notenum[track_idx]); if note==8*12-1 { note=8*12-2 };
    let freq0 = NOTE_FREQ_HEX[note as usize];
    let freq1 = NOTE_FREQ_HEX[note as usize+1];
    let mut freq_diff = freq1 - freq0;
    // freq_diff / 2u16.pow(instr.vibrato_depth as u32);
    for _ in 0..instr.vibrato_depth {
      freq_diff /= 2;
    }

    let mut tmpvfrq = freq0;
    if self.savelnthcc[track_idx] & 0x1f > 7 {
      // vibrato if note length >= 8
      for _ in 0..oscilatval {
        if tmpvfrq as u32  + freq_diff as u32 > 65535 {
          // XXX
          println!("overflow for pulse_width_timbe / instr.fx&8 see Commando code?");
          rvalue = 1;
        }
        tmpvfrq += freq_diff;
      }
    }
    self.sid.set_freq(track_idx, tmpvfrq);
    return rvalue;
  }

  // pulse-width timbre routine depending on the control/speed byte in the instrument datastructure, 
  // the pulse width is of course inc/decremented to produce timbre
  // strangely the delay value is also the size of the inc/decrements
  fn pulse_width_timbre(&mut self, track_idx: usize, freq_overflow: u8) {
    let instr_idx = self.instrnr[track_idx] as usize;
    let instr = &self.songs.instruments[instr_idx];
    let pulsevalue = instr.pulse_speed;

    // See Commando code
    if instr.fx&8 == 0 {
      let new_pulse = instr.pulse_width + pulsevalue as u16 + freq_overflow as u16;
      self.instr_pulse_width[instr_idx] = new_pulse;
      self.sid.set_pw(track_idx, new_pulse);
      return;
    }

    if pulsevalue == 0 {
      return;
    }

    if self.pulsedelay[track_idx] != 0 {
      self.pulsedelay[track_idx] -= 1;
    } else {
      // reset pulsedelay
      self.pulsedelay[track_idx] = pulsevalue & 0b000_11111;
      let pulsespeed = (pulsevalue & 0b111_00000) as u16;
      let new_pulse = if self.pulsedir[track_idx] == 0 {
        let new_pulse = self.instr_pulse_width[instr_idx] + pulsespeed;
        if new_pulse & 0x0f00 == 0x0e00 {
          self.pulsedir[track_idx] += 1;
        }
        new_pulse
      } else {
        // pulse down
        let new_pulse = self.instr_pulse_width[instr_idx] - pulsespeed;
        if new_pulse & 0x0f00 == 0x0800 {
          // reaches min
          self.pulsedir[track_idx] -= 1;
        }
        new_pulse
      };
      // dump pulse width to chip and back into the instr data str
      // instr.pulse_width = new_pulse;  // must clone instr? :(
      self.instr_pulse_width[instr_idx] = new_pulse;  // HACK
      self.sid.set_pw(track_idx, new_pulse);
    }
  }


  // portemento routine
  fn portamento(&mut self, track_idx: usize) {
    if self.portaval[track_idx] == 0 {
      return;
    }
    // toad unwanted bits
    let tempstore = (self.portaval[track_idx] & Note2::PortamentoMask) as u16;
    if self.portaval[track_idx] & Note2::PortamentoUpDownMask == 0 {
      // portamento up
      // add portval to current frequency
      self.savefreq[track_idx] += tempstore;
    } else {
      // portamento down
      self.savefreq[track_idx] -= tempstore;
    }
  }

  // drums
  fn drums(&mut self, track_idx: usize) {
    let instr = &self.songs.instruments[self.instrnr[track_idx] as usize];
    // check if drums needed this instr
    // don't bother if freq can't go any lower or if the note has finished

    //************ bit0:FX
    if instr.fx & 1 != 0 && self.savefreq[track_idx]>>8 != 0 && self.lengthleft[track_idx] != 0 {
      let mut vctrl = self.voicectrl[track_idx] & 0xfe;
      self.sid.set_freq(track_idx, self.savefreq[track_idx]);
      // check if this is the first vbl for this instrument-note
      if self.lengthleft[track_idx] >= self.savelnthcc[track_idx] & 0x1f {
        // first time: set noise
        vctrl = 0x80;
      } else {
        // next vbls..
        self.savefreq[track_idx] -= 0x0100;
        if vctrl == 0 {
          self.sid.set_freq(track_idx, self.savefreq[track_idx]);
          // set noise
          vctrl = 0x80;
        }
      }
      self.sid.set_ctrl(track_idx, vctrl);
    }
  }

  fn skydive(&mut self, track_idx: usize) {
    let instr = &self.songs.instruments[self.instrnr[track_idx] as usize];
    //************ bit1:Skydive - a long portamento-down from the note to zerofreq
    // check if skydive needed this instr every 2nd vbl && check if skydive already complete
    if instr.fx&2 != 0 && self.counter&1 != 0 && self.savefreq[track_idx]>>8!= 0 {
      match self.songs.musicplayer {
        MusicPlayer::MontyOnTheRun => {
          self.savefreq[track_idx] -= 0x0100;
        },
        MusicPlayer::Commando => {
          if self.savefreq[track_idx] as usize + 0x0200 < 0x10000 {
            self.savefreq[track_idx] += 0x0200;
          }
        },
      }
      self.sid.set_freq(track_idx, self.savefreq[track_idx]);
    }
  }

  fn octave_arpeggio(&mut self, track_idx: usize) {
    let instr = &self.songs.instruments[self.instrnr[track_idx] as usize];
    //************ bit2:instrfx is an octave arpeggio pretty tame huh?
    // check if arpt needed
    if instr.fx&4 != 0 {
      // only 2 arpt values
      let mut note = if self.counter&1 == 0 {
        // even, note
        self.notenum[track_idx]
      } else {
        // odd, change octave
        self.notenum[track_idx] + 12
      };
      // dump the corresponding frequencies
      note = RhPlayer::assert_high_note(note);
      self.sid.set_freq(track_idx, NOTE_FREQ_HEX[note as usize]);
    }
  }

  fn soundfx(&mut self) {
    let mut return_value = true;
    if !self.engine_up || self.engine_fx_play&Fx::Stop == Fx::Stop as u8 {
      return;
    }
    if self.engine_fx_play & Fx::Config == Fx::Config as u8 {
      self.soundfx_copy();
      println!("note:{}, note_dest:{}, note_delta:{}, ", self.sfx_note, self.sfx_note_dest, self.sfx_note_delta);
    }

    if self.sfx_counter > 0 {
      self.sfx_counter -= 1;
    } else {
      self.sfx_counter = self.sfx_incdec & 0b0000_1111;
      if self.sfx_note == self.sfx_note_dest {
        self.sid.set_ctrl(0, 0);
        self.sid.set_ctrl(1, 0);
        self.engine_fx_play = 0xff;

      } else {

        // XXX START SPECIAL: see 0x8555
        if self.sfx_incdec & 0b00110000 != 0b00100000 {
          // DEC assembly mnemonic
          self.sfx_note -= 1;
        } else {
          // INC assembly mnemonic
          self.sfx_note += 1;
        }
        // XXX END SPECIAL

        if self.sfx_incdec & 0b1000_0000 != 0b1000_0000 {
          if self.sfx_incdec & 0b0100_0000 != 0b0100_0000 {
            self.sid.set_freq(0, NOTE_FREQ_HEX[self.sfx_note as usize]);
          }
          let note :usize = if self.sfx_note > self.sfx_note_delta {
            (self.sfx_note - self.sfx_note_delta) as usize
          } else {
            0
          };
          self.sid.set_freq(1, NOTE_FREQ_HEX[note]);
        }
        if self.sfx_voices_ctrl & 0b1000_0000 == 0b1000_0000 {
          self.sfx_voice0_ctrl ^= 1;
          self.sid.set_ctrl(0, self.sfx_voice0_ctrl);
        }
        if self.sfx_voices_ctrl & 0b0100_0000 == 0b0100_0000 {
          self.sfx_voice1_ctrl ^= 1;
          self.sid.set_ctrl(1, self.sfx_voice1_ctrl);
        }
      }
    }
  }

  fn contplay(&mut self) {
    // slowdown songs from resetspd VBL
    if self.speed == 0 {
      self.speed = self.resetspd;
    } else {
      self.speed -= 1;
    }

    for track_idx in 0..3 {
      // check whether a new note is needed 
      if self.speed == self.resetspd {
        if self.lengthleft[track_idx] == 0 {
          self.get_new_note(track_idx);
        } else {
          self.lengthleft[track_idx] -= 1;
          if self.engine_song_playing {
            self.release(track_idx);
            let freq_overflow = self.vibrato(track_idx);
            self.pulse_width_timbre(track_idx,  freq_overflow);
            self.portamento(track_idx);
            self.drums(track_idx);
            self.skydive(track_idx);
            self.octave_arpeggio(track_idx);
          }
        }
      } else {
        if self.engine_song_playing {
          let freq_overflow = self.vibrato(track_idx);
          self.pulse_width_timbre(track_idx,  freq_overflow);
          self.portamento(track_idx);
          self.drums(track_idx);
          self.skydive(track_idx);
          self.octave_arpeggio(track_idx);
        }
      }
    }
  }

  // VBL call
  // finish: return false;
  pub fn play(&mut self) {
    self.counter = (self.counter + 1) & 7;  // only three bits used...


    if self.mstatus & SongState::Stop == SongState::Stop as u8 {
      if self.mstatus & SongState::Config == SongState::Config as u8 {
        self.sid.set_ctrl(0, 0);
        self.sid.set_ctrl(1, 0);
        self.sid.set_ctrl(2, 0);
        self.mstatus = SongState::Stop as u8;
      }
    } else {
      if self.mstatus & SongState::Config == SongState::Config as u8 {
        self.counter = 0;
        self.posoffset = [0, 0, 0];
        self.patoffset = [0, 0, 0];
        self.lengthleft = [0, 0, 0];
        self.notenum = [0, 0, 0];
        self.mstatus = SongState::Play as u8;
      }

      self.contplay(); // false = end

      self.engine_song_playing = true;
      if self.engine_up && self.engine_fx_play & Fx::Stop != Fx::Stop as u8{
        self.engine_song_playing = false;
      }
    }
    self.engine_song_playing = true;
    self.soundfx();
  }

  fn soundfx_copy(&mut self) {
    self.sid.set_ctrl(0, 0);
    self.sid.set_ctrl(1, 0);
    self.sfx_counter = 0;
    self.engine_fx_play = self.engine_fx_play & Fx::IdxMask; // Remove Stop, Configure and IncDec information
    let soundfx = &self.songs.soundfx[self.engine_fx_play as usize];
    self.sfx_incdec = soundfx.incdec;
    self.sfx_note = RhPlayer::assert_high_note(soundfx.voice0.freq as u8);
    self.sfx_note_dest = RhPlayer::assert_high_note(soundfx.sfx_note_dest);

    self.sfx_voices_ctrl = soundfx.voice1.freq as u8;
    self.sfx_note_delta = self.sfx_voices_ctrl & 0b0011_1111;

    self.sfx_voice0_ctrl = soundfx.voice0.ctrl;
    self.sfx_voice1_ctrl = soundfx.voice1.ctrl;

    self.sid.set_all(0, soundfx.voice0.freq, soundfx.voice0.pulse_width, soundfx.voice0.ctrl, soundfx.voice0.attack_and_decay_len, soundfx.voice0.sustain_vol_and_release_len);
    self.sid.set_all(1, soundfx.voice1.freq, soundfx.voice1.pulse_width, soundfx.voice1.ctrl, soundfx.voice1.attack_and_decay_len, soundfx.voice1.sustain_vol_and_release_len);

  }

  pub fn engine_up(&mut self) {
    self.engine_up = true;
  }

  pub fn engine_down(&mut self) {
    self.engine_up = false;
    self.sid.set_ctrl(0, 0);
    self.sid.set_ctrl(1, 0);
    self.engine_fx_play = 0xFF;  // Fx::Stop | Fx::Config | 0b0011_1111;
  }

  pub fn set_soundfx(&mut self, soundfx: usize) {
    if ! self.engine_up {
      self.engine_fx_play = 0xFF;  // Fx::Stop | Fx::Config | 0b0011_1111;
    } else {
      self.sid.set_modvol(15);
      self.engine_fx_play = soundfx as u8 | Fx::Config;
    }
  }

  pub fn init(&mut self, song: usize) {
    self.sid.set_modvol(15);
    if song < self.songs.tracks.len() {
      // song
      self.current_tracks = self.songs.tracks[song];
      self.sid.set_ctrl(0, 0);
      self.sid.set_ctrl(1, 0);
      self.sid.set_ctrl(2, 0);
      self.sid.set_modvol(15);
      self.mstatus = SongState::Config as u8;
    } else {
      // soundfx
      self.stop_music();
      self.set_soundfx(song - self.songs.tracks.len());
    }
  }

  pub fn stop_music(&mut self) {
    self.mstatus = SongState::ConfigStop as u8;
  }

  pub fn sample(&mut self, delta: u32, buffer: &mut [i16], interleave: usize) -> (usize, u32) {
    self.sid.sample(delta, buffer, interleave)
  }

  pub fn get_sid_regs(&self) -> Option<[(u16,u16,u8,u8,u8); 3]> {
    let v0 = self.sid.get_all(0).unwrap();
    let v1 = self.sid.get_all(1).unwrap();
    let v2 = self.sid.get_all(2).unwrap();
    Some([v0, v1, v2])
  }

}