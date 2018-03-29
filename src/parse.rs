pub type Prog = Vec<Op>;

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

pub fn parse(input: &str) -> Prog {
    let mut ret = Vec::new();
    for c in input.chars() {
        if let Some(op) = translate(c) {
            ret.push(op);
        }
    }
    ret
}

pub fn translate(symbol: char) -> Option<Op> {
    use self::Op::*;

    match symbol {
        '+' => Some(Inc),
        '-' => Some(Dec),
        '>' => Some(MoveUp),
        '<' => Some(MoveDown),
        '.' => Some(Out),
        ',' => Some(In),
        '[' => Some(Open),
        ']' => Some(Close),
        _ => None,
    }
}
