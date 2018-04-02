use machine::Machine;
use lexer::lex;
use parser::Program;

pub struct BfProgram {
    machine: Machine,
    program: Program,
    debug: bool,
}

impl BfProgram {
    pub fn new(source: &str, debug: bool) -> BfProgram {
        BfProgram {
            machine: Machine::new(),
            program: Program::new(lex(source)),
            debug,
        }
    }

    pub fn run(mut self) {
        self.program.run(&mut self.machine, self.debug);
    }
}
