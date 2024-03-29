use super::note::Note;
use super::noterh::NoteRh;

pub trait TrackRh {
    fn uncompress(pattern: &[u8], version: usize) -> Option<Vec<Note>>;
}

impl TrackRh for Vec<Note> {
    fn uncompress(pattern: &[u8], version: usize) -> Option<Vec<Note>> {
        let mut output = Self::new();
        let mut idx = 0;
        loop {
            let note_idx = Note::uncompress(&pattern[idx..], version);
            match note_idx {
                Some((note, i)) => {
                    output.push(note);
                    idx = idx + i;
                }
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
