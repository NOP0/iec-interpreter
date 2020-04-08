use std::io::{Write, stdin, stdout};

#[derive(PartialEq, Clone, Copy)]
enum Token{
    Integer(i32),
    Plus,
    Minus,
    EOF,
}

impl Token{
    fn variant_eq(left: Token, right: Token) -> bool {
        use Token::*;
        match(left, right) {
            (Integer(_), Integer(_)) => true,
            (Plus, Plus) => true,
            (Minus, Minus) => true,
            (EOF, EOF) => true,
            (_,_) => false,
        }
    }
}


struct Interpreter {
    text : Vec<char>,
    pos : usize,
    current_char : Option<char>,
    current_token: Token,
}

impl Interpreter {

    fn new(text: String) -> Interpreter {

        Interpreter {
            text: text.chars().collect(),
            pos : 0,
            current_char : text.chars().nth(0),
            current_token : Token::EOF, 
        }
    }

    fn reload(&mut self, text: String) {
        self.text = text.chars().collect();
        self.pos = 0;
        self.current_char = Some(self.text[self.pos]);
        self.current_token = Token::EOF; 
    }

    fn advance(&mut self) {
        self.pos += 1;
        if self.pos > self.text.len() - 1 {
            self.current_char = None;
        }
        else {
            self.current_char = Some(self.text[self.pos]);
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance()
            }
            else {
                break;
            }
        }
    }

    fn integer(&mut self) -> i32 {
        let mut result :i32 = 0;
        while let Some(ch) = self.current_char {
            if ch.is_digit(10) {
                result += ch.to_digit(10).unwrap() as i32;
                self.advance();
            }
            else {break;}
        }
        result
    }

    fn get_next_token(&mut self) -> Option<Token> {

        let mut token : Option<Token> = None;
      //  println!("{}", self.current_char.unwrap());
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.skip_whitespace();
                continue;
            }
            else if ch.is_digit(10) {
                token = Some(Token::Integer(self.integer()));
                break;
            }
            else if ch =='+' {
                self.advance();
                token = Some(Token::Plus);
                break;
            }
            else if ch == '-' {
                self.advance();
                token = Some(Token::Minus);
                break;
            }
            else {
                panic!();
            }

        }
        token
    }

            


    fn eat(&mut self, token: Token) {
        if Token::variant_eq(token, self.current_token) {
            if let Some(token) = self.get_next_token() {
                self.current_token = token;
            }
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
