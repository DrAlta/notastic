use iced::{widget::{button, column, mouse_area, row, scrollable, text, text_input, vertical_rule, Column, MouseArea}, Length};

use crate::{generate_lookup, Message, Notastic};

impl Notastic {
    pub fn nav_veiw(&self) -> Column<Message> {
        let filter_create_input = text_input("filter/create", &self.filter_title_open)
            .width(Length::Fill)
            .on_input(Message::FilterCreateChanged);

        let save_button = button(text("Open")).on_press(Message::CreateOpen);

        let filter_create_bar = row!(filter_create_input, save_button);

        let lookup = generate_lookup(&self.notes);
        let children = lookup.into_iter().map(|(title, uuid)| {
            MouseArea::new(text(title))
                .on_release(Message::CautiouLoadNoteInEditor(uuid.clone()))
                .into()
        });

        let note_list = scrollable(Column::with_children(children))
            .width(Length::Fill)
            .height(Length::Fill);

        let rule = mouse_area(vertical_rule(5))
            .on_press(Message::DragStart)
            .on_move(Message::Dragging)
            .on_exit(Message::DragEnd)
            .on_release(Message::DragEnd);
        let middle= row!(note_list, rule).width(Length::Fill);

        let import_button = button(text("Import")).on_press(Message::ImportButtonPressed);
        let export_button = button(text("Export")).on_press(Message::ExportButtonPressed);
        let save_load_bar = row!(import_button, export_button);

        column!(filter_create_bar, middle, save_load_bar).width(self.nav_size)
    }
}
