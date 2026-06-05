#[cfg(feature = "json")]
use serde_json::to_string_pretty;

fn main() {
    #[cfg(feature = "json")]
    {
        use parsehttp::{lex_response, parse_response};
        use std::fs::read_to_string;

        let path = std::env::args().nth(1).expect("no pattern given");
        let contents = read_to_string(path).expect("should read file");

        let tokens = lex_response(&contents).expect("should lex");
        let request = parse_response(&contents, tokens).expect("should parse");

        println!("{}", to_string_pretty(&request).unwrap());
    }

    #[cfg(not(feature = "json"))]
    {
        eprintln!("json feature is not enabled");
    }
}
