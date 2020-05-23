use std::io::{stdin, stdout, Write};
use log::*;

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

#[cfg(test)]
#[test]
fn interpret_addition() {
    let text = "1+2".to_string();
    let lexer = Lexer::new(text);
    let parser = Parser::new(lexer);
    let mut interpreter = Interpreter::new(parser);

    let mut buffer: Vec<u8> = Vec::new();

    interpreter.interpreter_writer(&mut buffer);

    assert_eq!(buffer[0], b'3');
}

#[test]
fn interpret_program() {
    let _ = env_logger::builder().is_test(true).try_init();
    info!("Test");
    let text = 
    "PROGRAM
        1+2
    END_PROGRAM".to_string();


    let lexer = Lexer::new(text);
    let parser = Parser::new(lexer);
    let mut interpreter = Interpreter::new(parser);
    let mut buffer: Vec<u8> = Vec::new();

    interpreter.interpreter_writer(&mut buffer);

    assert_eq!(buffer[0], b'2');

}



#[test]
fn interpret_program_with_assignment() {
    let text = 
    "PROGRAM
        x := 2;
    END_PROGRAM".to_string();


    let lexer = Lexer::new(text);
    let parser = Parser::new(lexer);
    let mut interpreter = Interpreter::new(parser);
    let mut buffer: Vec<u8> = Vec::new();

    interpreter.interpreter_writer(&mut buffer);

    assert_eq!(*interpreter.global_scope.get(&"x".to_string()).unwrap() ,1);

}