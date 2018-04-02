use machine::Machine;
use lexer::lex;
use parser::Program;

pub fn run(source: &str, debug: bool) -> String {
    let mut machine = Machine::new();
    let program = Program::new(lex(source));
    machine.execute(program, debug)
}
