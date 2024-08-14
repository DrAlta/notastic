use std::collections::HashMap;

use iced::{
    widget::{row, text_editor},
    Sandbox,
};
use qol::logy;
use uuid::Uuid;

use crate::{Message, Notastic, Note};

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
                self.note_editor =
                    Some((uuid, note.title.clone(), text_editor::Content::with_text(&note.body)));
                true
            }
            None => {
                false
            }
        }
    }
}

impl Sandbox for Notastic {
    type Message = Message;

    fn new() -> Self {
        let notes = match crate::load_notes_from_json("./test_notes.json") {
            Ok(ok) => ok,
            Err(err) => {
                logy!("error", "failed to load notes.json with {err}");
                HashMap::new()
            }
        };

        Self {
            notes,
            note_editor: None,
            filter_title_open: "".to_owned(),
        }
    }

    fn title(&self) -> String {
        "Notastic!".into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::CautiouLoadNoteInEditor(uuid) => {
                logy!("cautiou_load_note", "told to open note: {uuid}");
                if !self.cautions_load_note_in_editor(uuid) {
                    logy!("error", "Failed to get note {uuid}");
                }
            }
            Message::CreateOpen => {
                let mut old_notes_uuids = self.notes.iter().filter_map(
                    |(old_uuid, note)| {
                        if note.title.trim() == self.filter_title_open.trim() {
                            Some(old_uuid.clone())
                        } else {
                            None 
                        }
                    }
                );
                let loaded_old_note_ka = if let Some(old_notes_uuid) = old_notes_uuids.next() {
                    self.cautions_load_note_in_editor(old_notes_uuid)
                } else {
                    false
                };
                if loaded_old_note_ka {
                    logy!("trace", "loaded old note instead of creating new");
                } else {
                    let uuid = Uuid::new_v4();
                    self.note_editor = Some((uuid, self.filter_title_open.clone(), text_editor::Content::with_text("")))
                }
            },
            Message::Edit(action) => {
                println!("got edit message");
                let Some((_, _, note_body)) = &mut self.note_editor else {
                    logy!("trace", "got an Edit message but no editor is open");
                    return;
                };
                note_body.perform(action);
            },
            Message::ExportButtonPressed => {
                self.update(Message::ExportJson("./notes.json".to_owned()))
            },
            Message::ExportJson(path) => {
                let Err(err) = crate::save_notes_to_json(path, &self.notes) else {
                    return;
                };
                logy!("error", "failed to export notes JSON with:{err}");
            },
            Message::FilterCreateChanged(title) => {
                self.filter_title_open = title;       
            },
            Message::InportButtonPressed => {
                self.update(
                    Message::InportJson("./save_test_notes.json".to_owned())
                )
            },
            Message::InportJson(path) => {
                let Err(err) = crate::load_notes_from_json(path) else {
                    return;
                };
                logy!("error", "failed to inport notes JSON with:{err}");
            },
            Message::SaveNote => {
                let Some((uuid, title, editor_body)) = &mut self.note_editor else {
                    logy!("trace", "Got SaveNote but no note is open");
                    return;
                };
                if let Some(old_note) = self.notes.get_mut(uuid) {
                    let new_body = editor_body.text();
                    let new = new_body.trim();
                    let old = old_note.body.trim();
                    if old == new {
                        logy!(
                            "trace",
                            "no changes just closing the editor"
                        );
                        self.note_editor = None;
                        return;
                    }
                    old_note.body_history.push(old.to_owned());
                    old_note.body = new.to_owned();
                    std::mem::swap(&mut old_note.title, title);
                    self.note_editor = None;
                    return;
                } else {
                    logy!("trace", "saving note '{title}':{uuid}");
                    let ugly_hack = None; 
                    let old_editor = std::mem::replace(&mut self.note_editor, ugly_hack);
                    let Some((uuid, title, editor_body)) = old_editor else {
                        logy!("error", "the note editor has disappered on us!");
                        return;
                    };
                    self.notes.insert(
                        uuid, 
                        Note::new(
                            title, 
                            editor_body.text(), 
                            Vec::new()
                        )
                    );
                    self.note_editor = None;
                }
            },
            Message::TitleChanged(new_title) => {
                let Some((_, title, _)) = &mut self.note_editor else {
                    logy!("trace", "got an TitleChanged message but no editor is open");
                    return;
                };        
                *title = new_title;        
            },
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let nav = self.nav_veiw();
        let note_editor = self.note_editor_veiw();
        row!(nav, note_editor).into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}
