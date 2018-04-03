use std::io::{self, Read};

#[derive(Debug)]
pub struct Machine {
    pub tape: Vec<i32>,
    pub index: usize,
    pub debug: bool,
}

impl Machine {
    pub fn new(debug: bool) -> Machine {
        Machine {
            tape: vec![0],
            index: 0,
            debug,
        }
    }

    pub fn curr(&self) -> i32 {
        self.tape[self.index]
    }

    pub fn curr_char(&self) -> char {
        self.curr() as u8 as char
    }

    pub fn increment(&mut self, offset: i32) {
        self.tape[self.index] += offset;
    }

    pub fn shift(&mut self, offset: isize) {
        if self.index as isize + offset < 0 {
            panic!("no more room on left of tape!")
        }
        self.index = (self.index as isize + offset) as usize;
        if self.index >= self.tape.len() {
            self.tape.push(0);
        }
    }

    pub fn output(&self) {
        if self.curr_char().is_ascii() {
            print!("{}", self.curr_char());
            io::Write::flush(&mut io::stdout()).unwrap();
        } else {
            panic!("char at {} not ascii", self.index);
        }
    }

    pub fn input(&mut self) {
        let mut in_char: [u8; 1] = [0];
        io::stdin()
            .read_exact(&mut in_char)
            .expect("Could not read byte");
        self.tape[self.index] = i32::from(in_char[0]);
    }
}
