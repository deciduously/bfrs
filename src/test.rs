#[cfg(test)]
use machine::Machine;
#[cfg(test)]
use lexer::Token::*;
#[cfg(test)]
use lexer::*;
#[cfg(test)]
//use parser::{Command, Construct::{self, Op}, Program};
#[test]
fn test_lex() {
    assert_eq!(
        lex("+-><.,[]"),
        [
            Increment, Decrement, MoveRight, MoveLeft, Output, Input, Open, Close
        ]
    );
}
#[test]
fn test_lex_comment() {
    assert_eq!(
        lex("+-><.,[]x"),
        [
            Increment, Decrement, MoveRight, MoveLeft, Output, Input, Open, Close
        ]
    );
}
#[test]
fn test_increment() {
    let mut test_machine = Machine::new(false);
    &test_machine.increment(1);
    assert_eq!(test_machine.tape[test_machine.index], 1);
}
#[test]
fn test_decrement() {
    let mut test_machine = Machine::new(false);
    test_machine.tape[test_machine.index] = 1;
    test_machine.increment(-1);
    assert_eq!(test_machine.tape[test_machine.index], 0);
}
#[test]
fn test_move_right() {
    let mut test_machine = Machine::new(false);
    test_machine.shift(1);
    assert_eq!(test_machine.index, 1);
}
#[test]
fn test_move_left() {
    let mut test_machine = Machine::new(false);
    test_machine.index = 2;
    test_machine.shift(-1);
    assert_eq!(test_machine.index, 1);
}
#[test]
#[should_panic(expected = "no more room on left of tape")]
fn test_move_left_panic() {
    let mut test_machine = Machine::new(false);
    test_machine.shift(-1);
}
#[test]
#[should_panic(expected = "char at 0 not ascii")]
fn test_out_not_ascii() {
    let mut test_machine = Machine::new(false);
    test_machine.tape[test_machine.index] = 128;
    test_machine.output();
}
//#[test]
//fn test_make_loop() {
//    assert_eq!(
//        Construct::make_loop(lex("<+>-")),
//        Construct::Loop(Program {
//            commands: vec![
//                Op(Command::MoveLeft),
//                Op(Command::Increment),
//                Op(Command::MoveRight),
//                Op(Command::Decrement),
//            ],
//        })
//    );
//}
//#[test]
//fn test_parse_one_loop() {
//    assert_eq!(
//        Program::new(lex("[<+>-]")),
//        Program {
//           commands: vec![
//                Construct::Loop(Box<Program {
//                    commands: vec![
//                       Op(Command::MoveLeft),
//                        Op(Command::Increment),
//                        Op(Command::MoveRight),
//                        Op(Command::Decrement),
//                    ],
//                }>),
//            ],
//        }
//    )
//}
