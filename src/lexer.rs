pub type Tokens = Vec<Token>;
//Box<[Token]>?

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Increment,
    Decrement,
    MoveRight,
    MoveLeft,
    Output,
    Input,
    Open,
    Close,
}

pub fn lex(input: &str) -> Tokens {
    let mut ret = Vec::new();

    for c in input.chars() {
        use self::Token::*;

        if let Some(op) = match c {
            '+' => Some(Increment),
            '-' => Some(Decrement),
            '>' => Some(MoveRight),
            '<' => Some(MoveLeft),
            '.' => Some(Output),
            ',' => Some(Input),
            '[' => Some(Open),
            ']' => Some(Close),
            _ => None,
        } {
            ret.push(op);
        }
    }
    ret
}
