use lexer::lex;
use machine::Machine;
use parser::{Op, Program};

pub struct BfProgram {
    machine: Machine,
    program: Program,
}

impl BfProgram {
    pub fn new(source: &str, debug: bool) -> BfProgram {
        BfProgram {
            machine: Machine::new(debug),
            program: Program::new(&lex(source)),
        }
    }

    pub fn run(mut self) {
        _run(&self.program.commands, &mut self.machine);
    }
}

fn _run(program: &[Op], machine: &mut Machine) {
    use self::Op::*;

    for op in program {
        if machine.debug {
            println!("{:?}", machine);
        }

        match *op {
            Increment(x) => machine.increment(x),
            Shift(x) => machine.shift(x),
            Print => machine.output(),
            Input => machine.input(),
            Loop(ref ops) => while machine.curr() > 0 {
                _run(ops, machine);
            },
        }
    }
}
