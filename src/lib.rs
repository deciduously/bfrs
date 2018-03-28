//TODO docstringz yo
mod test;

pub const TAPE_SIZE: usize = 30_000; //TODO consider making it unlimited to the right

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
    tape: [u8; TAPE_SIZE],
    index: usize,
    output: Vec<u8>,
}

impl Machine {
    //TODO look at using Into<Option<T>>
    //http://xion.io/post/code/rust-optional-args.html for example
    fn new() -> Machine {
        let tape: [u8; TAPE_SIZE] = [0; TAPE_SIZE];
        let index: usize = 0;
        Machine {
            tape: tape,
            index: index,
            output: Vec::new(),
        }
    }

    // THIS IS YOU FROM THE FUTURE.  The BLEAK, BLEAK 7 MONTHS FUTURE.
    // Oh gosh, I got sad.  Things got weird.
    // And YOU ARE FUNDAMENTALLY INCORRECT ABOUT THIS but the rest of it should be ok for now.
    // In fact, you've already done a lot of the fiddly stuff!  Yay, you.  You're pretty close.
    //
    // By now, though, you've learned you must AST that shizz.
    // Which you may have been trying to do here, but I cannot tell.
    // You know, read it in, evaluate the forms recursively, return the result.
    // Rebox is a dumb function, you probably knew it was dumb when you wrote it but you didn't know any better.
    // I forgive you.
    // Go forth, present Ben.  Fuck this brain like it's probably been fucked millions of times before.
    // Aww, yeah.

    //execute for Machine runs all steps in order
    fn execute(&mut self, steps: Steps) {
        for step in steps.iter() {
            match *step {
                Op::Inc => self.increment(),
                Op::Dec => self.decrement(),
                Op::MoveDown => self.move_down(),
                Op::MoveUp => self.move_up(),
                Op::Out => self.out(),
                _ => panic!("Something went horribly wrong - tried to executing something that wasn't an OP")
            };
        }
    }
    //---Operations below---
    fn increment(&mut self) {
        if self.tape[self.index] < 255 {
            self.tape[self.index] += 1;
        } else {
            panic!("Cell overflow at {}, could not increment", self.index); //TODO should it wrap?
        }
    }

    fn decrement(&mut self) {
        if self.tape[self.index] > 0 {
            self.tape[self.index] -= 1;
        } else {
            panic!("Cell overflow at {}, could not decrement", self.index);
        }
    }

    fn move_up(&mut self) {
        if self.index < TAPE_SIZE - 1 {
            self.index += 1;
        } else {
            panic!("no more room on right of tape");
        }
    }

    fn move_down(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            panic!("no more room on left of tape");
        }
    }

    fn out(&mut self) {
        use std::ascii::AsciiExt;
        if self.tape[self.index].is_ascii() {
            self.output.push(self.tape[self.index] as u8)
        } else {
            panic!("char at {} not ascii", self.index);
        }
    }

    //fn open(&mut self) {
    //    self.loop_open = Some(self.index);
    //    if *self.active == 0 {
    //        self.index = self.loop_close.unwrap() + 1;
    //        self.rebox();
    //    } else {
    //        self.index += 1;
    //        self.rebox();
    //    }
    //}

    //fn close(&mut self) {
    //    self.loop_close = Some(self.index);
    //    if *self.active != 0 {
    //        self.index = self.loop_open.unwrap() + 1;
    //        self.rebox();
    //    } else {
    //        self.index += 1;
    //        self.rebox();
    //    }
   //}
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
    let mut machine = Machine::new();
    machine.execute(steps);
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
