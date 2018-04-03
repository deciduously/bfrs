use machine::Machine;
use lexer::lex;
use parser::Program;

pub struct BfProgram {
    machine: Machine,
    program: Program,
}

impl BfProgram {
    pub fn new(source: &str, debug: bool) -> BfProgram {
        BfProgram {
            machine: Machine::new(debug),
            program: Program::new(lex(source)),
        }
    }

    pub fn run(mut self) {
        self.program.run(&mut self.machine);
    }
}
