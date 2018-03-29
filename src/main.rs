extern crate bfrs;

use bfrs::run;
use std::{
    env,
    fs::File,
    io::{
        BufReader,
        Read,
        Result
    },
    path::Path
};

//TODO BfString type
fn get_bf(file_path: &str) -> Result<String> {
    let file = File::open(Path::new(file_path))?;

    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let file_path = if let Some(arg1) = env::args().nth(1) {
        arg1
    } else {
        panic!("Please specify a file");
    };

    let debug = if let Some(arg2) = env::args().nth(2) {
        arg2 == "-d" || arg2 == "--debug"
    } else {
        false
    };

    let prog = get_bf(&file_path).expect("Could not parse string");
    let result = run(&prog, debug);
    println!("{}", result);
}
