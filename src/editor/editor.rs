use std::collections::HashMap;

use iced::widget::text_editor;
use uuid::Uuid;

use crate::Note;

pub struct Editor {
    pub notes: HashMap<Uuid, Note>,
    pub note_editor: Option<(Uuid, text_editor::Content)>,
}
