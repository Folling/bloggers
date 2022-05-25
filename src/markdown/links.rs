// #[allow(clippy::wildcard_imports)]
// use nom::{bytes::complete::*, character::complete::*, combinator::*, sequence::*, IResult};
//
// pub fn link(input: &str) -> IResult<&str, String> {
//     map(
//         pair(
//             delimited(char('['), is_not("]"), char(']')),
//             delimited(char('('), is_not(")"), char(')')),
//         ),
//         |(display, location)| format!("<a href='/media/{}'>{}</a>", location, display),
//     )(input)
// }
//
// pub fn media(input: &str) -> IResult<&str, String> {
//     map(
//         tuple((
//             char('!'),
//             delimited(char('['), is_not("]"), char(']')),
//             delimited(char('('), is_not(")"), char(')')),
//         )),
//         |(_, display, location)| format!("<img src='{}' alt='/media/{}'></img>", location, display),
//     )(input)
// }
