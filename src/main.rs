use std::collections::BTreeMap;

use qol::{logy, pout};

mod json_note;
mod load_notes_from_json;
use load_notes_from_json::load_notes_from_json;
mod note;
mod reverse_str;
pub use reverse_str::ReverseStr;
mod save_notes_to_json;
pub use save_notes_to_json::save_notes_to_json;
use uuid::Uuid;

fn main() {

    let notes = match load_notes_from_json("./test_notes.json") {
    
        Ok(ok) => ok,
        Err(err) => {
            logy!("error", "failed to load notes.json with {err}");
            return;
        },
    };

    let lookup: BTreeMap<ReverseStr, &Uuid> = notes.iter().map(|(uuid, note)|{
        (Into::<ReverseStr>::into(&note.title), uuid)
    }).collect();
    for (title, uuid) in lookup {
        pout!("{title}={uuid}");
    }
}
