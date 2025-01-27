use std::{collections::HashMap, sync::Arc};

use iced::widget::text_editor;
use qol::logy;
use uuid::Uuid;

use crate::{json, notastic::update::save_to_wiki, DragState, EditorState, Message, Notastic, Note};

impl Notastic{
    pub fn update(&mut self, message: Message) -> iced::Task<Message>{
        match message {
            Message::CautiousLoadNoteInEditor(uuid) => {
                logy!("cautious_load_note", "told to open note: {uuid}");
                if !self.cautious_load_note_in_editor(uuid) {
                    logy!("error", "Failed to get note {uuid}");
                }
            }
            Message::CreateOpen => {
                // find all the notes with the title
                let mut old_notes_uuids = self.notes.iter().filter_map(|(old_uuid, note)| {
                    if note.title.trim() == self.filter_title_open.trim() {
                        Some(old_uuid.clone())
                    } else {
                        None
                    }
                });
                // this tries to load the first note with a matching title into 
                // the veiwer; `loaded_old_note_ka` will be true if it was load,
                // false if it wasn't/  the reasons it failed was the note 
                // didn't exist or there was an unsaved note in the editor
                let loaded_old_note_ka = if let Some(old_notes_uuid) = old_notes_uuids.next() {
                    self.cautious_load_note_in_veiwer(old_notes_uuid)
                } else {
                    false
                };
                if loaded_old_note_ka {
                    logy!("trace", "loaded old note instead of creating new");
                } else {
                    // there wasn't any old notes with the title so creating a 
                    // new one
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
                        return iced::Task::none();
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
                let Err(err) = json::save_notes_to_json(path, &self.notes) else {
                    return iced::Task::none();
                };
                logy!("error", "failed to export notes JSON with:{err}");
            }
            Message::FilterCreateChanged(title) => {
                self.filter_title_open = title;
            }
            Message::ImportButtonPressed => {
                return iced::Task::perform(Notastic::pick_file(), Message::LoadNotes)
            }
            Message::LoadNotes(Ok(mut notes)) => {
                logy!("trace", "trying to import notes");
                let Some(x) = Arc::get_mut(&mut notes) else {
                    return iced::Task::none();
                };
                let notes = std::mem::replace(x, HashMap::new());
                self.notes = notes;
                logy!("trace", "successfully imported notes");
                
            }
            Message::LoadNotes(Err(err)) => {
                logy!("error", "failed to load notes from file with {err}");
            }
            Message::SaveNote => {
                //let Some((uuid, title, body)) = &mut self.note_editor else {
                match &mut self.note_editor {
                    EditorState::Closed => {
                        logy!("trace", "Got SaveNote but no note is open");
                        return iced::Task::none();
                    },
                    EditorState::Uuid { uuid, title, body } => {
                        if let Some(old_note) = self.notes.get_mut(uuid) {
                            let new_body = body.text();
                            let new = new_body.trim();
                            let old = old_note.body.trim();
                            if old == new {
                                logy!("trace", "no changes in editor; just closing the editor");
                                self.note_editor = EditorState::Closed;
                                return iced::Task::none();
                            }
                            old_note.body_history.push(old.to_owned());
                            old_note.body = new.to_owned();
                            std::mem::swap(&mut old_note.title, title);
                            self.note_editor = EditorState::Closed;
                            return iced::Task::none();
                        } else {
                            logy!("trace", "saving note '{title}':{uuid}");
                            let old_editor = std::mem::replace(&mut self.note_editor, EditorState::Closed);
                            let EditorState::Uuid{uuid, title, body} = old_editor else {
                                logy!("error", "the note editor has disappered on us!");
                                return iced::Task::none();
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
                            return iced::Task::none();
                        }
                        return iced::Task::perform(save_to_wiki( title.clone(), new.to_owned(), baserevid.clone(), token.clone()), |_| Message::SaveToWikiResult)
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
                            return iced::Task::none();
                        }

                        let old_editor = std::mem::replace(&mut self.note_editor, EditorState::Closed);
                        let EditorState::Wiki { title, body, baserevid:_, csrf:_, original_text:_ } = old_editor else {
                            logy!("error", "the note editor has disappered on us!");
                            return iced::Task::none();
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
                    return iced::Task::none();
                };
                *title = new_title;
            }
        }
        iced::Task::none()
    }
}