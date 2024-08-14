use iced::{
    widget::{column, container, Column}, Border, Color, Length
};

use crate::{Message, Notastic, SPACING};

pub enum NoteDiv {
    Div(String),
    TemplateLoading{title:String},
    Template{title:String, body:String, show_body: bool}
}

impl Notastic {
    pub fn note_veiwer_veiw(&self) -> container::Container<Message> {
        let Some(divs) = Some(vec![
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
            },
            
        ]) else {
            return container::Container::new(iced::widget::Space::new(Length::Fill, Length::Fill));
        };

        let children = divs.iter().map(|div|{
            match div {
                NoteDiv::Div(text) => {
                    iced::widget::text(text).into()
                },
                NoteDiv::Template { title: _, body, show_body: true } => {

                    let style= iced::widget::container::Appearance{ 
                        text_color: Some(Color::from_rgb(0.0, 0.75, 0.75)), 
                        background: Some(iced::Background::Color(Color::from_rgb(0.23, 0.20, 0.25))), 
                        border: Border::default(), 
                        shadow: iced::Shadow::default() };
                        let text = iced::widget::text(body);
                    container::Container::new(column!(text)).style(style).into()
                }, 
                NoteDiv::Template { title, body:_, show_body: false } => {
                    let style= iced::widget::container::Appearance{ 
                        text_color: Some(Color::from_rgb(0.6, 0.6, 0.4)), 
                        background: Some(iced::Background::Color(Color::from_rgb(0.23, 0.20, 0.25))), 
                        border: Border::default(), 
                        shadow: iced::Shadow::default() };
                    container::Container::new(iced::widget::text(title)).style(style).into()
                }, 
                NoteDiv::TemplateLoading { title } => {
                    let style= iced::widget::container::Appearance{ 
                        text_color: Some(Color::from_rgb(0.8, 0.4, 0.4)), 
                        background: Some(iced::Background::Color(Color::from_rgb(0.23, 0.14, 0.25))), 
                        border: Border::default(), 
                        shadow: iced::Shadow::default() };
                    container::Container::new(iced::widget::text(title)).style(style).into()
                },
            }
        });

        container(Column::with_children(children)).padding(SPACING)
    }
}
