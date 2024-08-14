use iced::{Sandbox, Settings};

mod editor;
use editor::Notastic;

mod generate_lookup;
use generate_lookup::generate_lookup;

mod json_note;
pub use json_note::JsonNote;

mod load_notes_from_json;
pub use load_notes_from_json::load_notes_from_json;

mod message;
pub use message::Message;

mod note;
pub use note::Note;

mod reverse_str;
pub use reverse_str::ReverseStr;

mod save_notes_to_json;
pub use save_notes_to_json::save_notes_to_json;

const SPACING: u16 = 10;

fn main() {
    println!("{:?}", <Notastic as Sandbox>::run(Settings::default()));
}
