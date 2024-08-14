use std::{collections::HashMap, sync::Arc};

use iced::widget::text_editor;
use qol::logy;
use uuid::Uuid;

use crate::{DragState, Note};


pub struct Notastic {
    pub nav_size: f32,
    pub drag_state: DragState,
    pub notes: HashMap<Uuid, Note>,
    pub note_editor: Option<(Uuid, String, text_editor::Content)>,
    pub filter_title_open: String,
}

impl Notastic {
    pub fn cautions_load_note_in_editor(&mut self, uuid: Uuid) -> bool {
        if let Some((old_uuid, _, note_body)) = &self.note_editor {
            if let Some(old_note) = self.notes.get(&old_uuid) {
                let new_body = note_body.text();
                let new = new_body.trim();
                let old = old_note.body.trim();
                if old != new {
                    logy!(
                        "cautiou_load_note",
                        "{:?}\n!=\n{:?}",
                        note_body.text(),
                        old_note.body
                    );
                    return false;
                }
            }
        };
        match self.notes.get(&uuid) {
            Some(note) => {
                self.note_editor = Some((
                    uuid,
                    note.title.clone(),
                    text_editor::Content::with_text(&note.body),
                ));
                true
            }
            None => false,
        }
    }
    pub async fn pick_file() -> Result<Arc<HashMap<Uuid, Note>>, String> {
        let handle = rfd::AsyncFileDialog::new().set_title("Choose notes JSON file").pick_file().await.ok_or("open file dialog error".to_owned())?;
        match crate::load_notes_from_json(handle.path()) {
            Ok(ok) => {
                Ok(Arc::new(ok))
            }
            Err(err) => {
                Err(format!("failed to import notes JSON with:{err}"))
            }
        }
    }
}
