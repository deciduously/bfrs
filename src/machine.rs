use parse::{Prog, Op};

pub struct Machine {
    pub tape: Vec<u8>,
    pub index: usize,
    pub pstep: usize,
    pub prog: Prog,
    pub output: Vec<u8>,
}

impl Machine {
    pub fn new(prog: Prog) -> Machine {
        let tape: Vec<u8> = vec![0];
        let index: usize = 0;
        let pstep: usize = 0;
        Machine {
            tape,
            index,
            pstep,
            prog,
            output: Vec::new(),
        }
    }

    //execute for Machine runs all steps in order
    pub fn execute(&mut self) {
        while self.pstep < self.prog.len() {
            //self.dump(); // TODO cmdline option for debug
            match self.prog[self.pstep] {
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

    //todo Impl Debug
    pub fn dump(&self) {
        println!(
            "tape={:?}\nindex={}\npstep={}",
            self.tape, self.index, self.pstep
        );
    }

    //---Operations below---
    pub fn increment(&mut self) {
        self.tape[self.index] += 1;
    }

    pub fn decrement(&mut self) {
        if self.tape[self.index] > 0 {
            self.tape[self.index] -= 1;
        } else {
            panic!("Cell overflow at {}, could not decrement", self.index);
        }
    }

    pub fn move_up(&mut self) {
        if self.index == self.tape.len() - 1 {
            self.tape.push(0);
        }
        self.index += 1;
    }

    pub fn move_down(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            panic!("no more room on left of tape");
        }
    }

    pub fn out(&mut self) {
        if self.tape[self.index].is_ascii() {
            self.output.push(self.tape[self.index] as u8)
        } else {
            panic!("char at {} not ascii", self.index);
        }
    }

    pub fn open(&mut self) {
        let mut bal = 1;
        if self.tape[self.index] == 0 {
            loop {
                self.pstep += 1;
                if self.prog[self.pstep] == Op::Open {
                    bal += 1;
                } else if self.prog[self.pstep] == Op::Close {
                    bal -= 1;
                }
                if bal == 0 {
                    break;
                }
            }
        }
    }

    pub fn close(&mut self) {
        let mut bal = 0;
        loop {
            if self.prog[self.pstep] == Op::Open {
                bal += 1;
            } else if self.prog[self.pstep] == Op::Close {
                bal -= 1;
            }
            self.pstep -= 1; // TODO find a better way than unwinding the stack
            if bal == 0 {
                break;
            }
        }
    }
}
