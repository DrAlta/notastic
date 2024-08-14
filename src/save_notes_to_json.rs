use std::{collections::HashMap, io::Write, path::PathBuf};

use uuid::Uuid;

use crate::note::Note;

pub fn save_notes_to_json<P: Into<PathBuf>>(
    path: P,
    notes: &HashMap<Uuid, Note>,
) -> Result<(), String> {
    let json = match serde_jsonrc::to_string(notes) {
        Ok(ok) => ok,
        Err(err) => return Err(format!("Failed to convers notes to JSON with {err:?}")),
    };

    match fs_err::File::create(path)
        .map_err(|err| format!("failed to create json file with:{err:?}"))?
        .write_all(json.as_bytes())
    {
        Ok(ok) => Ok(ok),
        Err(err) => Err(format!("Failed to save notes json with {err:?}")),
    }
}
