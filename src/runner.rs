use std::fs;
use crate::{ast::{parser, printer::ASTPrinter}, lexer};

pub fn run(content: String) {
    let mut l1 = lexer::lexer::Lexer::new(content);
    l1.lex();
    let mut toks = l1.tokens.clone();
    let mut p1 = parser::Parser::new(toks);
    let mut printer = ASTPrinter::new();
    printer.print(p1.expr())
}

pub fn run_file(file_name: &str) {
    let file_content = fs::read_to_string(file_name).expect("Error while reading the file");
    run(file_content);
}

pub fn run_prompt() {
    todo!();
}
