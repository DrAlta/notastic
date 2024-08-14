use iced::widget::text_editor;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum Message {
    CautiouLoadNoteInEditor(Uuid),
    Edit(text_editor::Action),
    SaveNote,
    TitleChanged(String),
}
