//TODO docstringz yo
mod machine;
mod parse;
mod test;

use machine::Machine;
use parse::parse;

pub fn run(input: &str) -> String {
    let steps = parse(input);
    let mut machine = Machine::new(steps);
    machine.execute();
    String::from_utf8(machine.output).unwrap()
}

