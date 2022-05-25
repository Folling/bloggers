use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Token {
    Item(char),
    Newline,
    Eof,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Item(c) => write!(f, "Item({})", c),
            Self::Newline => write!(f, "Newline"),
            Self::Eof => write!(f, "EOF"),
        }
    }
}

pub struct Lexer {
    content: Vec<char>,
    current_idx: usize,
    current_token: Option<Token>,
}

impl Lexer {
    pub fn new<S: AsRef<str>>(content: S) -> Self {
        Self {
            content: content.as_ref().chars().collect(),
            current_idx: 0,
            current_token: Some(Token::Newline),
        }
    }

    #[allow(clippy::manual_assert)] // false positive, assert doesn't work in const funcs
    const fn assert_size_nz<const COUNT: usize>() {
        if COUNT == 0 {
            panic!("count must not be zero");
        }
    }

    pub const fn current(&self) -> Option<Token> {
        self.current_token
    }

    pub fn next(&mut self) -> Token {
        let ret = match self.content.get(self.current_idx) {
            None => Token::Eof,
            Some('\n') => Token::Newline,
            Some(c) => Token::Item(*c),
        };

        if let Some(i) = self.current_idx.checked_add(1) {
            self.current_idx = i;
        } else {
            return Token::Eof;
        }

        self.current_token.replace(ret);

        ret
    }

    pub fn next_n<const COUNT: usize>(&mut self) -> [Token; COUNT] {
        Self::assert_size_nz::<COUNT>();

        let mut ret = [Token::Newline; COUNT];

        for v in ret.iter_mut() {
            *v = self.next();
        }

        self.current_token = Some(*ret.last().expect("empty element array"));

        ret
    }

    pub fn next_n_dyn(&mut self, count: usize) -> Vec<Token> {
        let mut ret = Vec::new();

        for v in (0..count).map(|_| self.next()) {
            ret.push(v);
        }

        self.current_token = Some(*ret.last().expect("empty element array"));

        ret
    }

    pub fn skip(&mut self) {
        self.next();
    }

    pub fn skip_n<const COUNT: usize>(&mut self) {
        self.next_n::<COUNT>();
    }

    pub fn skip_n_dyn(&mut self, count: usize) {
        self.next_n_dyn(count);
    }

    pub fn prev(&mut self) -> Option<Token> {
        let ret = match self.content.get(self.current_idx) {
            None => Some(Token::Eof),
            Some('\n') => Some(Token::Newline),
            Some(c) => Some(Token::Item(*c)),
        };

        self.current_idx = self.current_idx.checked_sub(1)?;

        ret
    }

    pub fn prev_n<const COUNT: usize>(&mut self) -> [Option<Token>; COUNT] {
        Self::assert_size_nz::<COUNT>();

        let mut ret = [None; COUNT];

        for v in ret.iter_mut() {
            *v = self.prev();
        }

        self.current_token = *ret.last().expect("empty element array");

        ret
    }

    pub fn skip_prev(&mut self) {
        self.prev();
    }

    pub fn skip_prev_n<const COUNT: usize>(&mut self) {
        self.prev_n::<COUNT>();
    }

    pub fn peek(&self) -> Token {
        match self.content.get(self.current_idx) {
            None => Token::Eof,
            Some('\n') => Token::Newline,
            Some(c) => Token::Item(*c),
        }
    }

    pub fn peek_n<const COUNT: usize>(&self) -> [Token; COUNT] {
        let mut ret = [Token::Newline; COUNT];

        let mut current_idx = self.current_idx;

        for v in ret.iter_mut() {
            match self.content.get(current_idx) {
                None => break,
                Some('\n') => *v = Token::Newline,
                Some(c) => *v = Token::Item(*c),
            }

            current_idx = current_idx.checked_add(1).expect("overflowing idx during parse");
        }

        ret
    }

    pub fn peek_n_dyn(&self, count: usize) -> Vec<Token> {
        let mut ret = Vec::new();

        for i in 0..count {
            match self.current_idx.checked_add(i).and_then(|i| self.content.get(i)) {
                None => break,
                Some('\n') => ret.push(Token::Newline),
                Some(c) => ret.push(Token::Item(*c)),
            }
        }

        ret
    }

    pub fn peek_prev(&self) -> Option<Token> {
        match self.content.get(self.current_idx.checked_sub(1)?) {
            None => None,
            Some('\n') => Some(Token::Newline),
            Some(c) => Some(Token::Item(*c)),
        }
    }

    pub fn peek_prev_n<const COUNT: usize>(&self) -> [Option<Token>; COUNT] {
        let mut ret = [Some(Token::Newline); COUNT];

        for (i, v) in ret.iter_mut().enumerate() {
            match i
                .checked_add(1)
                .and_then(|i| self.current_idx.checked_sub(i).and_then(|v| self.content.get(v)))
            {
                None => break,
                Some('\n') => *v = Some(Token::Newline),
                Some(c) => *v = Some(Token::Item(*c)),
            }
        }

        ret
    }
}
