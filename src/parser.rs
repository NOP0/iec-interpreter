use crate::token::{Token};
use crate::lexer::{Lexer};


pub enum BinaryOp {
    Add,
    Subract,
    Divide,
    Multiply,
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Parser {
        let current_token = lexer.get_next_token().expect("No first token");
        Parser {
            lexer,
            current_token,
        }
    }

    pub fn parse(self) -> i32 {
        self.expr()
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