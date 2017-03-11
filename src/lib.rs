//TODO docstringz yo

pub const TAPE_SIZE: usize = 30_000;  //consider making it unlimited to the right

type Steps = Vec<Op>; //TODO prefer stack allocation if possible?  Vectors are convenient but heap

//TODO think about extending unlimited on the right for Turing-completeness
struct Machine {
    tape: [u8; TAPE_SIZE],
    pos: usize, //TODO use ptr to active cell instead of index
    steps: Steps,
}

impl Machine {
    //TODO look at using Into<Option<T>>
    //http://xion.io/post/code/rust-optional-args.html for example
    fn new(steps: Option<Steps>) -> Machine {
        Machine {
            tape: [0; TAPE_SIZE],
            pos: 0,
            steps: steps.unwrap_or(Vec::new()),
        }
    }
    //execute for Machine returns the Op completed, or an error code
    //fn execute(&mut self) -> Result<Op, u8> {

    //}
    //run() for machine will execute() all steps in order until empty

    //---Operations below---
    fn increment(&mut self) {
        if self.tape[self.pos] < 255 {
            self.tape[self.pos] += 1;
        } else {
            panic!("Cell overflow at {}", self.pos); //TODO should it wrap?
        }
    }

    fn decrement(&mut self) {
        if self.tape[self.pos] > 0 {
            self.tape[self.pos] -= 1;
        } else {
            panic!("Cell overflow at {}", self.pos);
        }
    }

    fn move_up(&mut self) {
        if self.pos < TAPE_SIZE - 1 {
            self.pos += 1;
        } else {
            panic!("no more room on right of tape");
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Inc,
    Dec,
    MoveUp,
    MoveDown,
    Out,
    In,
    Open,
    Close,
}

fn parse(input: &str) -> Steps {
    let mut ret = Vec::new();
    for c in input.chars() {
        ret.push(translate(c));
    }
    ret
}

//pub fn run(input: &str) {
//    let steps = parse(input);
//    let machine = Machine::new(steps);
//    machine::execute();
//}

pub fn translate(symbol: char) -> Op {
    use Op::*;

    match symbol {
        '+' => Inc,
        '-' => Dec,
        '>' => MoveUp,
        '<' => MoveDown,
        '.' => Out,
        ',' => In,
        '[' => Open,
        ']' => Close,
        _ => panic!("Unrecognized char: {}", symbol),
    }
}

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
        assert_eq!(translate(','), In);
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

        assert_eq!(parse("+-><.,[]"),
                   [Inc, Dec, MoveUp, MoveDown, Out, In, Open, Close]);
    }
    #[test]
    #[should_panic(expected = "Unrecognized char: x")]
    fn test_parse_illegal() {
        use parse;
        parse("+-><.,[]x");
    }
    #[test]
    fn test_increment() {
        use Machine;
        let mut test_machine = Machine::new(None);
        test_machine.increment();
        assert_eq!(test_machine.tape[test_machine.pos], 1);
    }
    #[test]
    fn test_decrement() {
        use Machine;
        let mut test_machine = Machine::new(None);
        test_machine.tape[test_machine.pos] = 1;
        test_machine.decrement();
        assert_eq!(test_machine.tape[test_machine.pos], 0);
    }
    #[test]
    fn test_move_up() {
        use Machine;
        let mut test_machine = Machine::new(None);
        test_machine.move_up();
        assert_eq!(test_machine.pos, 1);
    }
    #[test]
    #[should_panic(expected = "no more room on right of tape")]
    fn test_move_up_panic() {
        use Machine;
        use TAPE_SIZE;
        let mut test_machine = Machine::new(None);
        test_machine.pos = TAPE_SIZE - 1;
        test_machine.move_up();
    }
    //fn test_hello_world() {
    //   use run;
    //   assert_eq!(run(
    //    "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."
    //    ),
    //             "Hello, World!");
    //}
}
