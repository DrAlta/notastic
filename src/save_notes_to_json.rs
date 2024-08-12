use std::{collections::HashMap, fs, path::Path};

use uuid::Uuid;

use crate::note::Note;

pub fn save_notes_to_json<P: AsRef<Path> + std::fmt::Debug>(
    path: P,
    notes: &HashMap<Uuid, Note>,
) -> Result<(), String> {
    let json = match serde_jsonrc::to_string(notes) {
        Ok(ok) => ok,
        Err(err) => return Err(format!("Failed to convers notes to JSON with {err:?}")),
    };

    match fs::write(&path, json) {
        Ok(ok) => Ok(ok),
        Err(err) => Err(format!("Failed to savr {path:?} with {err:?}")),
    }
}
