use std::fs;
use crate::{ast::parser, lexer};

pub fn run(content: String) {
    let mut l1 = lexer::lexer::Lexer::new(content);
    l1.lex();
    let mut toks = l1.tokens.clone();
    let mut p1 = parser::Parser::new(toks);
    println!("{:?}", l1.tokens);
    let mut e1 = p1.expr();
    println!("{:?}", e1);
}

pub fn run_file(file_name: &str) {
    let file_content = fs::read_to_string(file_name).expect("Error while reading the file");
    run(file_content);
}

pub fn run_prompt() {
    todo!();
}
