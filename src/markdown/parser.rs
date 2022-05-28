// #[allow(clippy::wildcard_imports)]
// use nom::{branch::*, character::complete::*, combinator::*, multi::*};
//
// #[allow(clippy::wildcard_imports)]
// use crate::markdown::{auxiliary::*, textblock::*, textstyles::*};
//
// use crate::markdown::codeblocks::code_block;
// use crate::markdown::lists::list;
// use anyhow::{bail, Result};
//
// pub fn parse<S: AsRef<str>>(content: S) -> Result<String> {
//     let res = many_till(
//         alt((
//             header,
//             horizontal_line,
//             list,
//             code_block,
//             textblock,
//             map(new|_| "<br>".to_string()),
//         )),
//         eof,
//     )(content.as_ref());
//
//     match res {
//         Err(nom::Err::Error(e)) => bail!("errored with {}", e.code.description()),
//         Err(nom::Err::Failure(e)) => bail!("failed with {}", e.code.description()),
//         Err(nom::Err::Incomplete(_)) => bail!("incomplete"),
//         nom::IResult::Ok((_, (v, _))) => Ok(v.into_iter().collect::<String>()),
//     }
// }

use anyhow::{anyhow, bail, Ok, Result};

use crate::markdown::items::{
    CodeBlock, Header, HorizontalLine, List, ListItem, ListType, Newline, Paragraph, StyledText, Text, TextItem, TextStyleFlags,
    TopLevelItem,
};
use crate::markdown::lexer::Token;
use crate::Lexer;
use log::debug;
use std::fmt::Write;

pub struct Parser {
    lexer: Lexer,
    linenr: usize,
    items: Vec<TopLevelItem>,
}

impl Parser {
    pub fn new(content: String) -> Self {
        Self {
            lexer: Lexer::new(content),
            linenr: 0,
            items: vec![],
        }
    }

    pub fn generate(&mut self) -> String {
        let mut ret = String::with_capacity(0x4000);

        for item in &self.items {
            write!(ret, "{}", item.generate()).unwrap();
        }

        ret
    }

    pub fn parse(&mut self) -> Result<String> {
        debug!("parsing");

        loop {
            match self.lexer.peek() {
                Token::Newline => {
                    let newline = self.parse_newline()?;
                    self.items.push(TopLevelItem::Newline { newline });
                }
                Token::Item(c) => match c {
                    '#' => {
                        let header = self.parse_header()?;
                        self.items.push(TopLevelItem::Header { header });
                    }
                    '-' => {
                        if self.lexer.peek_n::<4>() == [Token::Item('-'), Token::Item('-'), Token::Item('-'), Token::Newline] {
                            let line = self.parse_horizontal_line()?;
                            self.items.push(TopLevelItem::HorizontalLine { line });
                        } else {
                            let list = self.parse_list(1)?;
                            self.items.push(TopLevelItem::List { list });
                        }
                    }
                    '`' => {
                        let code_block = self.parse_code_block()?;
                        self.items.push(TopLevelItem::CodeBlock { code_block });
                    }
                    _ => {
                        let paragraph = self.parse_paragraph()?;
                        self.items.push(TopLevelItem::Paragraph { paragraph });
                    }
                },
                Token::Eof => {
                    return Ok(self.generate());
                }
            }
        }
    }

    pub fn skip_whitespace(&mut self) {
        while let Token::Item(c) = self.lexer.peek() {
            if !c.is_whitespace() {
                break;
            }

            self.lexer.skip();
        }
    }

    pub fn match_until(&mut self, needle: char) -> Result<String> {
        let mut ret = String::with_capacity(256);

        loop {
            match self.lexer.peek() {
                Token::Item(c) if c == needle => {
                    self.lexer.next();
                    return Ok(ret);
                }
                Token::Item(c) => {
                    self.lexer.next();
                    ret.push(c);
                }
                v => bail!("expected characters until {} found {:?}", needle, v),
            }
        }
    }

    pub fn parse_newline(&mut self) -> Result<Newline> {
        debug!("parsing newline");

        if self.lexer.peek() != Token::Newline {
            bail!("expected newline found {:?}", self.lexer.peek());
        };

        self.lexer.skip();

        self.linenr = self.linenr.checked_add(1).ok_or_else(|| anyhow!("line nr overflow"))?;

        Ok(Newline {})
    }

    pub fn parse_header(&mut self) -> Result<Header> {
        debug!("parsing header");

        let mut level = 0_u8;
        while self.lexer.peek() == Token::Item('#') {
            level = level.checked_add(1).ok_or_else(|| anyhow!("header level overflow"))?;
            self.lexer.skip();
        }

        if level > 6 {
            bail!("@line{}: header level must be between 1-6, got {} instead", self.linenr, level);
        }

        self.skip_whitespace();

        let text = self.parse_text()?;

        Ok(Header { level, text })
    }

    pub fn parse_horizontal_line(&mut self) -> Result<HorizontalLine> {
        debug!("parsing horizontal line");

        if self.lexer.peek_n::<3>().iter().any(|v| *v != Token::Item('-')) {
            bail!("horizontal line requires three dashes");
        }

        self.lexer.skip_n::<3>();

        self.parse_newline()?;

        Ok(HorizontalLine {})
    }

    pub fn parse_list(&mut self, level: usize) -> Result<List> {
        debug!("parsing list");

        let mut ret = List {
            r#type: ListType::Unordered,
            items: vec![],
        };

        loop {
            if self.lexer.peek() != Token::Item('-') {
                break;
            }

            let next_level = level.checked_add(1).ok_or_else(|| anyhow!("integer overflow"))?;

            #[allow(clippy::as_conversions)]
            let next = self.lexer.peek_n_dyn(next_level);

            // list ended
            if next.iter().take(level).any(|v| *v != Token::Item('-')) {
                break;
            }

            #[allow(clippy::indexing_slicing, clippy::match_on_vec_items)]
            match next[level] {
                Token::Item('#') => {
                    ret.r#type = ListType::Ordered;
                }
                Token::Item('-') => {
                    ret.items.push(ListItem::SubList(self.parse_list(next_level)?));
                    continue;
                }
                Token::Newline => {
                    bail!("empty list item");
                }
                Token::Eof => {
                    bail!("premature EOF parsing list");
                }
                Token::Item(_) => {}
            }

            self.lexer.skip_n_dyn(next_level);

            self.skip_whitespace();

            ret.items.push(ListItem::Singular(self.parse_text()?));
        }

        if ret.items.is_empty() {
            bail!("no items found in list");
        }

        Ok(ret)
    }

    pub fn parse_text(&mut self) -> Result<Text> {
        debug!("parsing text");

        self.linenr = self.linenr.checked_add(1).ok_or_else(|| anyhow!("line nr overflow"))?;

        let mut items = vec![];

        let mut current_str = String::with_capacity(1024);

        let mut escaped = false;
        let mut current_style_flags_start = TextStyleFlags::empty();
        let mut current_style_flags_end = TextStyleFlags::empty();

        let set_style_flag = |items: &mut Vec<TextItem>,
                              str: &mut String,
                              start_flags: &mut TextStyleFlags,
                              end_flags: &mut TextStyleFlags,
                              appendage,
                              flag| {
            if start_flags.is_empty() {
                items.push(TextItem::Plain {
                    content: StyledText {
                        content: str.clone(),
                        flags: TextStyleFlags::empty(),
                    },
                });
                str.clear();
            }

            if start_flags.contains(flag) {
                if end_flags.contains(flag) {
                    str.push_str(appendage);
                } else {
                    *end_flags |= flag;
                }
            } else {
                *start_flags |= flag;
            }
        };

        loop {
            match Ok(self.lexer.next())? {
                Token::Item(c) => {
                    if escaped {
                        current_str.push(c);
                        escaped = false;
                        continue;
                    }

                    match c {
                        '[' => {
                            let display = self.match_until(']')?;
                            let mut media = false;
                            if self.lexer.peek() == Token::Item('!') {
                                media = true;
                                self.lexer.next();
                            }

                            match self.lexer.peek() {
                                Token::Item('(') => {
                                    self.lexer.next();
                                    let link = self.match_until(')')?;
                                    items.push(if media {
                                        TextItem::MediaLink { display, link }
                                    } else {
                                        TextItem::HyperLink { display, link }
                                    });
                                    continue;
                                }
                                v => bail!("link/media expected opening parenthesis, found {v} instead"),
                            }
                        }
                        '*' => {
                            if self.lexer.peek() == Token::Item('*') {
                                set_style_flag(
                                    &mut items,
                                    &mut current_str,
                                    &mut current_style_flags_start,
                                    &mut current_style_flags_end,
                                    "**",
                                    TextStyleFlags::BOLD,
                                );

                                self.lexer.skip();
                            } else {
                                set_style_flag(
                                    &mut items,
                                    &mut current_str,
                                    &mut current_style_flags_start,
                                    &mut current_style_flags_end,
                                    "*",
                                    TextStyleFlags::ITALIC,
                                );
                            }
                        }
                        '_' => {
                            set_style_flag(
                                &mut items,
                                &mut current_str,
                                &mut current_style_flags_start,
                                &mut current_style_flags_end,
                                "_",
                                TextStyleFlags::UNDERLINE,
                            );
                        }
                        '~' => {
                            set_style_flag(
                                &mut items,
                                &mut current_str,
                                &mut current_style_flags_start,
                                &mut current_style_flags_end,
                                "~",
                                TextStyleFlags::STRIKETHROUGH,
                            );
                        }
                        '`' => {
                            set_style_flag(
                                &mut items,
                                &mut current_str,
                                &mut current_style_flags_start,
                                &mut current_style_flags_end,
                                "`",
                                TextStyleFlags::INLINE_CODE,
                            );
                        }
                        '\\' => escaped = true,
                        _ => current_str.push(c),
                    }

                    if !current_style_flags_start.is_empty() && current_style_flags_start == current_style_flags_end {
                        items.push(TextItem::Plain {
                            content: StyledText {
                                content: current_str.clone(),
                                flags: current_style_flags_end,
                            },
                        });

                        current_style_flags_start = TextStyleFlags::empty();
                        current_style_flags_end = TextStyleFlags::empty();
                        current_str.clear();
                    }
                }
                Token::Newline => {
                    if current_style_flags_start != current_style_flags_end {
                        bail!(
                            "mismatched start & end styling. Start: {:b} End: {:b}",
                            current_style_flags_start,
                            current_style_flags_end
                        );
                    }

                    items.push(TextItem::Plain {
                        content: StyledText {
                            content: current_str.clone(),
                            flags: TextStyleFlags::empty(),
                        },
                    });

                    return Ok(Text { items });
                }
                Token::Eof => bail!("eof without newline"),
            }
        }
    }

    pub fn parse_paragraph(&mut self) -> Result<Paragraph> {
        Ok(Paragraph(self.parse_text()?))
    }

    pub fn parse_code_block(&mut self) -> Result<CodeBlock> {
        debug!("parsing code block");

        if self.lexer.peek_n::<3>().iter().any(|v| *v != Token::Item('`')) {
            bail!("code block must start with three '`'");
        }

        self.lexer.skip_n::<3>();

        let mut lang = String::with_capacity(32);
        let mut content = String::with_capacity(2048);

        while let Token::Item(c) = self.lexer.peek() {
            lang.push(c);
            self.lexer.skip();
        }

        self.parse_newline()?;

        loop {
            match self.lexer.next() {
                Token::Item(c) if c == '`' => match &self.lexer.peek_n::<2>()[..] {
                    &[Token::Item('`'), Token::Item('`')] => {
                        self.lexer.skip_n::<2>();
                        break;
                    }
                    _ => content.push(c),
                },
                Token::Item(c) => content.push(c),
                Token::Newline => {
                    self.linenr = self.linenr.checked_add(1).ok_or_else(|| anyhow!("line nr overflow"))?;

                    content.push('\n');
                }
                t @ Token::Eof => bail!("invalid token {:?}, expected Item | Newline", t),
            }
        }

        Ok(CodeBlock {
            language: if lang.is_empty() { None } else { Some(lang) },
            content,
        })
    }
}
