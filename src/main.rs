use std::env;
use std::fs;
use std::process::exit;

use org_parser::Parser;
use serde_json;

fn main() {
    let input = env::args().nth(1).expect("Expected a valid filename!");
    let output = env::args().nth(2).expect("Expected an output filename!");
    let data = fs::read_to_string(input).expect("Expected a valid orgmode file!");

    let parser = Parser::parse(data);

    let contents = match serde_json::to_string_pretty(&parser) {
        Err(error) => {
            eprintln!("{}", error);
            exit(1);
        }
        Ok(result) => result,
    };

    match fs::write(output, contents) {
        Err(error) => {
            eprintln!("{}", error);
            exit(2);
        }
        Ok(_) => (),
    }
}
