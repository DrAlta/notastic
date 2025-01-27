use std::{collections::HashMap, sync::Arc};

use iced::{widget::text_editor, Point};
use uuid::Uuid;

use crate::Note;

#[derive(Debug, Clone)]
pub enum Message {
    CautiousLoadNoteInEditor(Uuid),
    CreateOpen,
    DragEnd,
    Dragging(Point),
    DragStart,

    Edit(text_editor::Action),
    ExportButtonPressed,
    ExportJson(String),
    FilterCreateChanged(String),
    ImportButtonPressed,
    LoadNotes(Result<Arc<HashMap<Uuid, Note>>, String>),
    SaveNote,
    SaveToWikiResult,
    TitleChanged(String),
}
