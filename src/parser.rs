use lexer::{Token, Tokens};

#[derive(Debug, PartialEq)]
pub enum Op {
    Increment(i32),
    Shift(isize),
    Loop(Box<[Op]>),
    Print,
    Input,
}

impl Op {
    // TODO Result
    pub fn from_token(token: Token) -> Op {
        use self::Op::*;

        match token {
            Token::Increment => Increment(1),
            Token::Decrement => Increment(-1),
            Token::MoveRight => Shift(1),
            Token::MoveLeft => Shift(-1),
            Token::Output => Print,
            Token::Input => Input,
            _ => panic!("Reached a loop char where there should not be one"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub commands: Box<[Op]>,
}

impl Program {
    // TODO return a Result
    pub fn new(source: &Tokens) -> Program {
        let mut pstep: usize = 0;

        let mut ret = Vec::new();

        while pstep < source.len() {
            match source[pstep] {
                Token::Open => {
                    // Op::Loop(Program::new())
                    let mut loop_body = Vec::new();
                    let mut bal = 1;

                    loop {
                        pstep += 1;

                        if source[pstep] == Token::Open {
                            bal += 1;
                        }

                        if source[pstep] == Token::Close {
                            bal -= 1;
                        }

                        if bal == 0 {
                            break;
                        }
                        loop_body.push(source[pstep]);
                        // if EOF, unmatched '['?
                    }
                    ret.push(Op::Loop(Program::new(&loop_body).commands));
                }
                Token::Close => panic!("Unmatched ']'"),
                _ => ret.push(Op::from_token(source[pstep])),
            }
            pstep += 1;
        }

        Program {
            commands: ret.into_boxed_slice(),
        }
    }
}
