#[allow(clippy::wildcard_imports)]
use nom::{branch::*, character::complete::*, combinator::*, multi::*, IResult};

#[allow(clippy::wildcard_imports)]
use crate::markdown::{auxiliary::*, codeblocks::*, links::*, textstyles::*};

pub fn textblock(input: &str) -> IResult<&str, String> {
    let res = many_till(
        alt((
            inline_code,
            link,
            media,
            bold_italic,
            italic,
            bold,
            strikethrough,
            underlined,
            map(newline, |_| String::from("<br>")),
            map(
                many_till(
                    alt((map(newline, |_| String::from("<br>")), map(anychar, String::from))),
                    peek(alt((
                        map(eof, |_| String::new()),
                        header,
                        inline_code,
                        link,
                        media,
                        bold_italic,
                        italic,
                        bold,
                        strikethrough,
                        underlined,
                        code_block,
                        horizontal_line,
                    ))),
                ),
                |(data, _)| data.into_iter().collect(),
            ),
        )),
        alt((map(eof, |_| String::new()), peek(alt((header, code_block, horizontal_line))))),
    )(input)?;

    #[rustfmt::skip]
    IResult::Ok((res.0, format!("<p>{}</p>", res.1.0.join(""))))
}
