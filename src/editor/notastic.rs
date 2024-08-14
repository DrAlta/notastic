use std::collections::HashMap;

use iced::widget::text_editor;
use uuid::Uuid;

use crate::{DragState, Note};


pub struct Notastic {
    pub nav_size: f32,
    pub drag_state: DragState,
    pub notes: HashMap<Uuid, Note>,
    pub note_editor: Option<(Uuid, String, text_editor::Content)>,
    pub filter_title_open: String,
}
