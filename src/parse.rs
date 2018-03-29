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
        ret.push(translate(c));
    }
    ret
}

pub fn translate(symbol: char) -> Op {
    use self::Op::*;

    match symbol {
        '+' => Inc,
        '-' => Dec,
        '>' => MoveUp,
        '<' => MoveDown,
        '.' => Out,
        ',' => In,
        '[' => Open,
        ']' => Close,
        _ => panic!("Unrecognized char: {}", symbol), //TODO comments, not errors
    }
}
