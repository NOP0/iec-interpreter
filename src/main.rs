use std::io::{Write, stdin, stdout};

#[derive(PartialEq, Clone, Copy)]
enum Token{
    Integer(i32),
    Plus,
    EOF,
}

impl Token{
    fn variant_eq(left: Token, right: Token) -> bool {
        use Token::*;
        match(left, right) {
            (Integer(_), Integer(_)) => true,
            (Plus, Plus) => true,
            (EOF, EOF) => true,
            (_,_) => false,
        }
    }
}


struct Interpreter {
    text : String,
    pos : usize,
    current_token: Token,
}

impl Interpreter {

    fn new(text: String) -> Interpreter {
        Interpreter {
            text,
            pos : 0,
            current_token : Token::EOF, 
        }
    }

    fn reload(&mut self, text: String) {
        self.text = text;
        self.pos = 0;
        self.current_token = Token::EOF; 
    }

    fn get_next_token(&mut self) -> Result<Token, ()> {
        let token : Token;
        if self.pos > self.text.len() - 1 {
            token = Token::EOF;
            return Ok(token)
        }

        let current_char : char = self.text.as_bytes()[self.pos] as char;
            
        if current_char.is_ascii_digit() {
            token = Token::Integer(current_char.to_digit(10).unwrap() as i32);
            self.pos += 1;
        }
        else if current_char == '+' {
            token = Token::Plus;
            self.pos += 1;
        }
        else {
            return Err(())
        }

        Ok(token)
        
    }

    fn eat(&mut self, token: Token) {
        if Token::variant_eq(token, self.current_token) {
            self.current_token = self.get_next_token().expect("Incorrect token");
        }
        else {
            panic!("Wrong token")
        }
    }

    fn expr(&mut self) -> i32 {

        self.current_token = self.get_next_token().unwrap(); // Get first token

        let left = self.current_token; // Save left operand
        self.eat(Token::Integer(0)); // Should be integer

        let op = self.current_token; // Save operator
        self.eat(Token::Plus); // Should be PLUS

        let right = self.current_token; // Save right operand
        self.eat(Token::Integer(0)); // Should be integer 

        
        if let (Token::Integer(left_value), Token::Integer(right_value)) = (left, right){
            left_value + right_value
        }  
        else
        {
            panic!()
        }

    }
}

fn main() -> std::io::Result<()>{
    let mut text : String = "".to_string();
    let mut interpreter = Interpreter::new(text.clone());
    loop{
        print!(">>");
        stdout().flush()?;
        
        text.clear();
        stdin().read_line(&mut text)?;
        text = text.trim().to_string();
        interpreter.reload(text.clone());
        let result = interpreter.expr();
        println!("{}", result);
        
    }
}
