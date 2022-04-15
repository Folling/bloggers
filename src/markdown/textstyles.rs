#[allow(clippy::wildcard_imports)]
use nom::{branch::*, bytes::complete::*, character::complete::*, combinator::*, multi::*, sequence::*, IResult};

pub fn header(input: &str) -> IResult<&str, String> {
    map(
        tuple((many1_count(char('#')), space0, alphanumeric1, newline)),
        |(level, _, content, _)| format!("<h{0}>{1}</h{0}>", level, content),
    )(input)
}

pub fn italic(input: &str) -> IResult<&str, String> {
    map(
        delimited(
            char('*'),
            alt((strikethrough, underlined, map(is_not("*"), std::string::ToString::to_string))),
            char('*'),
        ),
        |content| format!("<i>{}</i>", content),
    )(input)
}

pub fn bold(input: &str) -> IResult<&str, String> {
    map(
        delimited(
            tag("**"),
            alt((strikethrough, underlined, map(is_not("*"), std::string::ToString::to_string))),
            tag("**"),
        ),
        |content| format!("<b>{}</b>", content),
    )(input)
}

pub fn bold_italic(input: &str) -> IResult<&str, String> {
    map(
        delimited(
            tag("***"),
            alt((strikethrough, underlined, map(is_not("*"), std::string::ToString::to_string))),
            tag("***"),
        ),
        |content| format!("<i><b>{}</b></i>", content),
    )(input)
}

pub fn underlined(input: &str) -> IResult<&str, String> {
    map(
        delimited(
            char('_'),
            alt((
                bold_italic,
                italic,
                bold,
                strikethrough,
                map(is_not("_"), std::string::ToString::to_string),
            )),
            char('_'),
        ),
        |content| format!("<u>{}</u>", content),
    )(input)
}

pub fn strikethrough(input: &str) -> IResult<&str, String> {
    map(
        delimited(
            char('~'),
            alt((
                bold_italic,
                italic,
                bold,
                underlined,
                map(is_not("~"), std::string::ToString::to_string),
            )),
            char('~'),
        ),
        |content| format!("<s>{}</s>", content),
    )(input)
}
