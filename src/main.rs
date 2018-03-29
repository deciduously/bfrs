extern crate bfrs;

use bfrs::run::run;
use std::{env, fs::File, io::{self, BufReader, Read}, path::Path, process::exit};

//TODO BfString type
fn get_bf(file_path: &str) -> io::Result<String> {
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
        println!("Usage: bfrsc file.bf <-d||--debug>");
        exit(1);
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
