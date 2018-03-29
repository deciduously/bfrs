//TODO docstringz yo
mod machine;
mod parse;
mod test;

use machine::Machine;
use parse::parse;

pub fn run(input: &str, debug: bool) -> String {
    let steps = parse(input);
    let mut machine = Machine::new(steps);
    machine.execute(debug);
    String::from_utf8(machine.output).unwrap()
}
