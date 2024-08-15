use iced::{
    widget::{button, column, container, horizontal_space, row, text, text_editor, text_input}, Length
};

use crate::{EditorState, Message, Notastic, SPACING};

impl Notastic {
    pub fn note_editor_veiw(&self) -> container::Container<Message> {
        match &self.note_editor {
            EditorState::Closed  => {
                return container::Container::new(iced::widget::Space::new(Length::Fill, Length::Fill));
            },  
            EditorState::Uuid { uuid, title, body } => {

                let editor_title =
                    text_input("enter note title here", title).on_input(Message::TitleChanged);

                let save_button: button::Button<Message> = button(text("Save")).on_press(Message::SaveNote);

                let title_bar = row!(editor_title, save_button);

                let editor_body = text_editor(body)
                    .height(Length::Fill)
                    .on_action(Message::Edit);

                let position = {
                    let (line, column) = body.cursor_position();

                    text(format!("{}:{}", line + 1, column + 1))
                };


                let status_bar = row!(text(format!("{uuid}")), horizontal_space(), position).spacing(SPACING);

                return container(column!(title_bar, editor_body, status_bar)).padding(SPACING);
            },
            EditorState::Wiki { title, body, baserevid:_, csrf, original_text:_ } => {
                let editor_title = text(title.clone());

                let save_button: button::Button<Message> = 
                    button(text(if csrf.is_some() {"Save to wiki"} else {"Save Local"})).on_press(Message::SaveNote);

                let title_bar = row!(editor_title, save_button);

                let editor_body = text_editor(body)
                    .height(Length::Fill)
                    .on_action(Message::Edit);

                let position = {
                    let (line, column) = body.cursor_position();

                    text(format!("{}:{}", line + 1, column + 1))
                };


                let status_bar = row!(text(format!("Wiki:{title}")), horizontal_space(), position).spacing(SPACING);

                return container(column!(title_bar, editor_body, status_bar)).padding(SPACING);
            }
        }
    }
}
