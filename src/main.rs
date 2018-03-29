extern crate bfrs;

use bfrs::run;
use std::env;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use std::path::Path;

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
        panic!("Please specify a file");
    };

    let prog = get_bf(&file_path).expect("Could not parse string");
    let result = run(&prog);
    println!("{}", result);
}
