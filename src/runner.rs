use std::fs;
use crate::lexer;

pub fn run(content: String) {
    let mut l1 = lexer::lexer::Lexer::new(content);
    l1.lex();
    println!("{:?}", l1.tokens)
}

pub fn run_file(file_name: &str) {
    let file_content = fs::read_to_string(file_name).expect("Error while reading the file");
    run(file_content);
}

pub fn run_prompt() {
    todo!();
}
