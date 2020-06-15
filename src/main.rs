use std::env;
use std::fs;
use std::process::exit;

use org_parser::Parser;

fn main() {
    let input = env::args().nth(1).expect("Expected a valid filename!");
    let output = env::args().nth(2).expect("Expected an output filename!");
    let data = fs::read_to_string(input).expect("Expected a valid orgmode file!");

    let parser = Parser::parse(&data);

    match parser.print_json_pretty(&output) {
        Ok(_) => (),
        Err(error) => {
            eprintln!("{}", error);
            exit(1);
        }
    }
}
