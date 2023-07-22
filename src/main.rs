use std::fs;

pub mod class_parser;
pub use class_parser::*;

pub mod values;
pub use values::*;

fn main() {
    let file = fs::read("./tests/Main.class").unwrap();

    let mut parser = ClassFileParser::new(file);
    let class = match parser.parse() {
        Ok(value) => value,
        Err(error) => return eprintln!("Failed to parse class file: {:?}", error),
    };

    println!("Parsed class file: {:?}", class);
}
