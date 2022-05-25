// #[allow(clippy::wildcard_imports)]
// use nom::{bytes::complete::*, character::complete::*, combinator::*, sequence::*, IResult};
//
// pub fn code_block(input: &str) -> IResult<&str, String> {
//     map(
//         tuple((tag("```"), opt(alpha1), newline, take_until("```"), tag("```"), opt(newline))),
//         |(_, language, _, content, _, _)| {
//             format!(
//                 "<pre><code class='language-{}'>{}</code></pre>",
//                 language.unwrap_or("plaintext"),
//                 content
//             )
//         },
//     )(input)
// }
//
// pub fn inline_code(input: &str) -> IResult<&str, String> {
//     map(pair(delimited(char('`'), is_not("`"), char('`')), opt(newline)), |(content, _)| {
//         format!("<code class='plaintext'>{}</code>", content)
//     })(input)
// }
