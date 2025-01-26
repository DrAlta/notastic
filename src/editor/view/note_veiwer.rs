use std::sync::LazyLock;

use iced::{
    widget::{container, span, text::Rich}, Color, Length
};

use crate::{Message, Notastic, NoteDiv, SPACING};

static FOO: LazyLock<Option<Vec<NoteDiv>>> =LazyLock::new(||Some(vec![
    NoteDiv::Div("First".into()),
    NoteDiv::TemplateLoading { title: "unloadedTemplate".into() },
    NoteDiv::Div("Second".into()),
    NoteDiv::Template { 
        title: "FilledTemplate".into(), 
        body: "This is some text from, templete".into(), 
        show_body: true 
    },
    NoteDiv::Div("Third".into()),
    NoteDiv::Template { 
        title: "FoldedTemplate".into(), 
        body: "This is some hidden text from, templete".into(), 
        show_body: false
    }
]));
impl Notastic {
    pub fn note_veiwer_veiw(&self) -> container::Container<Message> {
        let Some(divs) =  &*FOO else {
            return container::Container::new(iced::widget::Space::new(Length::Fill, Length::Fill));
        };

        let spans: Vec<_> = divs.iter().map(|div| -> iced::widget::text::Span<Message,iced::Font>{
            match div {
                NoteDiv::Div(text) => {
                    span(text)
                },
                NoteDiv::Template { title: _, body, show_body: true } => {
                    span(body)
                        .color(Color::from_rgb(0.0, 0.75, 0.75))
                        .background(iced::Background::Color(Color::from_rgb(0.23, 0.20, 0.25)))
                }, 
                NoteDiv::Template { title, body:_, show_body: false } => {
                    span(title)
                        .color(Color::from_rgb(0.6, 0.6, 0.4))
                        .background(iced::Background::Color(Color::from_rgb(0.23, 0.20, 0.25)))
                }, 
                NoteDiv::TemplateLoading { title } => {
                    span(title)
                        .color(Color::from_rgb(0.8, 0.4, 0.4))
                        .background(iced::Background::Color(Color::from_rgb(0.23, 0.14, 0.25)))
                },
            }
        }).collect();

        container(Rich::with_spans(spans)).padding(SPACING)
    }
}
