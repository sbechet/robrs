use resid::Sid;

pub mod reg {
    pub const FREQLO1: u8 = 0x00;
    pub const FREQHI1: u8 = 0x01;
    pub const PWLO1: u8 = 0x02;
    pub const PWHI1: u8 = 0x03;
    pub const CR1: u8 = 0x04;
    pub const AD1: u8 = 0x05;
    pub const SR1: u8 = 0x06;
    pub const FREQLO2: u8 = 0x07;
    pub const FREQHI2: u8 = 0x08;
    pub const PWLO2: u8 = 0x09;
    pub const PWHI2: u8 = 0x0a;
    pub const CR2: u8 = 0x0b;
    pub const AD2: u8 = 0x0c;
    pub const SR2: u8 = 0x0d;
    pub const FREQLO3: u8 = 0x0e;
    pub const FREQHI3: u8 = 0x0f;
    pub const PWLO3: u8 = 0x10;
    pub const PWHI3: u8 = 0x11;
    pub const CR3: u8 = 0x12;
    pub const AD3: u8 = 0x13;
    pub const SR3: u8 = 0x14;
    pub const FCLO: u8 = 0x15;
    pub const FCHI: u8 = 0x16;
    pub const RESFILT: u8 = 0x17;
    pub const MODVOL: u8 = 0x18;
    pub const POTX: u8 = 0x19;
    pub const POTY: u8 = 0x1a;
    pub const OSC3: u8 = 0x1b;
    pub const ENV3: u8 = 0x1c;
}

//   C,     C#,      D,     D#,      E,      F,     F#,      G,     G#,      A,     A#,      B
#[allow(dead_code)]
pub static NOTE_FREQ_HEX: [u16; 8*12] = [
 0x116,  0x127,  0x138,  0x14B,  0x15F,  0x173,  0x18A,  0x1A1,  0x1BA,  0x1D4,  0x1F0,  0x20E, // 0
 0x22D,  0x24E,  0x271,  0x296,  0x2BD,  0x2E7,  0x313,  0x342,  0x374,  0x3A9,  0x3E0,  0x41B, // 1
 0x45A,  0x49B,  0x4E2,  0x52C,  0x57B,  0x5CE,  0x627,  0x685,  0x6E8,  0x751,  0x7C1,  0x837, // 2
 0x8B4,  0x937,  0x9C4,  0xA57,  0xAF5,  0xB9C,  0xC4E,  0xD09,  0xDD0,  0xEA3,  0xF82, 0x106E, // 3
0x1168, 0x126E, 0x1388, 0x14AF, 0x15EB, 0x1739, 0x189C, 0x1A13, 0x1BA1, 0x1D46, 0x1F04, 0x20DC, // 4
0x22D0, 0x24DC, 0x2710, 0x295E, 0x2BD6, 0x2E72, 0x3138, 0x3426, 0x3742, 0x3A8C, 0x3E08, 0x41B8, // 5
0x45A0, 0x49B8, 0x4E20, 0x52BC, 0x57AC, 0x5CE4, 0x6270, 0x684C, 0x6E84, 0x7518, 0x7C10, 0x8370, // 6
0x8B40, 0x9370, 0x9C40, 0xA578, 0xAF58, 0xB9C8, 0xC4E0, 0xD098, 0xDD08, 0xEA30, 0xF820, 0xFD2E]; // 7

pub trait SidExt {
    fn print_note(&self, note: u8);

    fn set_freq(&mut self, idx: usize, freq:u16);
    fn set_pw(&mut self, idx: usize, pw:u16);
    fn set_ctrl(&mut self, idx: usize, cr:u8);
    fn set_ad(&mut self, idx: usize, ad:u8);
    fn set_sr(&mut self, idx: usize, ad:u8);

    fn get_freq(&self, idx: usize) -> Option<u16>;
    fn get_pw(&self, idx: usize) -> Option<u16>;
    fn get_cr(&self, idx: usize) -> Option<u8>;
    fn get_ad(&self, idx: usize) -> Option<u8>;
    fn get_sr(&self, idx: usize) -> Option<u8>;

    fn set_all(&mut self, idx: usize, freq:u16, pw:u16, ctrl:u8, ad:u8, sr:u8);
    fn get_all(&self, idx: usize) -> Option<(u16, u16, u8, u8, u8)>;
    fn set_resfilt(&mut self, resfilt:u8);
    fn set_modvol(&mut self, vol:u8);
}

#[allow(dead_code)]
pub static PR_NOTE: [&str; 12] = ["C-","C#","D-","D#","E-","F-","F#","G-","G#","A-","A#","B-"];

impl SidExt for Sid {

    fn print_note(&self, note: u8) {
        let octave = note / 12;
        let rnote = note % 12;
        print!("{}{}", PR_NOTE[rnote as usize], octave);
    }

    fn set_freq(&mut self, idx: usize, freq:u16) {
        match idx {
            0 => {
                self.write(reg::FREQLO1, freq as u8);
                self.write(reg::FREQHI1, (freq>>8) as u8);
            },
            1 => {
                self.write(reg::FREQLO2, freq as u8);
                self.write(reg::FREQHI2, (freq>>8) as u8);
            },
            2 => {
                self.write(reg::FREQLO3, freq as u8);
                self.write(reg::FREQHI3, (freq>>8) as u8);
            },
            _ => {
                println!("SidExt:No {} wave found", idx);
            },
        }
    }

    fn set_pw(&mut self, idx: usize, pw:u16) {
        match idx {
            0 => {
                self.write(reg::PWLO1, pw as u8);
                self.write(reg::PWHI1, (pw>>8) as u8);
            },
            1 => {
                self.write(reg::PWLO2, pw as u8);
                self.write(reg::PWHI2, (pw>>8) as u8);
            },
            2 => {
                self.write(reg::PWLO3, pw as u8);
                self.write(reg::PWHI3, (pw>>8) as u8);
            },
            _ => {
                println!("SidExt:No {} wave found", idx);
            },
        }
    }

    fn set_ctrl(&mut self, idx: usize, cr:u8) {
        match idx {
            0 => {
                self.write(reg::CR1, cr);
            },
            1 => {
                self.write(reg::CR2, cr);
            },
            2 => {
                self.write(reg::CR3, cr);
            },
            _ => {
                println!("SidExt:No {} wave found", idx);
            },
        }
    }

    fn set_ad(&mut self, idx: usize, ad:u8) {
        match idx {
            0 => {
                self.write(reg::AD1, ad);
            },
            1 => {
                self.write(reg::AD2, ad);
            },
            2 => {
                self.write(reg::AD3, ad);
            },
            _ => {
                println!("SidExt:No {} wave found", idx);
            },
        }
    }
    
    fn set_sr(&mut self, idx: usize, sr:u8) {
        match idx {
            0 => {
                self.write(reg::SR1, sr);
            },
            1 => {
                self.write(reg::SR2, sr);
            },
            2 => {
                self.write(reg::SR3, sr);
            },
            _ => {
                println!("SidExt:No {} wave found", idx);
            },
        }
    }

    fn get_freq(&self, idx: usize) -> Option<u16> { 
        let fq = match idx {
            0 => Some((self.read(reg::FREQHI1) as u16)<<8 | self.read(reg::FREQLO1) as u16),
            1 => Some((self.read(reg::FREQHI2) as u16)<<8 | self.read(reg::FREQLO2) as u16),
            2 => Some((self.read(reg::FREQHI3) as u16)<<8 | self.read(reg::FREQLO3) as u16),
            _ => None,
        };
        fq
    }

    fn get_pw(&self, idx: usize) -> Option<u16> {
        let pw = match idx {
            0 => Some((self.read(reg::PWHI1) as u16)<<8 | self.read(reg::PWLO1) as u16),
            1 => Some((self.read(reg::PWHI2) as u16)<<8 | self.read(reg::PWLO2) as u16),
            2 => Some((self.read(reg::PWHI3) as u16)<<8 | self.read(reg::PWLO3) as u16),
            _ => None,
        };
        pw
    }

    fn get_cr(&self, idx: usize) -> Option<u8> {
        let cr = match idx {
            0 => Some(self.read(reg::CR1)),
            1 => Some(self.read(reg::CR2)),
            2 => Some(self.read(reg::CR3)),
            _ => None,
        };
        cr
    }

    fn get_ad(&self, idx: usize) -> Option<u8> {
        let ad = match idx {
            0 => Some(self.read(reg::AD1)),
            1 => Some(self.read(reg::AD2)),
            2 => Some(self.read(reg::AD3)),
            _ => None,
        };
        ad
    }

    fn get_sr(&self, idx: usize) -> Option<u8> {
        let sr = match idx {
            0 => Some(self.read(reg::SR1)),
            1 => Some(self.read(reg::SR2)),
            2 => Some(self.read(reg::SR3)),
            _ => None,
        };
        sr
    }

    fn set_all(&mut self, idx: usize, freq:u16, pw:u16, ctrl:u8, ad:u8, sr:u8) {
        self.set_freq(idx, freq);
        self.set_pw(idx, pw);
        self.set_ctrl(idx, ctrl);
        self.set_ad(idx, ad);
        self.set_sr(idx, sr);
    }

    fn get_all(&self, idx: usize) -> Option<(u16, u16, u8, u8, u8)> {
        let fq = self.get_freq(idx);
        let pw = self.get_pw(idx);
        let cr = self.get_cr(idx);
        let ad = self.get_ad(idx);
        let sr = self.get_sr(idx);

        if fq.is_none() {
            None
        } else {
            Some((fq.unwrap(), pw.unwrap(), cr.unwrap(), ad.unwrap(), sr.unwrap()))
        }
    }

    fn set_resfilt(&mut self, resfilt:u8) {
        self.write(reg::RESFILT, resfilt);
    }

    fn set_modvol(&mut self, vol:u8) {
        self.write(reg::MODVOL, vol);
    }

}

