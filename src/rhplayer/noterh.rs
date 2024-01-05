use super::note::Note;

pub trait NoteRh {
    /* usize for next pattern index */
    fn uncompress(pattern: &[u8], version: usize) -> Option<(Note, usize)>;
}

impl NoteRh for Note {
    fn uncompress(pattern: &[u8], version: usize) -> Option<(Note, usize)> {
        let mut i = 0;

        // EOP (End Of Pattern)
        if pattern[i] == 0b1111_1111 {
            return None;
        }

        let mut first_pass = true;
        let mut note = Note::new();

        loop {
            let length = pattern[i] & 0b000_11111;
            let release: bool = pattern[i] & 0b0010_0000 != 0b0010_0000;
            let appended: bool = pattern[i] & 0b0100_0000 == 0b0100_0000;
            let next_instr_or_port: bool = pattern[i] & 0b1000_0000 == 0b1000_0000;
            i += 1;
            if !appended {
                if !first_pass {
                    return Some((note, i - 1));
                }
                first_pass = false;

                note.portamento = 0;
                note.instr = 0;
                if next_instr_or_port {
                    match version {
                        10 => {
                            let is_portamento = pattern[i] & 0b1000_0000 == 0b1000_0000;
                            if is_portamento {
                                note.portamento = (pattern[i] as i16) & 0b0111_1110;
                                if pattern[i] & 1 == 1 {
                                    note.portamento = -note.portamento;
                                }
                            } else {
                                note.instr = pattern[i] & 0b0111_1111;
                            }
                        }
                        15 => {
                            println!("spellbound instr: 0b{:08b}", pattern[i]);
                            note.instr = pattern[i];
                        }
                        20 | 30 => {
                            let is_portamento = pattern[i] & 0b1000_0000 == 0b1000_0000;
                            if is_portamento {
                                let sign = (pattern[i] & 0b0_1_000000) == 0b0_1_000000;
                                note.portamento = ((pattern[i] & 0b00_111111) as i16) << 8;
                                i += 1;
                                note.portamento |= pattern[i] as i16;
                                if sign {
                                    note.portamento = -note.portamento;
                                }
                            } else {
                                note.instr = pattern[i] & 0b0111_1111;
                            }
                        }
                        _ => {
                            println!("uncompress(): unknown version!");
                        }
                    }
                    i += 1;
                }
                let value = if pattern[i] == 104 {
                    println!("WARN/HACK: Avoid an original freq array overflow in monty on the run (base error? 0d104=0x68 near 0x65)");
                    65
                } else {
                    pattern[i]
                };
                note.set_value(value & 0b0111_1111);
                note.length = length as u16;
                note.release = release;
                i += 1;
            } else {
                note.length += length as u16;
            }

            // EOP (End Of Pattern)
            if pattern[i] == 0b1111_1111 {
                return Some((note, i));
            }
        }
    }
}
