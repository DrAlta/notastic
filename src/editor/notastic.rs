use std::collections::HashMap;

use iced::widget::text_editor;
use uuid::Uuid;

use crate::Note;

pub struct Notastic {
    pub notes: HashMap<Uuid, Note>,
    pub note_editor: Option<(Uuid, String, text_editor::Content)>,
}
