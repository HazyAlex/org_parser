use std::env;
use std::fs;

use org_parser::Parser;

fn main() {
    let filename = env::args().nth(1).expect("Expected a valid filename!");
    let data = fs::read_to_string(filename).expect("Expected a valid orgmode file!");

    let parser = Parser::parse(data);

    println!("{:#?}", parser.headers);
    println!("{:#?}", parser.options);
}
