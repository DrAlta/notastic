use std::{collections::HashMap, sync::Arc};

use iced::widget::text_editor;
use qol::logy;
use uuid::Uuid;

use crate::{json, to_vec_note_div, DragState, EditorState, Message, Note, NoteDiv};

pub struct Notastic {
    pub nav_size: f32,
    pub drag_state: DragState,
    pub notes: HashMap<Uuid, Note>,
    pub note_editor: EditorState,
    pub veiw_state: Option<Vec<NoteDiv>>,
    pub filter_title_open: String,
}

impl Notastic {
    pub fn new(_flags:()) -> (Self, iced::Task<Message>) {
        let notes = match json::load_notes_from_json("./test_notes.json") {
            Ok(ok) => ok,
            Err(err) => {
                logy!("error", "failed to load notes.json with {err}");
                HashMap::new()
            }
        };

        (
            Self {
                drag_state: DragState::NotDragging,
                nav_size: 200.0,
                notes,
                note_editor: EditorState::Closed,
                veiw_state: None,
                filter_title_open: "".to_owned(),
            },
            iced::Task::none()
        )
    }
    /// this check the note_editor state
    /// if it's EditorState::Uuid or EditorState::Wiki then it checks if 
    /// the note's body is the same as the body being edited if it's diffrent 
    /// then it returns false
    /// 
    /// if it passed that check thne it gets the note if it exists and changes 
    /// note_editor to EditorState::Uuid with the note loaded and returns true
    /// if it doesn'r exist it returns false.
    pub fn cautious_load_note_in_editor(&mut self, uuid: Uuid) -> bool {
        match &self.note_editor {
            EditorState::Uuid { uuid: old_uuid, title:_, body: note_body } => {
                if let Some(old_note) = self.notes.get(&old_uuid) {
                    let new_body = note_body.text();
                    let new = new_body.trim();
                    let old = old_note.body.trim();
                    if old != new {
                        logy!(
                            "cautious_load_note",
                            "{:?}\n!=\n{:?}",
                            note_body.text(),
                            old_note.body
                        );
                        return false;
                    }
                }
            }
            EditorState::Closed => todo!(),
            EditorState::Wiki { title:_, body, baserevid:_, csrf:_, original_text } => {
                let new_body = body.text();
                let new = new_body.trim();
                let old = original_text.trim();
                if old != new {
                    logy!(
                        "cautious_load_note",
                        "{:?}\n!=\n{:?}",
                        note_body.text(),
                        old_note.body
                    );
                    return false;
                }
            },
        };
        match self.notes.get(&uuid) {
            Some(note) => {
                self.note_editor = EditorState::Uuid{
                    uuid,
                    title: note.title.clone(),
                    body: text_editor::Content::with_text(&note.body),
                };
                true
            }
            None => false,
        }
    }
    /// this check the note_editor state
    /// if it's EditorState::Uuid or EditorState::Wiki then it checks if 
    /// the note's body is the same as the body being edited if it's diffrent 
    /// then it returns false
    /// 
    /// if it passed that check thne it gets the note if it exists and changes 
    /// note_editor to EditorState::Uuid with the note loaded and returns true
    /// if it doesn'r exist it returns false.
    pub fn cautious_load_note_in_veiwer(&mut self, uuid: Uuid) -> bool {
        match &self.note_editor {
            EditorState::Uuid { uuid: old_uuid, title:_, body: note_body } => {
                if let Some(old_note) = self.notes.get(&old_uuid) {
                    let new_body = note_body.text();
                    let new = new_body.trim();
                    let old = old_note.body.trim();
                    if old != new {
                        logy!(
                            "cautious_load_note_in_veiwer",
                            "{:?}\n!=\n{:?}",
                            note_body.text(),
                            old_note.body
                        );
                        return false;
                    }
                }
            }
            EditorState::Closed => (),
            EditorState::Wiki { title:_, body, baserevid:_, csrf:_, original_text } => {
                let new_body = body.text();
                let new = new_body.trim();
                let old = original_text.trim();
                if old != new {
                    logy!(
                        "cautious_load_note_in_veiwer",
                        "{:?}\n!=\n{:?}",
                        note_body.text(),
                        old_note.body
                    );
                    return false;
                }
            },
        };
        match self.notes.get(&uuid) {
            Some(note) => {
                self.veiw_state = Some(to_vec_note_div(&note.body));
                true
            }
            None => false,
        }
    }
    pub async fn pick_file() -> Result<Arc<HashMap<Uuid, Note>>, String> {
        let handle = rfd::AsyncFileDialog::new().set_title("Choose notes JSON file").pick_file().await.ok_or("open file dialog error".to_owned())?;
        match json::load_notes_from_json(handle.path()) {
            Ok(ok) => {
                Ok(Arc::new(ok))
            }
            Err(err) => {
                Err(format!("failed to import notes JSON with:{err}"))
            }
        }
    }
}
