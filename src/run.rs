use machine::Machine;
use lexer::lex;
use parser::Program;

pub fn run(source: &str, debug: bool) {
    let mut machine = Machine::new();
    let program = Program::new(&lex(source));
    program.run(&mut machine, debug);
}
