use std::collections::HashMap;

use iced::{widget::{row, text_editor}, Sandbox};
use qol::logy;

use crate::{Editor, Message};

impl Sandbox for Editor {
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
         }
    }

    fn title(&self) -> String {
        "Notastic!".into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::CautiouLoadNoteInEditor(uuid) => {
                logy!("cautiou_load_note", "told to open note: {uuid}");
                if let Some((old_uuid, note_body)) = &self.note_editor {
                    if let Some(old_note) = self.notes.get(&old_uuid) {
                        let new_body = note_body.text();
                        let new = new_body.trim();
                        let old = old_note.body.trim();
                        if old != new {
                            logy!("cautiou_load_note", "{:?}\n!=\n{:?}", note_body.text(), old_note.body);
                            return;
                        }
                    }
                };
                match self.notes.get(&uuid) {
                    Some(note) => {
                        self.note_editor = Some((uuid, text_editor::Content::with_text(&note.body)));
                    },
                    None => {logy!("error", "Failed to get note {uuid}")},
                }
 
            },
            Message::Edit(action) => {
                println!("got edit message");
                let Some((_, note_body))= &mut self.note_editor else {
                    logy!("trace", "got an Edit message but no editor is open");
                    return
                };
                note_body.perform(action);
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