use iced::{widget::text_editor, Sandbox, Settings};
use uuid::Uuid;

mod editor;
use editor::Editor;

mod generate_lookup;
use generate_lookup::generate_lookup;


mod json_note;
pub use json_note::JsonNote;

mod load_notes_from_json;
pub use load_notes_from_json::load_notes_from_json;

mod note;
pub use note::Note;

mod reverse_str;
pub use reverse_str::ReverseStr;

const SPACING: u16 = 10;

fn main() {
    println!(
        "{:?}", 
        <Editor as Sandbox>::run(Settings::default())
    );
}


#[derive(Debug, Clone)]
pub enum Message {
    CautiouLoadNoteInEditor(Uuid),
    Edit(text_editor::Action)

}


