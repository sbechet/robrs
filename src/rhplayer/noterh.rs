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
            i+=1;
            if ! appended {
       
                if ! first_pass {
                    return Some( (note, i-1) );
                }
                first_pass = false;


                note.portamento = 0;
                note.instr = 0;
                if next_instr_or_port {
                    if let 15 = version {
                        println!("spellbound instr: 0b{:08b}", pattern[i]);
                        note.instr = pattern[i];
                    } else {
                        let is_portamento = pattern[i]&0b1000_0000 == 0b1000_0000;
                        let v = pattern[i]&0b0111_1111;
                        if is_portamento {
                            note.portamento = v as i16;
                            if version == 20 {
                                i+=1;
                                let sign = note.portamento&0b0100_0000 << 9;
                                print!("sign:{}, p1:{}, p2:{}", sign, note.portamento, pattern[i]);
                                note.portamento = note.portamento&0b0011_1111 << 8;
                                note.portamento += pattern[i] as i16;
                                note.portamento |= sign;
                                println!(", res:{}", note.portamento);
                            }
                        } else {
                            note.instr = v;
                        }
                    }
                    i+=1;
                }
                note.set_value(pattern[i]);
                note.length = length as u16;
                note.release = release;
                i+=1;
            } else {
                note.length += length as u16;
            }

            // EOP (End Of Pattern)
            if pattern[i] == 0b1111_1111 {
                return Some( (note, i) );
            }
        }

    }
}