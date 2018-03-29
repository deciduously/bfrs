#[cfg(test)]
use machine::Machine;
use parse::Op::*;
use parse::*;

#[test]
fn test_translate() {
    assert_eq!(translate('+'), Inc);
    assert_eq!(translate('-'), Dec);
    assert_eq!(translate('<'), MoveDown);
    assert_eq!(translate('>'), MoveUp);
    assert_eq!(translate('.'), Out);
    //assert_eq!(translate(','), In);  --NOT REQUIRED FOR HELLO_WORLD
    assert_eq!(translate('['), Open);
    assert_eq!(translate(']'), Close);
}
#[test]
#[should_panic(expected = "Unrecognized char: x")]
fn test_translate_unrecognized() {
    translate('x');
}
#[test]
fn test_parse() {
    assert_eq!(
        parse("+-><.[]"), //REMEMBER TO ADD OP::IN
        [Inc, Dec, MoveUp, MoveDown, Out, Open, Close]
    );
}
#[test]
#[should_panic(expected = "Unrecognized char: x")]
fn test_parse_illegal() {
    parse("+-><.[]x"); //REMEMBER TO ADD OP::IN
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
#[should_panic(expected = "Cell overflow at 0, could not decrement")]
fn test_decrement_overflow() {
    let mut test_machine = Machine::new(Vec::new());
    test_machine.decrement();
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
    assert_eq!(run("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."),
                   "Hello World!\n");
}
#[test]
fn test_no_loops() {
    use super::run;
    assert_eq!(
        run("++++++++++++++++++++++++++++++++++++++++++++++++."),
        "0"
    )
}
#[test]
fn test_simple_loop() {
    use super::run;
    assert_eq!(run("++>+++++[<+>-]++++++++[<++++++>-]<."), "7")
}
