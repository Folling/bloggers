// use crate::markdown::textblock::text_item;
// #[allow(clippy::wildcard_imports)]
// use nom::{branch::*, character::complete::*, combinator::*, multi::*, sequence::*, IResult};
//
// pub fn list_item(input: &str) -> IResult<&str, String> {
//     map(delimited(space0, text_item, newline), |item| format!("<li>{}</li>", item))(input)
// }
//
// pub fn ordered_list(input: &str) -> IResult<&str, String> {
//     map(many1(preceded(pair(char('-'), char('#')), list_item)), |str| {
//         format!("<ol>{}</ol>", str.join(""))
//     })(input)
// }
//
// pub fn unordered_list(input: &str) -> IResult<&str, String> {
//     map(many1(pair(many1_count(char('-')), list_item)), |(count, str)| {
//         format!("<ul>{}</ul>", str.join(""))
//     })(input)
// }
//
// pub fn list(input: &str) -> IResult<&str, String> {
//     alt((ordered_list, unordered_list))(input)
// }
