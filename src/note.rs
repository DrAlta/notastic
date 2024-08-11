use serde::Serialize;

//use uuid::Uuid;

use crate::json_note::JsonNote;


#[derive(Debug, PartialEq, Serialize)]
pub struct Note {
    //pub uuid: String,
    pub title: String,
    pub body: String,
    pub body_history: Vec<String>,
}
impl Note {
    pub fn new(
        //uuid: String,
        title: String,
        body: String, 
        body_history: Vec<String>
    ) -> Self {
        Self { /*uuid,*/ title, body, body_history}
    }
}

impl From<JsonNote> for Note {
    fn from(value: JsonNote) -> Self {
        Note::new(value.title, value.body, value.body_history)
    }
}