use iced::{Application, Settings};

mod editor;
use editor::Notastic;

mod editor_state;
pub use editor_state::EditorState;

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
mod note_dev;
pub use note_dev::NoteDiv;

mod reverse_str;
pub use reverse_str::ReverseStr;

mod save_notes_to_json;
pub use save_notes_to_json::save_notes_to_json;

mod save_to_wiki;
pub use save_to_wiki::save_to_wiki;

const SPACING: u16 = 10;

pub enum DragState {
    NotDragging,
    StartDraging,
    Dragging(f32),
}

fn main() {
    println!("{:?}", <Notastic as Application>::run(Settings::default()));
}
