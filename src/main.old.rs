use std::collections::HashMap;

use qol::{logy, pout};
mod generate_lookup;
use generate_lookup::generate_lookup;
mod json_note;
mod load_notes_from_json;
use load_notes_from_json::load_notes_from_json;
mod note;
mod reverse_str;
pub use reverse_str::ReverseStr;
mod save_notes_to_json;
pub use save_notes_to_json::save_notes_to_json;
use uuid::Uuid;

mod iced;
fn main() {}
//#[allow(dead_code)]
pub fn foo() {
    let notes: HashMap<Uuid, note::Note> = match load_notes_from_json("./test_notes.json") {
        Ok(ok) => ok,
        Err(err) => {
            logy!("error", "failed to load notes.json with {err}");
            return;
        }
    };

    let lookup = generate_lookup(&notes);
    for (title, uuid) in lookup {
        pout!("{title}={uuid}");
    }
}
