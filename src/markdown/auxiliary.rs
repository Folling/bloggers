#[allow(clippy::wildcard_imports)]
use nom::{bytes::complete::*, character::complete::*, combinator::*, sequence::*, IResult};

pub fn horizontal_line(input: &str) -> IResult<&str, String> {
    map(pair(tag("---"), opt(newline)), |_| String::from("<hr>"))(input)
}
