use iced::widget::{button, column, row, text, text_input, Column, MouseArea};

use crate::{generate_lookup, Notastic, Message};

impl Notastic {
    pub fn nav_veiw(&self) -> Column<Message> {
        let filter_create_input =  text_input("filter/create", &self.filter_title_open)
            .on_input(Message::FilterCreateChanged);

        let save_button = button(text("Open")).on_press(Message::CreateOpen);
        
        let filter_create_bar = row!(filter_create_input, save_button);

        let lookup = generate_lookup(&self.notes);
        let children = lookup.into_iter().map(|(title, uuid)| {
            MouseArea::new(text(title))
                .on_release(Message::CautiouLoadNoteInEditor(uuid.clone()))
                .into()
        });

        let note_list = Column::with_children(children);

        let inport_button = button(text("Inport")).on_press(Message::InportButtonPressed);
        let export_button = button(text("Export")).on_press(Message::InportButtonPressed);
        let save_load_bar = row!(inport_button, export_button);

        column!(filter_create_bar, note_list, save_load_bar)
    }
}
