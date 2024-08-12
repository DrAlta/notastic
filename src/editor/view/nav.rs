use iced::widget::{text, Column, MouseArea};

use crate::{Editor, generate_lookup, Message};


impl Editor {
    pub fn nav_veiw(&self) -> Column<Message> {
        let lookup = generate_lookup(&self.notes);
        let children = lookup.into_iter().map(
            |(title, uuid)| {
                MouseArea::new(text(title))
                    .on_release(Message::CautiouLoadNoteInEditor(uuid.clone()))
                    .into()
            }
        );
    
        Column::with_children(children)

    }
}