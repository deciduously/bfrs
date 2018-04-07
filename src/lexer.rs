use std::{io, str::FromStr};
pub type Tokens = Vec<Token>;

pub fn lex(s: &str) -> Tokens {
    let mut ret = Vec::new();
    for c in s.split("") {
        if let Ok(token) = Token::from_str(c) {
            ret.push(token);
        }
    }

    ret
}

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq)]
pub enum Token {
    Increment,
    Decrement,
    MoveRight,
    MoveLeft,
    Output,
    Input,
    Open,
    Close,
    Debug,
}

impl FromStr for Token {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Token::*;

        let token = match s {
            "+" => Some(Increment),
            "-" => Some(Decrement),
            ">" => Some(MoveRight),
            "<" => Some(MoveLeft),
            "." => Some(Output),
            "," => Some(Input),
            "[" => Some(Open),
            "]" => Some(Close),
            "#" => Some(Debug),
            _ => None,
        };

        match token {
            Some(token) => Ok(token),
            None => Err(io::Error::new(io::ErrorKind::Other, "unrecognized token")),
        }
    }
}
