//TODO docstringz yo
mod test;

//pub const TAPE_SIZE: usize = 30_000;

type Steps = Vec<Op>;

#[derive(Debug, PartialEq)]
pub enum Op {
    Inc,
    Dec,
    MoveUp,
    MoveDown,
    Out,
    //In, --NOT REQUIRED FOR GETTING HELLO_WORLD TO PASS
    Open,
    Close,
}

struct Machine {
    tape: Vec<u8>,
    index: usize,
    pstep: usize,
    steps: Steps,
    output: Vec<u8>,
}

impl Machine {
    //TODO look at using Into<Option<T>>
    //http://xion.io/post/code/rust-optional-args.html for example
    fn new(steps: Steps) -> Machine {
        let tape: Vec<u8> = vec![0];
        let index: usize = 0;
        let pstep: usize = 0;
        Machine {
            tape,
            index,
            pstep,
            steps,
            output: Vec::new(),
        }
    }

    //execute for Machine runs all steps in order
    fn execute(&mut self) {
        while self.pstep < self.steps.len() {
            self.dump(); // DEBUG
            match self.steps[self.pstep] {
                Op::Inc => self.increment(),
                Op::Dec => self.decrement(),
                Op::MoveDown => self.move_down(),
                Op::MoveUp => self.move_up(),
                Op::Out => self.out(),
                Op::Open => self.open(),
                Op::Close => self.close(),
            };
            self.pstep += 1;
        }
    }
    fn dump(&self) {
        println!(
            "tape={:?}\nindex={}\npstep={}",
            self.tape, self.index, self.pstep
        );
    }

    //---Operations below---
    fn increment(&mut self) {
        self.tape[self.index] += 1;
    }

    fn decrement(&mut self) {
        if self.tape[self.index] > 0 {
            self.tape[self.index] -= 1;
        } else {
            panic!("Cell overflow at {}, could not decrement", self.index);
        }
    }

    fn move_up(&mut self) {
        if self.index == self.tape.len() - 1 {
            self.tape.push(0);
        }
        self.index += 1;
    }

    fn move_down(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            panic!("no more room on left of tape");
        }
    }

    fn out(&mut self) {
        if self.tape[self.index].is_ascii() {
            self.output.push(self.tape[self.index] as u8)
        } else {
            panic!("char at {} not ascii", self.index);
        }
    }

    fn open(&mut self) {
        let mut bal = 1;
        if self.tape[self.index] == 0 {
            loop {
                self.pstep += 1;
                if self.steps[self.pstep] == Op::Open {
                    bal += 1;
                } else if self.steps[self.pstep] == Op::Close {
                    bal -= 1;
                }
                if bal == 0 {
                    break;
                }
            }
        }
    }

    fn close(&mut self) {
        let mut bal = 0;
        loop {
            if self.steps[self.pstep] == Op::Open {
                bal += 1;
            } else if self.steps[self.pstep] == Op::Close {
                bal -= 1;
            }
            self.pstep -= 1; // TODO find a better way than unwinding the stack
            if bal == 0 {
                break;
            }
        }
    }
}

fn parse(input: &str) -> Steps {
    let mut ret = Vec::new();
    for c in input.chars() {
        ret.push(translate(c));
    }
    ret
}

pub fn run(input: &str) -> String {
    let steps = parse(input);
    let mut machine = Machine::new(steps);
    machine.execute();
    String::from_utf8(machine.output).unwrap()
}

pub fn translate(symbol: char) -> Op {
    use Op::*;

    match symbol {
        '+' => Inc,
        '-' => Dec,
        '>' => MoveUp,
        '<' => MoveDown,
        '.' => Out,
        //',' => In, --NOT REQUIRED FOR HELLO_WORLD
        '[' => Open,
        ']' => Close,
        _ => panic!("Unrecognized char: {}", symbol),
    }
}
