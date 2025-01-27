mod json;

mod message;
pub use message::Message;


mod notastic;
pub use notastic::Notastic;

mod note;
pub use note::Note;

mod ui;
pub use ui::{DragState, EditorState, NoteDiv, to_vec_note_div};

const SPACING: u16 = 10;



fn main() {
    println!("{:?}", iced::application("Notastic", Notastic::update, Notastic::view).run_with(|| Notastic::new(())));
}
