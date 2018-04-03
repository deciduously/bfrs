use lexer::{Token, Tokens};
use machine::Machine;

#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    Increment(i32),
    Shift(isize),
    Loop(Program),
    Print,
    Input,
}

impl Op {
    // TODO Result
    pub fn from_token(token: &Token) -> Op {
        use self::Op::*;

        match *token {
            Token::Increment => Increment(1),
            Token::Decrement => Increment(-1),
            Token::MoveRight => Shift(1),
            Token::MoveLeft => Shift(-1),
            Token::Output => Print,
            Token::Input => Input,
            _ => panic!("Reached a loop char where there should not be one"),
        }
    }

    pub fn run(self, machine: &mut Machine) {
        use self::Op::*;
        match self {
            Increment(x) => machine.increment(x),
            Shift(x) => machine.shift(x),
            Print => machine.output(),
            Input => machine.input(),
            Loop(ref ops) => while machine.curr() > 0 {
                ops.clone().run(machine); // TODO don't clone
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub commands: Box<[Op]>,
}

impl Program {
    // TODO return a Result
    pub fn new(tokens: Tokens) -> Program {
        let mut pstep: usize = 0;

        let mut ret = Vec::new();

        while pstep < tokens.len() {
            match tokens[pstep] {
                Token::Open => {
                    let mut loop_body = Vec::new();
                    let mut bal = 1;

                    loop {
                        pstep += 1;

                        if tokens[pstep] == Token::Open {
                            bal += 1;
                        }

                        if tokens[pstep] == Token::Close {
                            bal -= 1;
                        }

                        if bal == 0 {
                            break;
                        }
                        loop_body.push(tokens[pstep].clone());
                        // if EOF, unmatched '['?
                    }
                    ret.push(Op::Loop(Program::new(loop_body)));
                }
                Token::Close => panic!("Unmatched ']'"),
                _ => ret.push(Op::from_token(&tokens[pstep])),
            }
            pstep += 1;
        }

        Program { commands: ret.into_boxed_slice() }
    }

    pub fn run(self, machine: &mut Machine) {
        for command in self.commands.iter() {
            if machine.debug {
                println!("{:?}", machine);
            }
            command.clone().run(machine); // TODO don't clone
        }
    }
}
