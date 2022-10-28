use super::note::Note;
use super::noterh::NoteRh;


pub trait PatternRh {
    fn uncompress(pattern: &[u8]) -> Option<Vec<Note>>;
}

impl PatternRh for Vec<Note> {
    fn uncompress(pattern: &[u8]) -> Option<Vec<Note>> {
        let mut output = Self::new();
        let mut idx = 0;
        loop {
            let note_idx = Note::uncompress(&pattern[idx..]);
            match note_idx {
                Some( (note, i) ) => { output.push(note); idx=idx+i; },
                None => break,
            }
        }
        if output.len() == 0 {
            None
        } else {
            Some(output)
        }
    }
}