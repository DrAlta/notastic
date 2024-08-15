use std::{collections::HashMap, sync::Arc};

use iced::{
    executor, widget::{row, text_editor}, Application, Command, Theme
};
use qol::logy;
use uuid::Uuid;

use crate::{save_to_wiki, DragState, EditorState, Message, Notastic, Note};

impl Application for Notastic {
    type Message = Message;

    type Executor = executor::Default;
    
    type Theme = Theme;
    
    type Flags = ();


    fn new(_flags:()) -> (Self, Command<Message>) {
        let notes = match crate::load_notes_from_json("./test_notes.json") {
            Ok(ok) => ok,
            Err(err) => {
                logy!("error", "failed to load notes.json with {err}");
                HashMap::new()
            }
        };

        (Self {
            drag_state: DragState::NotDragging,
            nav_size: 200.0,
            notes,
            note_editor: EditorState::Closed,
            filter_title_open: "".to_owned(),
        },
        Command::none())
    }

    fn title(&self) -> String {
        "Notastic!".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::CautiouLoadNoteInEditor(uuid) => {
                logy!("cautiou_load_note", "told to open note: {uuid}");
                if !self.cautions_load_note_in_editor(uuid) {
                    logy!("error", "Failed to get note {uuid}");
                }
            }
            Message::CreateOpen => {
                let mut old_notes_uuids = self.notes.iter().filter_map(|(old_uuid, note)| {
                    if note.title.trim() == self.filter_title_open.trim() {
                        Some(old_uuid.clone())
                    } else {
                        None
                    }
                });
                let loaded_old_note_ka = if let Some(old_notes_uuid) = old_notes_uuids.next() {
                    self.cautions_load_note_in_editor(old_notes_uuid)
                } else {
                    false
                };
                if loaded_old_note_ka {
                    logy!("trace", "loaded old note instead of creating new");
                } else {
                    let uuid = Uuid::new_v4();
                    self.note_editor = EditorState::Uuid {
                        uuid,
                        title: self.filter_title_open.clone(),
                        body: text_editor::Content::with_text(""),
                    }
                }
            }
            Message::DragEnd => {
                logy!("trace", "dragging end");
                self.drag_state = DragState::NotDragging;
            },
            Message::DragStart => {
                if let DragState::NotDragging = self.drag_state {
                    logy!("trace", "starting drag");
                    self.drag_state = DragState::StartDraging;
                }
            },
            Message::Dragging(point) => {
                match self.drag_state {
                    DragState::NotDragging => (),
                    DragState::StartDraging => {
                        self.drag_state = DragState::Dragging(point.x);
                    },
                    DragState::Dragging(origin) => {
                        let delta = point.x - origin;
                        self.nav_size += delta;
                    },
                }
            },
            Message::Edit(action) => {
                println!("got edit message");
                match &mut self.note_editor {
                    EditorState::Closed => {
                        logy!("trace", "got an Edit message but no editor is open");
                        return Command::none();
                    },
                    EditorState::Uuid{uuid:_, title:_, body} => {
                        body.perform(action);
                    },
                    EditorState::Wiki { title:_, body, baserevid:_, csrf:_, original_text:_ } => {
                        body.perform(action);
                    },
                };
            }
            Message::ExportButtonPressed => {
                return self.update(Message::ExportJson("./notes.json".to_owned()));
            }
            Message::ExportJson(path) => {
                logy!("trace", "exporting to {path}");
                let Err(err) = crate::save_notes_to_json(path, &self.notes) else {
                    return Command::none();
                };
                logy!("error", "failed to export notes JSON with:{err}");
            }
            Message::FilterCreateChanged(title) => {
                self.filter_title_open = title;
            }
            Message::ImportButtonPressed => {
                return Command::perform(Notastic::pick_file(), Message::LoadNotes)
            }
            Message::LoadNotes(Ok(mut notes)) => {
                logy!("trace", "trying to import notes");
                let Some(x) = Arc::get_mut(&mut notes) else {
                    return Command::none();
                };
                let ugly_hack = HashMap::new();
                let notes = std::mem::replace(x, ugly_hack);
                self.notes = notes;
                logy!("trace", "successfully imported notes");
                
            }
            Message::LoadNotes(Err(err)) => {
                logy!("error", "loaded to selec file ot load with {err}");
            }
            Message::SaveNote => {
                //let Some((uuid, title, body)) = &mut self.note_editor else {
                match &mut self.note_editor {
                    EditorState::Closed => {
                        logy!("trace", "Got SaveNote but no note is open");
                        return Command::none();
                    },
                    EditorState::Uuid { uuid, title, body } => {
                        if let Some(old_note) = self.notes.get_mut(uuid) {
                            let new_body = body.text();
                            let new = new_body.trim();
                            let old = old_note.body.trim();
                            if old == new {
                                logy!("trace", "no changes just closing the editor");
                                self.note_editor = EditorState::Closed;
                                return Command::none();
                            }
                            old_note.body_history.push(old.to_owned());
                            old_note.body = new.to_owned();
                            std::mem::swap(&mut old_note.title, title);
                            self.note_editor = EditorState::Closed;
                            return Command::none();
                        } else {
                            logy!("trace", "saving note '{title}':{uuid}");
                            let ugly_hack = EditorState::Closed;
                            let old_editor = std::mem::replace(&mut self.note_editor, ugly_hack);
                            let EditorState::Uuid{uuid, title, body} = old_editor else {
                                logy!("error", "the note editor has disappered on us!");
                                return Command::none();
                            };
                            self.notes
                                .insert(uuid, Note::new(title, body.text(), Vec::new()));
                            self.note_editor = EditorState::Closed;
                        }
                    },
                    EditorState::Wiki { title, body, baserevid, csrf: Some(token), original_text } => {
                        logy!("trace", "saving to wiki '{title}'");
                        let new_body = body.text();
                        let new = new_body.trim();
                        let old = original_text.trim();
                        if old == new {
                            logy!("trace", "no changes just closing the editor");
                            self.note_editor = EditorState::Closed;
                            return Command::none();
                        }
                        return Command::perform(save_to_wiki( title.clone(), new.to_owned(), baserevid.clone(), token.clone()), |_| Message::SaveToWikiResult)
                    },
                    EditorState::Wiki { title, body, baserevid:_, csrf: None, original_text } => {
                        let uuid = Uuid::new_v4();
                        logy!("trace", "saving wiki page locally '{title}':{uuid}");
                        let new_body = body.text();
                        let new = new_body.trim();
                        let old = original_text.trim();
                        if old == new {
                            logy!("trace", "no changes just closing the editor");
                            self.note_editor = EditorState::Closed;
                            return Command::none();
                        }

                        let ugly_hack = EditorState::Closed;
                        let old_editor = std::mem::replace(&mut self.note_editor, ugly_hack);
                        let EditorState::Wiki { title, body, baserevid:_, csrf:_, original_text:_ } = old_editor else {
                            logy!("error", "the note editor has disappered on us!");
                            return Command::none();
                        };
                        self.notes
                            .insert(uuid, Note::new(title, body.text(), Vec::new()));
                        self.note_editor = EditorState::Closed;

                    },
                }
            }
            Message::SaveToWikiResult => {
                logy!("error", "Message::SaveToWikiResult not handled yet");
            },
            Message::TitleChanged(new_title) => {
                let EditorState::Uuid{uuid:_, title, body:_} = &mut self.note_editor else {
                    logy!("trace", "got an TitleChanged message but no editor is open");
                    return Command::none();
                };
                *title = new_title;
            }
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let nav = self.nav_veiw();

        let right_side = if let EditorState::Closed = self.note_editor {
            self.note_veiwer_veiw()
        } else {
            self.note_editor_veiw()
        };
        row!(nav, right_side).into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
    
}
