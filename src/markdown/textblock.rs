// #[allow(clippy::wildcard_imports)]
// use nom::{branch::*, bytes::complete::*, character::complete::*, combinator::*, multi::*, sequence::*, IResult};
//
// use crate::markdown::auxiliary::horizontal_line;
// use crate::markdown::lists::list;
// #[allow(clippy::wildcard_imports)]
// use crate::markdown::{codeblocks::*, links::*, textstyles::*};
//
// pub fn text_item(input: &str) -> IResult<&str, String> {
//     alt((
//         inline_code,
//         link,
//         media,
//         bold_italic,
//         italic,
//         bold,
//         strikethrough,
//         underlined,
//         map(is_not("\n"), |str: &str| str.to_string()),
//     ))(input)
// }
//
// pub fn textblock(input: &str) -> IResult<&str, String> {
//     let res = many_till(
//         map(pair(text_item, opt(newline)), |(content, newline)| {
//             format!("{}{}", content, if newline.is_some() { "<br>" } else { "" })
//         }),
//         peek(alt((map(eof, |_| String::new()), code_block, list, horizontal_line))),
//     )(input)?;
//
//     #[rustfmt::skip]
//     IResult::Ok((res.0, format!("<p>{}</p>", res.1.0.join(""))))
// }
