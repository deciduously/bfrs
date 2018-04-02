use lexer::{Token, Tokens};
use machine::Machine;

#[derive(Clone, Debug, PartialEq)]
pub enum Construct {
    Op(Command),
    Loop(Program),
}

impl Construct {
    pub fn make_loop(loop_body: &Tokens) -> Construct {
        Construct::Loop(Program::new(loop_body))
    }

    pub fn from_token(token: &Token) -> Construct {
        use self::{Command::*, Construct::Op};

        match *token {
            Token::Increment => Op(Increment),
            Token::Decrement => Op(Decrement),
            Token::MoveRight => Op(MoveRight),
            Token::MoveLeft => Op(MoveLeft),
            Token::Output => Op(Output),
            Token::Input => Op(Input),
            _ => panic!("Reached a loop char where there should not be one"),
        }
    }

    pub fn run(self, machine: &mut Machine, debug: bool) {
        use self::Construct::*;

        match self {
            Op(command) => command.run(machine),
            Loop(commands) => while machine.tape[machine.index] != 0 {
                commands.clone().run(machine, debug); // cloning because we loop over it
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Increment,
    Decrement,
    MoveRight,
    MoveLeft,
    Output,
    Input,
}

impl Command {
    pub fn run(self, machine: &mut Machine) {
        use self::Command::*;
        match self {
            Increment => machine.increment(),
            Decrement => machine.decrement(),
            MoveLeft => machine.move_left(),
            MoveRight => machine.move_right(),
            Output => machine.output(),
            Input => machine.input(),
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub commands: Vec<Construct>,
}

impl Program {
    pub fn new(tokens: &Tokens) -> Program {
        let mut pstep: usize = 0;

        let mut ret: Vec<Construct> = Vec::new();

        while pstep < tokens.len() {
            match tokens[pstep] {
                Token::Open => {
                    let mut loop_body = Vec::new();
                    let mut bal = 1;

                    loop {
                        pstep += 1;
                        let curr = &tokens[pstep];

                        if curr == &Token::Open {
                            bal += 1;
                        }

                        if curr == &Token::Close {
                            bal -= 1;
                        }

                        if bal == 0 {
                            break;
                        }
                        loop_body.push(curr.clone());
                        // if EOF, unmatched '['?
                    }
                    ret.push(Construct::make_loop(&loop_body));
                },
                Token::Close => panic!("Unmatched ']'"),
                _ => ret.push(Construct::from_token(&tokens[pstep])),
            }
            pstep += 1;
        }

        Program { commands: ret }
    }

    pub fn run(self, machine: &mut Machine, debug: bool) {
        for command in self.commands {
            if debug {
                println!("{:?}", machine);
            }
            command.run(machine, debug);
        }
    }
}
