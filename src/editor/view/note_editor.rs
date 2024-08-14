use iced::{
    widget::{button, column, container, horizontal_space, row, text, text_editor, text_input},
    Length,
};

use crate::{Notastic, Message, SPACING};

impl Notastic {
    pub fn note_editor_veiw(&self) -> container::Container<Message> {
        let Some((_uuid, title, note_body)) = &self.note_editor else {
            return container::Container::new(iced::widget::Space::new(Length::Fill, Length::Fill));
        };

        let editor_title = text_input("enter note title here", title)
            .on_input(Message::TitleChanged);

        let save_button: button::Button<Message> = button(text("Save")).on_press(Message::SaveNote);

        let title_bar = row!(editor_title, save_button);

        let editor_body = text_editor(note_body)
            .height(Length::Fill)
            .on_action(Message::Edit);

        let position = {
            let (line, column) = note_body.cursor_position();

            text(format!("{}:{}", line + 1, column + 1))
        };


        let status_bar = row!(horizontal_space(), position).spacing(SPACING);

        container(column!(title_bar, editor_body, status_bar)).padding(SPACING)
    }
}
