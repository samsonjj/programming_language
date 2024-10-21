use std::{
    borrow::{Borrow, Cow},
    iter::Peekable,
    str::Chars,
    vec::IntoIter,
};

enum Token {
    Function,
    Identifier,
    LParen,
    RParen,
    LQuote,
    RQuote,
    StrContent,
    SemiColon,
}

struct Tokenizer {
    char_iter: Peekable<IntoIter<char>>,
}

impl Tokenizer {
    fn new(data: Cow<str>) -> Self {
        let code = match data {
            Cow::Borrowed(x) => x.to_owned(),
            Cow::Owned(x) => x,
        };

        let char_vec: Vec<char> = code.chars().collect();

        Self {
            char_iter: char_vec.into_iter().peekable(),
        }
    }
    fn peek(&mut self) -> char {
        *self.char_iter.peek().unwrap()
    }
    fn next(&mut self) -> char {
        self.char_iter.next().unwrap()
    }
    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let mut stored = String::new();
        loop {
            if self.peek() == ' ' {
                tokens.push(Self::token_from_str(stored.as_str()));
            }
            stored.push(self.next())
        }

        tokens
    }

    fn token_from_char(c: char) -> Option<Token> {
        match c {
            '(' => Some(Token::RParen),
            ')' => Some(Token::LParen),
            _ => None,
        }
    }

    fn token_from_str(token_str: &str) -> Token {
        match token_str {
            "function" => Token::Function,
            _ => panic!("invalid token {}", token_str),
        }
    }
}

fn main() {}
