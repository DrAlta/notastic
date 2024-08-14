use iced::widget::text_editor;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum Message {
    CautiouLoadNoteInEditor(Uuid),
    CreateOpen,
    Edit(text_editor::Action),
    ExportButtonPressed,
    ExportJson(String),
    FilterCreateChanged(String),
    ImportButtonPressed,
    ImportJson(String),
    SaveNote,
    TitleChanged(String),
}
