use std::fs::read_to_string;

use http_lexer_project::{lex_request, parse_request};

fn main() {
    let path = std::env::args().nth(1).expect("no pattern given");
    let contents = read_to_string(path).expect("should read file");

    let tokens = lex_request(&contents).expect("should lex");
    let request = parse_request(&contents, tokens).expect("should parse");

    println!("{}", request);
}
