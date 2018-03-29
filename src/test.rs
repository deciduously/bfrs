#[cfg(test)]
use machine::Machine;
use parse::Op::*;
use parse::*;

#[test]
fn test_translate() {
    assert_eq!(translate('+'), Some(Inc));
    assert_eq!(translate('-'), Some(Dec));
    assert_eq!(translate('<'), Some(MoveDown));
    assert_eq!(translate('>'), Some(MoveUp));
    assert_eq!(translate('.'), Some(Out));
    assert_eq!(translate(','), Some(In));
    assert_eq!(translate('['), Some(Open));
    assert_eq!(translate(']'), Some(Close));
    assert_eq!(translate('x'), None);
}
#[test]
fn test_parse() {
    assert_eq!(
        parse("+-><.,[]"),
        [Inc, Dec, MoveUp, MoveDown, Out, In, Open, Close]
    );
}
#[test]
fn test_parse_illegal() {
    assert_eq!(
        parse("+-><.,[]x"),
        [Inc, Dec, MoveUp, MoveDown, Out, In, Open, Close]
    );
}
#[test]
fn test_increment() {
    let mut test_machine = Machine::new(Vec::new());
    test_machine.increment();
    assert_eq!(test_machine.tape[test_machine.index], 1);
}
#[test]
fn test_decrement() {
    let mut test_machine = Machine::new(Vec::new());
    test_machine.tape[test_machine.index] = 1;
    test_machine.decrement();
    assert_eq!(test_machine.tape[test_machine.index], 0);
}
#[test]
fn test_move_up() {
    let mut test_machine = Machine::new(Vec::new());
    test_machine.move_up();
    assert_eq!(test_machine.index, 1);
}
#[test]
fn test_move_down() {
    let mut test_machine = Machine::new(Vec::new());
    test_machine.index = 2;
    test_machine.move_down();
    assert_eq!(test_machine.index, 1);
}
#[test]
#[should_panic(expected = "no more room on left of tape")]
fn test_move_down_panic() {
    let mut test_machine = Machine::new(Vec::new());
    test_machine.move_down();
}
#[test]
fn test_out() {
    let mut test_machine = Machine::new(Vec::new());

    test_machine.tape[test_machine.index] = 65;
    test_machine.out();
    assert_eq!(test_machine.output[0], 'A' as u8);

    test_machine.tape[test_machine.index] = 97;
    test_machine.out();
    assert_eq!(test_machine.output[1], 'a' as u8);

    test_machine.tape[test_machine.index] = 9;
    test_machine.out();
    assert_eq!(test_machine.output[2], '\t' as u8);

    test_machine.tape[test_machine.index] = 0;
    test_machine.out();
    assert_eq!(test_machine.output[3], '\0' as u8);

    test_machine.tape[test_machine.index] = 2;
    test_machine.out();
    assert_eq!(test_machine.output[4], 2u8);
}
#[test]
#[should_panic(expected = "char at 0 not ascii")]
fn test_out_not_ascii() {
    let mut test_machine = Machine::new(Vec::new());
    test_machine.tape[test_machine.index] = 128;
    test_machine.out();
}
#[test]
fn test_hello_world() {
    use super::run;
    assert_eq!(run("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.", false),
                   "Hello World!\n");
}
#[test]
fn test_no_loops() {
    use super::run;
    assert_eq!(
        run("++++++++++++++++++++++++++++++++++++++++++++++++.", false),
        "0"
    )
}
#[test]
fn test_simple_loop() {
    use super::run;
    assert_eq!(run("++>+++++[<+>-]++++++++[<++++++>-]<.", false), "7")
}
