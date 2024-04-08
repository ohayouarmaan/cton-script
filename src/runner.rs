use std::fs;
use crate::{ast::{parser, printer::ASTPrinter}, interpreter::interpreter::Interpreter, lexer};

pub fn run(content: String) {
    let mut l1 = lexer::lexer::Lexer::new(content);
    l1.lex();
    let toks = l1.tokens.clone();
    println!("{:?}", toks);
    let mut p1 = parser::Parser::new(toks);
    let mut printer = ASTPrinter::new();
    printer.print(p1.expr());
    p1.reset();
    let mut i1 = Interpreter::new(vec![p1.expr()]);
    i1.evaluate();
}

pub fn run_file(file_name: &str) {
    let file_content = fs::read_to_string(file_name).expect("Error while reading the file");
    run(file_content);
}

pub fn run_prompt() {
    todo!();
}
