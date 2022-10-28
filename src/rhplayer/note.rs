use std::fmt;

pub struct Note {
    pub value: u8,
    pub length: u16,
    pub instr: u8,
    pub portamento: u8,
    pub release: bool,
}


impl Note {
    pub fn new() -> Note {
        Note {
            value: 0,
            length: 0,
            instr: 0,
            portamento: 0,
            release: false,
        }
    }

    pub fn set_value(&mut self, value: u8) -> bool {
        if value > 127 {
            return false;
        } else {
            self.value = value;
            return true;
        }
    }
}

impl fmt::Debug for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Note [{},{},{},{},{}]", self.value, self.length, self.instr, self.portamento, self.release)
    }
}