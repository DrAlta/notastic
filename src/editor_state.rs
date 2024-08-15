use iced::widget::text_editor;
use uuid::Uuid;

#[derive(Debug)]
pub enum EditorState {
    Closed,
    Uuid{uuid:Uuid, title:String, body:text_editor::Content},
    Wiki{title:String, body:text_editor::Content, baserevid: i64, csrf: Option<String>, original_text: String}
}