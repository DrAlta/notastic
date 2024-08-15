use qol::logy;

extern crate nom;

pub enum NoteDiv {
    Div(String),
    TemplateLoading{title:String},
    Template{title:String, body:String, show_body: bool}
}

fn template_parser(i: &str) -> nom::IResult<&str, NoteDiv> {
    let (tail, (_, body, _)) = nom::sequence::tuple((
        nom::bytes::complete::tag("{{"),
        nom::bytes::complete::take_until("}}"),
        nom::bytes::complete::tag("}}"),
    ))(i)?;
    Ok((tail, NoteDiv::TemplateLoading { title: body.into()}))   
}

fn text_parser(i: &str) -> nom::IResult<&str, NoteDiv> {
    let (tail, body) = nom::combinator::recognize(
        nom::multi::many0(
            nom::character::complete::none_of("[{")
        )
    )(i)?;
    Ok((tail, NoteDiv::Div(body.into())))   
}



pub fn to_vec_note_div(value:&str) -> Vec<NoteDiv> {
    let value = value.trim();
    match nom::multi::many0(
        nom::branch::alt((
            text_parser,
            template_parser,
        ))
    )(value) {
        Ok((tail , mut ok)) => {
            if tail != "" {
                ok.push(NoteDiv::Div(tail.into()))
            };
            ok
        },
        Err(err) => {
            logy!("error", "failed to parse note{err:?}");
            vec![NoteDiv::Div(value.into())]
        },
    }

    
}