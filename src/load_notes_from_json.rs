use std::{collections::HashMap, path::Path};

use uuid::Uuid;

use crate::note::Note;

pub fn load_notes_from_json<P: AsRef<Path>>(path: P) -> Result<HashMap<Uuid, Note>, String> {
    let file_data: String = match fs_err::read_to_string(path) {
        Ok(ok) => ok,
        Err(err) => return Err(format!("Failed to open notes json with {err:?}")),
    };
    let notes_result: Result<HashMap<Uuid, Note>, serde_jsonrc::Error> =
        serde_jsonrc::from_str(&file_data);

    match notes_result {
        Ok(ok) => Ok(ok),
        Err(err) => return Err(format!("failed to load notes form json with: {err:?}")),
    }
}
