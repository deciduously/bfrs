use machine::Machine;
use parse::parse;

pub fn run(input: &str, debug: bool) -> String {
    let program = parse(input);
    let mut machine = Machine::new(program);
    machine.execute(debug);
    String::from_utf8(machine.output).unwrap()
}
