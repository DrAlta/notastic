use std::{collections::HashMap, fs, path::Path};

use uuid::Uuid;

use crate::{json_note::JsonNote, note::Note};

pub fn load_notes_from_json<P: AsRef<Path> + std::fmt::Debug>(
    path: P,
) -> Result<HashMap<Uuid, Note>, String> {
    let file_data: String = match fs::read_to_string(&path) {
        Ok(ok) => ok,
        Err(err) => return Err(format!("Failed to open {path:?} with {err:?}")),
    };
    let json_notes: Result<Vec<JsonNote>, serde_jsonrc::Error> = serde_jsonrc::from_str(&file_data);

    let mut notes = HashMap::<Uuid, Note>::new();
    match json_notes {
        Ok(x) => {
            for json_note in x {
                let uuid = match Uuid::parse_str(&json_note.uuid) {
                    Ok(x) => x,
                    Err(err) => {
                        return Err(format!(
                            "failed to connvert {} into a UUID with {err:?}",
                            json_note.uuid
                        ))
                    }
                };
                notes.insert(uuid, json_note.into());
            }
        }
        Err(err) => return Err(format!("failed to load notes form json with: {err:?}")),
    }
    Ok(notes)
}
