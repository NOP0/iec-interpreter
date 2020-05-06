use std::io::{stdin, stdout, Write};

mod lexer;
mod token;
mod parser;
mod interpreter;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;







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
        let result = interpreter.interpret();
        println!("{}", result);
    }
}
