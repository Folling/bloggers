#[allow(clippy::wildcard_imports)]
use nom::{branch::*, character::complete::*, combinator::*, multi::*, sequence::*};

#[allow(clippy::wildcard_imports)]
use crate::markdown::{auxiliary::*, codeblocks::*, links::*, textblock::*, textstyles::*};

use anyhow::{bail, Result};

`pub fn parse<S: AsRef<str>>(content: S) -> Result<String> {
    let res = many_till(
        pair(
            alt((
                header,
                horizontal_line,
                link,
                media,
                bold_italic,
                bold,
                italic,
                underlined,
                strikethrough,
                code_block,
                inline_code,
                textblock,
            )),
            opt(newline),
        ),
        eof,
    )(content.as_ref());

    match res {
        Err(nom::Err::Failure(e) | nom::Err::Error(e)) => bail!("failed with {}", e.code.description()),
        Err(nom::Err::Incomplete(_)) => bail!("incomplete"),
        nom::IResult::Ok((_, (v, _))) => Ok(v
            .into_iter()
            .map(|(v, lf)| format!("{}{}", v, if lf.is_some() { "<br>" } else { "" }))
            .collect::<String>()),
    }
}
