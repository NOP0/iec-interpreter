use std::io::{stdin, stdout, Write};

mod ast;
mod interpreter;
mod lexer;
mod parser;
mod token;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

fn main() -> std::io::Result<()> {
    loop {
        let mut text: String = "".to_string();
        print!(">>");
        stdout().flush()?;
        text.clear();
        stdin().read_line(&mut text)?;
        text = text.trim().to_string();
        let lexer = Lexer::new(text.clone());
        let parser = Parser::new(lexer);
        let mut interpreter = Interpreter::new(parser);
        interpreter.interpret();
    }
}


#[test]
fn interpret_addition() {
    let text = "1+2".to_string();
    let lexer = Lexer::new(text);
    let parser = Parser::new(lexer);
    let mut interpreter = Interpreter::new(parser);
    interpreter.interpret();
}