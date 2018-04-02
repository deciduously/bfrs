use parser::Program;
use std::io::{stdin, Read};

#[derive(Debug)]
pub struct Machine {
    pub tape: Vec<u8>,
    pub index: usize,
    //pub output: Vec<u8>,
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            tape: vec![0],
            index: 0,
            //output: Vec::new(),
            // debug bool?
        }
    }

   // pub fn execute(mut self, program: Program, debug: bool) -> String {
   //     program.run(self, debug); // value moved here.
   //     String::from_utf8(self.output.clone()).expect("could not parse output")
   // }

    pub fn increment(&mut self) {
        if self.tape[self.index] == 255 {
            self.tape[self.index] = 0;
        } else {
            self.tape[self.index] += 1;
        }
    }

    pub fn decrement(&mut self) {
        if self.tape[self.index] == 0 {
            self.tape[self.index] = 255;
        } else {
            self.tape[self.index] -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.index == self.tape.len() - 1 {
            self.tape.push(0);
        }
        self.index += 1;
    }

    pub fn move_left(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            panic!("no more room on left of tape");
        }
    }

    pub fn output(&self) {
        if self.tape[self.index].is_ascii() {
            print!("{}", self.tape[self.index] as char)
        } else {
            panic!("char at {} not ascii", self.index);
        }
    }

    pub fn input(&mut self) {
        let mut in_char: [u8; 1] = [0];
        stdin()
            .read_exact(&mut in_char)
            .expect("Could not read byte");
        self.tape[self.index] = in_char[0];
    }
}
