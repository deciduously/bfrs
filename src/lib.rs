//TODO docstringz yo

pub const TAPE_SIZE: usize = 30_000; //TODO consider making it unlimited to the right

type Steps = Vec<Op>;

#[derive(Debug, PartialEq)]
pub enum Op {
    Inc,
    Dec,
    MoveUp,
    MoveDown,
    Out,
    //In, --NOT REQUIRED FOR GETTING HELLO_WORLD TO PASS
    Open,
    Close,
}

struct Machine {
    tape: [u8; TAPE_SIZE],
    index: usize, //TODO Storing the index and generating a new Box each move feels clunky
    active: Box<u8>,
    steps: Steps,
    loop_open: Option<usize>, //this also seems clunky
    loop_close: Option<usize>, //TODO ensure matching, i.e [], not [[]
}

impl Machine {
    //TODO look at using Into<Option<T>>
    //http://xion.io/post/code/rust-optional-args.html for example
    fn new(steps: Option<Steps>) -> Machine {
        let tape: [u8; TAPE_SIZE] = [0; TAPE_SIZE];
        let index: usize = 0;
        Machine {
            tape: tape,
            index: index,
            active: Box::new(tape[index]),
            steps: steps.unwrap_or(Vec::new()),
            loop_open: None,
            loop_close: None,
        }
    }

    fn rebox(&mut self) {
        self.active = Box::new(self.tape[self.index]);
    }
    //execute for Machine runs all steps in order
    //fn execute(&mut self) {

    //}
    //---Operations below---
    fn increment(&mut self) {
        if *self.active < 255 {
            *self.active += 1;
        } else {
            panic!("Cell overflow at {}, could not increment", self.index); //TODO should it wrap?
        }
    }

    fn decrement(&mut self) {
        if *self.active > 0 {
            *self.active -= 1;
        } else {
            panic!("Cell overflow at {}, could not decrement", self.index);
        }
    }

    fn move_up(&mut self) {
        if self.index < TAPE_SIZE - 1 {
            self.index += 1;
            self.rebox();
        } else {
            panic!("no more room on right of tape");
        }
    }

    fn move_down(&mut self) {
        if self.index > 0 {
            self.index -= 1;
            self.rebox();
        } else {
            panic!("no more room on left of tape");
        }
    }

    fn out(&self) -> char {
        use std::ascii::AsciiExt;
        if self.active.is_ascii() {
            *self.active as char
        } else {
            panic!("char at {} not ascii", self.index);
        }
    }

    fn open(&mut self) {
        self.loop_open = Some(self.index);
        if *self.active == 0 {
            self.index = self.loop_close.unwrap() + 1;
            self.rebox();
        } else {
            self.index += 1;
            self.rebox();
        }
    }

    fn close(&mut self) {
        self.loop_close = Some(self.index);
        if *self.active != 0 {
            self.index = self.loop_open.unwrap() + 1;
            self.rebox();
        } else {
            self.index += 1;
            self.rebox();
        }
    }
}

fn parse(input: &str) -> Steps {
    let mut ret = Vec::new();
    for c in input.chars() {
        ret.push(translate(c));
    }
    ret
}

pub fn run(input: &str) -> &str {
    //let steps = parse(input);
    //let machine = Machine::new(steps);
    //machine.execute();
    "Not Yet"
}

pub fn translate(symbol: char) -> Op {
    use Op::*;

    match symbol {
        '+' => Inc,
        '-' => Dec,
        '>' => MoveUp,
        '<' => MoveDown,
        '.' => Out,
        //',' => In, --NOT REQUIRED FOR HELLO_WORLD
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
