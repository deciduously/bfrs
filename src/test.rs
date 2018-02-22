#[cfg(test)]
mod tests {
    #[test]
    fn test_translate() {
        use translate;
        use Op::*;
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
        use translate;
        translate('x');
    }
    #[test]
    fn test_parse() {
        use parse;
        use Op::*;

        assert_eq!(parse("+-><.[]"), //REMEMBER TO ADD OP::IN
                   [Inc, Dec, MoveUp, MoveDown, Out, Open, Close]);
    }
    #[test]
    #[should_panic(expected = "Unrecognized char: x")]
    fn test_parse_illegal() {
        use parse;
        parse("+-><.[]x"); //REMEMBER TO ADD OP::IN
    }
    #[test]
    fn test_increment() {
        use Machine;
        let mut test_machine = Machine::new(None);
        test_machine.increment();
        assert_eq!(*test_machine.active, 1);
    }
    #[test]
    #[should_panic(expected = "Cell overflow at 0, could not increment")]
    fn test_increment_overflow() {
        use Machine;
        let mut test_machine = Machine::new(None);
        *test_machine.active = 255;
        test_machine.increment();
    }
    #[test]
    fn test_decrement() {
        use Machine;
        let mut test_machine = Machine::new(None);
        *test_machine.active = 1;
        test_machine.decrement();
        assert_eq!(*test_machine.active, 0);
    }
    #[test]
    #[should_panic(expected = "Cell overflow at 0, could not decrement")]
    fn test_decrement_overflow() {
        use Machine;
        let mut test_machine = Machine::new(None);
        test_machine.decrement();
    }
    #[test]
    fn test_move_up() {
        use Machine;
        let mut test_machine = Machine::new(None);
        test_machine.move_up();
        assert_eq!(test_machine.index, 1);
    }
    #[test]
    #[should_panic(expected = "no more room on right of tape")]
    fn test_move_up_panic() {
        use Machine;
        use TAPE_SIZE;
        let mut test_machine = Machine::new(None);
        test_machine.index = TAPE_SIZE - 1;
        test_machine.active = Box::new(test_machine.tape[test_machine.index]);
        test_machine.move_up();
    }
    #[test]
    fn test_move_down() {
        use Machine;
        let mut test_machine = Machine::new(None);
        test_machine.index = 2;
        test_machine.move_down();
        assert_eq!(test_machine.index, 1);
    }
    #[test]
    #[should_panic(expected = "no more room on left of tape")]
    fn test_move_down_panic() {
        use Machine;
        let mut test_machine = Machine::new(None);
        test_machine.move_down();
    }
    #[test]
    fn test_out() {
        use Machine;
        let mut test_machine = Machine::new(None);
        *test_machine.active = 65;
        assert_eq!(test_machine.out(), 'A');
        *test_machine.active = 97;
        assert_eq!(test_machine.out(), 'a');
        *test_machine.active = 9;
        assert_eq!(test_machine.out(), '\t');
        *test_machine.active = 0;
        assert_eq!(test_machine.out(), '\0');
        *test_machine.active = 2;
        assert_eq!(test_machine.out(), 2u8 as char);
    }
    #[test]
    #[should_panic(expected = "char at 0 not ascii")]
    fn test_out_not_ascii() {
        use Machine;
        let mut test_machine = Machine::new(None);
        *test_machine.active = 128;
        test_machine.out();
    }
    #[test]
    fn test_hello_world() {
        use run;
        assert_eq!(run("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."),
                   "Hello, World!");
    }
}
