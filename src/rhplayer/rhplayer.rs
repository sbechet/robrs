/*
  Original 6510 code: Rob Hubbard
  Rust rewrite: Sebastien Bechet
*/

use super::residext::{SidExt, NOTE_FREQ_HEX};
use super::rhsongs::Instrument;
use resid::Sid;
use std::ops::{BitAnd, BitOr};

use super::rhsongs::RhSongs;

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
    ReleaseMask = 0b0010_0000,    // 0:no release
    AppendedMask = 0b0100_0000,   // signal appended note
    ChgIntrOrPorta = 0b1000_0000, // new instrument or portamento coming up (2 or 3 bytes for note)
    EndOfPattern = 0b1111_1111,
}

impl BitAnd<Note1> for u8 {
    type Output = Self;
    fn bitand(self, rhs: Note1) -> Self::Output {
        self & rhs as u8
    }
}

#[repr(u8)]
enum Note2 {
    TypeMask = 0b1000_0000, // 1 = portamento, 0 = instrument
    // InstrNrMask           = 0b0111_1111, // for instrnr 0..128
    PortamentoMask = 0b0111_1110,       // for portamento
    PortamentoUpDownMask = 0b0000_0001, // 0 for up, 1 for down
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
    sid: &'a mut Sid,
    songs: &'a RhSongs<'a>,
    instr_pw: Vec<u16>, // HACK
    current_tracks: &'a [&'a [u8]; 3],
    posoffset: [u8; 3],
    patoffset: [u8; 3],
    patloop: [u8; 3],
    lengthleft: [u8; 3], // length left (see first byte)
    savelnthcc: [u8; 3], // first byte (full)
    voicectrl: [u8; 3],
    notenum: [u8; 3],
    instrnr: [u8; 3],
    appendfl: u8, // ctrl mask
    pulsedelay: [u8; 3],
    pulsedir: [u8; 3],
    speed: u8,
    resetspd: u8,
    counter_note: i8,
    mstatus: u8,
    savefreq: [u16; 3],
    portaval: [u8; 3],
    portaval20: [i16; 3],
    counter: u8,

    vibincdec: [i8; 3],     // Spellbound code : 0=inc, -1=dec
    vibosc: [i8; 3],        // Spellbound code : oscilator value 0..depth
    vib_osc_depth: [i8; 3], // Spellbound code : osc depth

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
            sid: sid,
            songs: songs,
            instr_pw: vec![0; songs.instruments.len()],
            current_tracks: songs.channels[0],
            posoffset: [0, 0, 0],
            patoffset: [0, 0, 0],
            patloop: [1, 1, 1],
            lengthleft: [0, 0, 0],
            savelnthcc: [0, 0, 0],
            voicectrl: [0, 0, 0],
            notenum: [0, 0, 0],
            instrnr: [0, 0, 0],
            appendfl: 0, // 0xff: no append
            pulsedelay: [0, 0, 0],
            pulsedir: [0, 0, 0],
            speed: 0,
            resetspd: songs.resetspd,
            counter_note: 4,
            mstatus: SongState::Stop as u8,
            savefreq: [0, 0, 0],
            portaval: [0, 0, 0],
            portaval20: [0, 0, 0],
            counter: 0,

            vibincdec: [0, 0, 0],     // Spellbound code
            vibosc: [0, 0, 0],        // Spellbound code
            vib_osc_depth: [0, 0, 0], // Spellbound code

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

    fn assert_high_note(&mut self, track_idx: usize, note: u8) -> u8 {
        if note >= 8 * 12 {
            // all from commando
            let rnote = match note {
                96 => 0,   // self.regoffsets[0], for crazycomet
                97 => 0,   // self.regoffsets[0], for commando
                98 => 12,  // 8 for crazycomet, 16 for commando
                100 => 3,  // 3 for commando, // self.patoffset[0],
                104 => 65, // self.voicectrl[1],    // Good for Monty on the Run, Commando
                105 => 65, // self.voicectrl[2],
                107 => 6,  // self.instrnr[0],
                127 => 0,
                _ => note & 0b0011_1111,
            };
            println!(
                "INFO{}: octave {} too high (note {}), avoiding overflow: {}->{}",
                track_idx,
                note / 12,
                note,
                note,
                rnote
            );
            return rnote;
        }
        return note;
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
            self.patloop[track_idx] = 1;
            patnum = current_track[0] as usize;
        } else if patnum == 0xfe {
            // STOP
            self.mstatus = SongState::ConfigStop as u8;
            return;
        }
        // GET NOTE DATA
        let current_pattern = self.songs.tracks[patnum];
        self.portaval[track_idx] = 0;
        self.portaval20[track_idx] = 0;
        let mut pattern_idx = self.patoffset[track_idx] as usize;
        self.appendfl = 0b1111_1111;
        // 1st byte is the length of the note 0-31
        // 0x20 bit5 signals no release (see sndwork)
        // 0x40 bit6 signals appended note
        // 0x80 bit7 signals a new instrument or portamento coming up
        let templnthcc = current_pattern[pattern_idx]; // GET
        self.savelnthcc[track_idx] = templnthcc;
        self.lengthleft[track_idx] = templnthcc & 0x1f;

        if let 15 = self.songs.version {
            let mut v = 119 - self.posoffset[2];
            if v >= 15 {
                v = 15;
            }
            self.sid.set_modvol(v);
        }

        if templnthcc & Note1::AppendedMask == Note1::AppendedMask as u8 {
            self.appendfl = 255 - 1; // append note (begin dec)
        } else {
            pattern_idx += 1;
            self.patoffset[track_idx] += 1;
            if templnthcc & Note1::ChgIntrOrPorta == Note1::ChgIntrOrPorta as u8 {
                // next byte is a new instrument or portamento coming up
                let instr_or_portamento = current_pattern[pattern_idx]; // GET
                match self.songs.version {
                    10 => {
                        if instr_or_portamento & Note2::TypeMask == Note2::TypeMask as u8 {
                            self.portaval[track_idx] = instr_or_portamento;
                        } else {
                            self.instrnr[track_idx] = instr_or_portamento;
                            let instr = &self.songs.instruments[instr_or_portamento as usize];
                            self.instr_pw[instr_or_portamento as usize] = instr.pulse_width;
                        }
                    }
                    15 => {
                        if instr_or_portamento & Note2::TypeMask == Note2::TypeMask as u8 {
                            self.instrnr[track_idx] = instr_or_portamento;
                            let instr = &self.songs.instruments[instr_or_portamento as usize];
                            self.instr_pw[instr_or_portamento as usize] = instr.pulse_width;
                        }
                    }
                    _ => {
                        if instr_or_portamento & Note2::TypeMask == Note2::TypeMask as u8 {
                            let sign = (instr_or_portamento & 0b0_1_000000) == 0b0_1_000000;
                            self.portaval20[track_idx] =
                                ((instr_or_portamento & 0b00_111111) as i16) << 8;
                            pattern_idx += 1;
                            self.patoffset[track_idx] += 1;
                            self.portaval20[track_idx] |= current_pattern[pattern_idx] as i16;
                            if sign {
                                self.portaval20[track_idx] = -self.portaval20[track_idx];
                            }
                        } else {
                            let intrnum = instr_or_portamento & 0b0111_1111;
                            self.instrnr[track_idx] = intrnum;
                            let instr = &self.songs.instruments[intrnum as usize];
                            self.instr_pw[intrnum as usize] = instr.pulse_width;
                        }
                    }
                }
                pattern_idx += 1;
                self.patoffset[track_idx] += 1;
            }
            // next byte is the note of the note: get the 'base frequency' here
            // self.sid.print_note(current_pattern[pattern_idx]); print!(" {} {}", self.instrnr[track_idx], 1+self.lengthleft[track_idx]);

            let note = self.assert_high_note(track_idx, current_pattern[pattern_idx] & 0b0111_1111);
            self.notenum[track_idx] = note;

            if self.engine_song_playing {
                let freq = NOTE_FREQ_HEX[note as usize];
                self.savefreq[track_idx] = freq;
                self.sid.set_freq(track_idx, freq);
            }
        }

        let reset_effect = match self.songs.version {
            30 => current_pattern[pattern_idx] & 0x80 == 0x80,
            _ => true,
        };

        // fetch all the initial values from the instrument data structure
        let instr = &self.songs.instruments[self.instrnr[track_idx] as usize];
        if self.engine_song_playing {
            self.sid
                .set_ctrl(track_idx, instr.ctrl_register & self.appendfl);
            if reset_effect {
                self.sid
                    .set_pw(track_idx, self.instr_pw[self.instrnr[track_idx] as usize]); // instr.pulse_width; // HACK: use self.instr_pw[]
                self.sid.set_ad(track_idx, instr.attack_and_decay);
                self.sid.set_sr(track_idx, instr.sustain_and_release);
                // reset some effects
                if let 15 = self.songs.version {
                    self.pulsedir[track_idx] = 0;
                    self.pulsedelay[track_idx] = 0;
                }
            }
        }

        self.voicectrl[track_idx] = instr.ctrl_register;

        // last byte checks for the end of pattern if eop found, inc the position and reset patoffset for new pattern
        pattern_idx += 1;
        self.patoffset[track_idx] += 1;

        match self.songs.version {
            30 => {
                if current_pattern[pattern_idx] == Note1::EndOfPattern as u8 {
                    println!(
                        "{} patloop={}, posoffset={}",
                        track_idx, self.patloop[track_idx], self.posoffset[track_idx]
                    );
                    self.patoffset[track_idx] = 0;
                    self.patloop[track_idx] -= 1;
                    if self.patloop[track_idx] == 0 {
                        self.posoffset[track_idx] += 1;
                        let patnum = current_track[self.posoffset[track_idx] as usize];
                        if patnum != 0xff && patnum != 0xfe {
                            self.patloop[track_idx] = patnum;
                            self.posoffset[track_idx] += 1;
                        }
                    }
                }
            }
            _ => {
                if current_pattern[pattern_idx] == Note1::EndOfPattern as u8 {
                    self.patoffset[track_idx] = 0;
                    self.posoffset[track_idx] += 1;
                }
            }
        };
    }

    fn release(&mut self, track_idx: usize) {
        // set off a release when the length of the note is exceeded
        // bit5 of the 1st note-byte can specify for no release
        if self.savelnthcc[track_idx] & Note1::ReleaseMask == 0 && self.lengthleft[track_idx] == 0 {
            // kill adsr
            self.sid
                .set_ctrl(track_idx, self.voicectrl[track_idx] & 0b1111_1110);
            self.sid.set_ad(track_idx, 0);
            self.sid.set_sr(track_idx, 0);
        }
    }

    fn vibrato_spellbound_osc2(&mut self, track_idx: usize) {
        self.vibosc[track_idx] += 1;
        if self.vib_osc_depth[track_idx] < self.vibosc[track_idx] {
            self.vibosc[track_idx] = self.vib_osc_depth[track_idx];
            self.vibincdec[track_idx] -= 1;
            self.vibosc[track_idx] -= 1;
        }
    }

    // oscillator: 0..vib_osc_depth..0 ...
    fn vibrato_update_osc(&mut self, track_idx: usize) -> i8 {
        if self.vibincdec[track_idx] == 0 {
            self.vibrato_spellbound_osc2(track_idx);
        } else {
            self.vibosc[track_idx] -= 1;
            if self.vibosc[track_idx] == 0 {
                self.vibincdec[track_idx] += 1;
                if self.vibincdec[track_idx] < 0 {
                    self.vibrato_spellbound_osc2(track_idx);
                }
            }
        }
        // println!("osc{}:{}", track_idx, self.vibosc[track_idx]);
        return self.vibosc[track_idx];
    }

    fn vibrato_spellbound(&mut self, instr: &Instrument, track_idx: usize) -> u16 {
        self.vib_osc_depth[track_idx] = (instr.vibrato_depth as i8 & 0b0_1111_000) >> 3;
        let vib_div = instr.vibrato_depth & 0b0000_111;

        // println!("{}: osc_depth:{}, vibr:{}", track_idx, self.vib_osc_depth[track_idx], vib_div);

        let osc = self.vibrato_update_osc(track_idx);

        // XXX in V2 code only if self.portaval[track_idx] == 0
        let note = self.assert_high_note(track_idx, self.notenum[track_idx]);
        let freq0 = if note != 0 {
            NOTE_FREQ_HEX[note as usize - 1]
        } else {
            0x106
        };
        let freq1 = NOTE_FREQ_HEX[note as usize];

        let mut temp_vdif_freq_diff = freq1 - freq0;
        let mut temp_freq_diff = freq1;

        // freq_diff / 2u16.pow(instr.vibrato_depth as u32);
        for _ in 0..vib_div {
            temp_vdif_freq_diff /= 2;
        }

        let vib_osc_ct = self.vib_osc_depth[track_idx] >> 1;
        for _ in 0..vib_osc_ct {
            temp_freq_diff -= temp_vdif_freq_diff;
        }
        if self.savelnthcc[track_idx] & 0b000_11111 != 0 {
            // > 2 for v2 code
            for _ in 0..osc {
                temp_freq_diff += temp_vdif_freq_diff;
            }
        }
        // XXX in V2 code: self.savefreq[track_idx] = temp_freq_diff;
        // println!("uknow{}: {},{}", track_idx, freq1-freq0, temp_freq_diff as i16-freq1 as i16);
        return temp_freq_diff;
    }

    fn vibrato(&mut self, track_idx: usize) {
        let instr = &self.songs.instruments[self.instrnr[track_idx] as usize];
        if instr.vibrato_depth == 0 {
            return;
        }
        let freq = match self.songs.version {
            10 => {
                // the counter's turned into an oscillating value (01233210)
                let osc = self.counter as i8 & 7;
                let oscilatval = if osc > 3 { osc ^ 7 } else { osc };
                let note = self.assert_high_note(track_idx, self.notenum[track_idx]);
                let freq0 = NOTE_FREQ_HEX[note as usize];
                let freq1 = NOTE_FREQ_HEX[note as usize + 1]; // See array hack to remove overflow
                let mut freq_diff = freq1 - freq0;
                // freq_diff / 2u16.pow(instr.vibrato_depth as u32);
                for _ in 0..instr.vibrato_depth {
                    freq_diff /= 2;
                }

                let mut tmpvfrq = freq0;
                if self.savelnthcc[track_idx] & 0x1f > 7 {
                    // vibrato if note length >= 8
                    for _ in 0..oscilatval {
                        tmpvfrq += freq_diff;
                    }
                }
                // println!("vibrato_v10{}:{}",track_idx,tmpvfrq);
                // vd=0..15
                // d=(F1-F0)/2^vd
                // tmpvfrq = F0 + osc*d
                tmpvfrq
            }
            _ => {
                let temp_freq_diff = self.vibrato_spellbound(instr, track_idx);
                // println!("vibrato_def{}:{}",track_idx, temp_freq_diff);
                temp_freq_diff
            }
        };
        self.sid.set_freq(track_idx, freq);
    }

    fn pulse_width_timbre(&mut self, track_idx: usize) {
        let instr_idx = self.instrnr[track_idx] as usize;
        let instr = &self.songs.instruments[instr_idx];
        let pulsevalue = instr.pulse_speed;

        let mask_lo: u8 = if let 15 = self.songs.version {
            0b0000_1111
        } else {
            0b000_11111
        };
        let mask_hi: u8 = if let 15 = self.songs.version {
            0b1111_0000
        } else {
            0b111_00000
        };

        if instr.fx & 8 == 0 {
            if pulsevalue == 0 {
                return;
            }

            if self.pulsedelay[track_idx] != 0 {
                self.pulsedelay[track_idx] -= 1;
            } else {
                self.pulsedelay[track_idx] = pulsevalue & mask_lo;
                let pulsespeed = (pulsevalue & mask_hi) as u16;

                // println!("{} delay:{} speed:{}, dir:{}",track_idx, self.pulsedelay[track_idx], pulsespeed, self.pulsedir[track_idx]);

                let new_pulse = if self.pulsedir[track_idx] == 0 {
                    let new_pulse = self.instr_pw[instr_idx] + pulsespeed;
                    if new_pulse & 0x0f00 == 0x0e00 {
                        self.pulsedir[track_idx] += 1;
                    }
                    new_pulse
                } else {
                    // pulse down
                    let new_pulse = self.instr_pw[instr_idx].wrapping_sub(pulsespeed);
                    if new_pulse & 0x0f00 == 0x0800 {
                        // reaches min
                        self.pulsedir[track_idx] -= 1;
                    }
                    new_pulse
                };
                self.instr_pw[instr_idx] = new_pulse; // HACK
                                                      // println!("{} pw:{}", track_idx, new_pulse);
                self.sid.set_pw(track_idx, new_pulse);
            }
        } else {
            // See Commando, Crazy Commets code
            let new_pulse = instr.pulse_width + pulsevalue as u16;
            self.instr_pw[instr_idx] = new_pulse;
            // println!("{} instr.fx==1: commando code, pulsevalue:{}, pw:{}", track_idx, pulsevalue, new_pulse);
            self.sid.set_pw(track_idx, new_pulse);
        }
    }

    // portemento routine
    // TODO: v2 code: see noterh.rs
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
            self.savefreq[track_idx] = self.savefreq[track_idx].wrapping_sub(tempstore);
        }
    }

    // drums
    fn drums(&mut self, track_idx: usize) {
        let instr = &self.songs.instruments[self.instrnr[track_idx] as usize];
        //************ bit0:FX
        if instr.fx & 1 != 0
            && self.savefreq[track_idx] >> 8 != 0
            && self.lengthleft[track_idx] != 0
        {
            let mut vctrl = self.voicectrl[track_idx] & 0xfe;
            // check if this is the first vbl for this instrument-note
            if self.lengthleft[track_idx] >= self.savelnthcc[track_idx] & 0x1f {
                // first time: set noise
                vctrl = 0x80;
            } else {
                // next vbls..
                self.savefreq[track_idx] -= 0x0100; // near 1 octave
                if vctrl == 0 {
                    // set noise
                    vctrl = 0x80;
                }
            }
            // println!("drums{}:{} / {}",track_idx,self.savefreq[track_idx],vctrl);
            self.sid.set_freq(track_idx, self.savefreq[track_idx]);
            self.sid.set_ctrl(track_idx, vctrl);
        }
    }

    fn skydive(&mut self, track_idx: usize) {
        let instr = &self.songs.instruments[self.instrnr[track_idx] as usize];
        let ok_counter: bool = if let 15 = self.songs.version {
            self.counter & 3 == 0
        } else {
            self.counter & 1 != 0
        };
        //************ bit1:Skydive - a long portamento-down from the note to zerofreq
        if instr.fx & 2 != 0 && ok_counter && self.savefreq[track_idx] >> 8 != 0 {
            let freq = match self.songs.version {
                10 => {
                    if self.savelnthcc[track_idx] & 0x1f > self.songs.skydive_v1_when {
                        if (self.savefreq[track_idx] as isize + self.songs.skydive_v1_add as isize)
                            < 0x10000
                        {
                            if (self.savefreq[track_idx] as isize
                                + self.songs.skydive_v1_add as isize)
                                > 0
                            {
                                // println!("savefreq={}, skydive_v1={}", self.savefreq[track_idx], self.songs.skydive_v1_add);
                                self.savefreq[track_idx] = (self.savefreq[track_idx] as isize
                                    + self.songs.skydive_v1_add as isize)
                                    as u16;
                            }
                        }
                    }
                    self.savefreq[track_idx]
                }
                15 => {
                    self.notenum[track_idx] += 1;
                    let note = self.notenum[track_idx];
                    NOTE_FREQ_HEX[note as usize]
                }
                _ => {
                    // TODO
                    self.savefreq[track_idx]
                }
            };
            // println!("skydive{}:{}",track_idx, freq);
            self.sid.set_freq(track_idx, freq);
        }
    }

    fn octave_arpeggio(&mut self, track_idx: usize) {
        let instr = &self.songs.instruments[self.instrnr[track_idx] as usize];

        //************ bit2:instrfx
        // check if arpt needed
        if instr.fx & 4 != 0 {
            let mut note = if let 15 = self.songs.version {
                let mask = if (instr.fx >> 4) == 12 { 2 } else { 1 }; // instr.fx : 0bAAAACCCC : C for option, A value for &8 effect
                if self.counter & mask == 0 {
                    self.notenum[track_idx] - (instr.fx >> 4)
                } else {
                    self.notenum[track_idx]
                }
            } else {
                if self.counter & 1 == 0 {
                    // even, note
                    self.notenum[track_idx]
                } else {
                    // odd, change octave
                    self.notenum[track_idx] + 12
                }
            };
            // dump the corresponding frequencies
            note = self.assert_high_note(track_idx, note);
            // println!("octarp{}:{}", track_idx, NOTE_FREQ_HEX[note as usize]);
            self.sid.set_freq(track_idx, NOTE_FREQ_HEX[note as usize]);
        }
    }

    fn soundfx(&mut self) {
        if !self.engine_up || self.engine_fx_play & Fx::Stop == Fx::Stop as u8 {
            return;
        }
        if self.engine_fx_play & Fx::Config == Fx::Config as u8 {
            self.soundfx_copy();
            println!(
                "note:{}, note_dest:{}, note_delta:{}, ",
                self.sfx_note, self.sfx_note_dest, self.sfx_note_delta
            );
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
                    let note: usize = if self.sfx_note > self.sfx_note_delta {
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
        self.counter_note -= 1;
        if self.counter_note < 0 {
            self.counter_note = 4; // TODO: use array
        } else {
            // slowdown songs from resetspd VBL
            if self.speed == 0 {
                self.speed = self.resetspd;
            } else {
                self.speed -= 1;
            }
        }

        for track_idx in (0..3).rev() {
            // let instr = &self.songs.instruments[self.instrnr[track_idx] as usize];
            // println!("instr.fx{}: {:08b}", track_idx, instr.fx);

            if self.counter_note != 0 && self.speed == self.resetspd {
                if self.lengthleft[track_idx] == 0 {
                    self.get_new_note(track_idx);
                    // println!("note{}:{}",track_idx,self.notenum[track_idx]);
                    continue;
                } else {
                    self.lengthleft[track_idx] -= 1;
                    if self.engine_song_playing {
                        self.release(track_idx);
                    }
                }
            }
            if self.engine_song_playing {
                if self.notenum[track_idx] != 0 {
                    self.vibrato(track_idx);
                    self.pulse_width_timbre(track_idx);
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
        self.counter = (self.counter + 1) & 7; // only three bits used...

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
            if self.engine_up && self.engine_fx_play & Fx::Stop != Fx::Stop as u8 {
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
        self.sfx_note = self.assert_high_note(0, soundfx.voice0.freq as u8);
        self.sfx_note_dest = self.assert_high_note(0, soundfx.sfx_note_dest);

        self.sfx_voices_ctrl = soundfx.voice1.freq as u8;
        self.sfx_note_delta = self.sfx_voices_ctrl & 0b0011_1111;

        self.sfx_voice0_ctrl = soundfx.voice0.ctrl;
        self.sfx_voice1_ctrl = soundfx.voice1.ctrl;

        self.sid.set_all(
            0,
            soundfx.voice0.freq,
            soundfx.voice0.pulse_width,
            soundfx.voice0.ctrl,
            soundfx.voice0.attack_and_decay_len,
            soundfx.voice0.sustain_vol_and_release_len,
        );
        self.sid.set_all(
            1,
            soundfx.voice1.freq,
            soundfx.voice1.pulse_width,
            soundfx.voice1.ctrl,
            soundfx.voice1.attack_and_decay_len,
            soundfx.voice1.sustain_vol_and_release_len,
        );
    }

    #[allow(dead_code)]
    pub fn engine_up(&mut self) {
        self.engine_up = true;
    }

    #[allow(dead_code)]
    pub fn engine_down(&mut self) {
        self.engine_up = false;
        self.sid.set_ctrl(0, 0);
        self.sid.set_ctrl(1, 0);
        self.engine_fx_play = 0xFF; // Fx::Stop | Fx::Config | 0b0011_1111;
    }

    pub fn set_soundfx(&mut self, soundfx: usize) {
        if !self.engine_up {
            self.engine_fx_play = 0xFF; // Fx::Stop | Fx::Config | 0b0011_1111;
        } else {
            self.sid.set_modvol(15);
            self.engine_fx_play = soundfx as u8 | Fx::Config;
        }
    }

    pub fn init(&mut self, song: usize) {
        self.sid.set_modvol(15);
        if song < self.songs.channels.len() {
            // song
            self.current_tracks = self.songs.channels[song];
            self.sid.set_resfilt(0);
            self.sid.set_ctrl(0, 0);
            self.sid.set_ctrl(1, 0);
            self.sid.set_ctrl(2, 0);
            self.sid.set_modvol(15);
            self.mstatus = SongState::Config as u8;
        } else {
            // soundfx
            self.stop_music();
            self.set_soundfx(song - self.songs.channels.len());
        }
    }

    pub fn stop_music(&mut self) {
        self.mstatus = SongState::ConfigStop as u8;
    }

    pub fn sample(&mut self, delta: u32, buffer: &mut [i16], interleave: usize) -> (usize, u32) {
        self.sid.sample(delta, buffer, interleave)
    }

    #[allow(dead_code)]
    pub fn get_sid_regs(&self) -> Option<[(u16, u16, u8, u8, u8); 3]> {
        let v0 = self.sid.get_all(0).unwrap();
        let v1 = self.sid.get_all(1).unwrap();
        let v2 = self.sid.get_all(2).unwrap();
        Some([v0, v1, v2])
    }
}
