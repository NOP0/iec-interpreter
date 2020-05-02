use std::io::{stdin, stdout, Write};

#[derive(PartialEq, Clone, Debug)]
enum Token {
    Integer(i32),
    Plus,
    Minus,
    Mul,
    Div,
    Rparen,
    Lparen,
}

impl Token {
    fn variant_eq(left: Token, right: &Token) -> bool {
        use Token::*;
        match (left, right) {
            (Integer(_), Integer(_)) => true,
            (Plus, Plus) => true,
            (Minus, Minus) => true,
            (Mul, Mul) => true,
            (Div, Div) => true,
            (Rparen, Rparen) => true,
            (Lparen, Lparen) => true,
            (_, _) => false,
        }
    }
}

struct Lexer {
    text: Vec<char>,
    pos: usize,
    current_char: Option<char>,
}

impl Lexer {
    fn new(text: String) -> Lexer {
        Lexer {
            text: text.chars().collect(),
            pos: 0,
            current_char: text.chars().nth(0),
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
        if self.pos > self.text.len() - 1 {
            self.current_char = None;
        } else {
            self.current_char = Some(self.text[self.pos]);
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance()
            } else {
                break;
            }
        }
    }

    fn integer(&mut self) -> i32 {
        let mut result = "".to_string();
        while let Some(ch) = self.current_char {
            if ch.is_digit(10) {
                result.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        result.parse().unwrap()
    }

    fn get_next_token(&mut self) -> Option<Token> {
        let mut token: Option<Token> = None;
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.skip_whitespace();
                continue;
            } else if ch.is_digit(10) {
                token = Some(Token::Integer(self.integer()));
                break;
            } else if ch == '+' {
                self.advance();
                token = Some(Token::Plus);
                break;
            } else if ch == '-' {
                self.advance();
                token = Some(Token::Minus);
                break;
            } else if ch == '*' {
                self.advance();
                token = Some(Token::Mul);
                break;
            } else if ch == '/' {
                self.advance();
                token = Some(Token::Div);
                break;
            } else if ch == '(' {
                self.advance();
                token = Some(Token::Lparen);
                break;
            } else if ch == ')' {
                self.advance();
                token = Some(Token::Rparen);
                break;
            } else {
                panic!();
            }
        }
        token
    }
}

struct Interpreter {
    lexer: Lexer,
    current_token: Token,
}

impl Interpreter {
    fn new(mut lexer: Lexer) -> Interpreter {
        let current_token = lexer.get_next_token().expect("No first token");
        Interpreter {
            lexer,
            current_token,
        }
    }

    fn eat(&mut self, token: Token) {
        if Token::variant_eq(token.clone(), &self.current_token) {
            if let Some(token) = self.lexer.get_next_token() {
                self.current_token = token;
            }
        } else {
            panic!(
                "Wrong token, got {:?}, expected {:?}",
                self.current_token, token
            );
        }
    }

    fn factor(&mut self) -> i32 {
        let result: i32;
        let token = self.current_token.clone();

        match token {
            Token::Integer(value) => {
                self.eat(Token::Integer(0));
                result = value;
            }
            Token::Lparen => {
                self.eat(Token::Lparen);
                result = self.expr();
                self.eat(Token::Rparen);
            }
            _ => panic!("Unexpected token in factor"),
        }
        result
    }

    fn term(&mut self) -> i32 {
        let mut result = self.factor();

        while (self.current_token == Token::Mul) | (self.current_token == Token::Div) {
            match self.current_token {
                Token::Mul => {
                    self.eat(Token::Mul);
                    result = result * self.factor();
                }
                Token::Div => {
                    self.eat(Token::Div);
                    result = result / self.factor();
                }
                _ => panic!(),
            }
        }
        result
    }

    fn expr(&mut self) -> i32 {
        let mut result;

        result = self.term();

        while (self.current_token == Token::Plus) | (self.current_token == Token::Minus) {
            match self.current_token {
                Token::Plus => {
                    self.eat(Token::Plus);
                    result = result + self.term();
                }
                Token::Minus => {
                    self.eat(Token::Minus);
                    result = result - self.term();
                }
                _ => panic!(),
            }
        }
        result
    }
}

fn main() -> std::io::Result<()> {
    loop {
        let mut text: String = "".to_string();
        print!(">>");
        stdout().flush()?;
        text.clear();
        stdin().read_line(&mut text)?;
        text = text.trim().to_string();
        let lexer = Lexer::new(text.clone());
        let mut interpreter = Interpreter::new(lexer);
        let result = interpreter.expr();
        println!("{}", result);
    }
}
