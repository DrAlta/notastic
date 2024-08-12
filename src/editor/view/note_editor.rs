use iced::{widget::{column, container, horizontal_space, row, text, text_editor}, Length};

use crate::{Editor, Message, SPACING};


impl Editor {
    pub fn note_editor_veiw(&self) -> container::Container<Message> {
        let Some((_, note_body))= &self.note_editor else {
            return container::Container::new(iced::widget::Space::new(Length::Fill, Length::Fill))
        };
        let editor_body = text_editor(note_body)
            .height(Length::Fill)
            .on_action(Message::Edit);

        let position = {
            let (line, column) = note_body.cursor_position();

            text(format!("{}:{}", line + 1, column + 1))
        };


        let status_bar =  row!(horizontal_space(), position).spacing(SPACING);

        container(column!(editor_body, status_bar)).padding(SPACING)
    }

}

