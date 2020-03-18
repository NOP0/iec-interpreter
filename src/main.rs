#[derive(PartialEq)]
enum Token{
    Integer(i32),
    Plus(String),
    EOF,
}


struct Interpreter {
    text : String,
    pos : usize,
    current_token: Token,
}

impl Interpreter {
    fn error(self) {
        panic!("Could not parse input");
    }

    fn get_next_token(&mut self) -> Result<Token, ()> {
        let token : Token;
        if self.pos > self.text.len() - 1 {
            token = Token::EOF;
            return Ok(token)
        }

        let current_char : char = self.text.as_bytes()[self.pos] as char;
            
        if current_char.is_ascii_digit() {
            token = Token::Integer(current_char as i32);
            self.pos += 1;
        }
        else if current_char == '+' {
            token = Token::Plus("+".to_string());
            self.pos += 1;
        }
        else {
            return Err(())
        }

        Ok(token)
        
    }

    fn eat(&mut self, token: Token) {
        if self.current_token == token {
            self.current_token = self.get_next_token().expect("Incorrect token");
        }
    }
}

fn main() {
    println!("Hello, world!");
}
