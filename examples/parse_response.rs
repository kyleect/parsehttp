use std::fs::read_to_string;

use parsehttp::{lex_response, parse_response};

fn main() {
    let path = std::env::args().nth(1).expect("no pattern given");
    let contents = read_to_string(path).expect("should read file");

    let tokens = lex_response(&contents).expect("should lex");
    let response = parse_response(&contents, tokens).expect("should parse");

    println!("{}", response);
}
