use std::env;

mod runner;
mod lexer;
mod general_types;
mod ast;
mod interpreter;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: mark [script]")
    } else if args.len() == 2 {
        runner::run_file(&args[1]);
    } else {
        runner::run_prompt();
    }
}
