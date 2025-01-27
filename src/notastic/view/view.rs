
use iced::widget::row;

use crate::{EditorState, Message, Notastic};

impl Notastic {
    pub fn view(&self) -> iced::Element<'_, Message> {
        let nav = self.nav_veiw();

        let right_side = if let EditorState::Closed = self.note_editor {
            self.note_veiwer_veiw()
        } else {
            self.note_editor_veiw()
        };
        row!(nav, right_side).into()
    }
}