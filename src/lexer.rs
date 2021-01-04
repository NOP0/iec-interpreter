use log::trace;

use crate::token::Token;
use std::collections::HashMap;
pub struct Lexer {
    text: Vec<char>,
    pos: usize,
    current_char: Option<char>,
    reserved_keywords: HashMap<String, Token>,
}

impl Lexer {
    pub fn new(text: String) -> Lexer {
        let mut reserved_keywords: HashMap<String, Token> = HashMap::new();
        reserved_keywords.insert("PROGRAM".to_string(), Token::Program);
        reserved_keywords.insert("END_PROGRAM".to_string(), Token::EndProgram);
        trace!("New Lexer");
        Lexer {
            text: text.chars().collect(),
            pos: 0,
            current_char: text.chars().nth(0),
            reserved_keywords,
        }
    }

    fn id(&mut self) -> Token {
        let mut result = "".to_string();
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                result.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        if self.reserved_keywords.contains_key(&result) {
            trace!("Reserved keyword {}", result);
            self.reserved_keywords.get(&result).unwrap().clone()
        } else {
            trace!("Token::Id({})", result);
            Token::Id(result)
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

    fn peek(&mut self) -> Option<char> {
        if self.pos + 1 > self.text.len() {
            None
        } else {
            Some(self.text[self.pos + 1])
        }
    }

    pub fn get_next_token(&mut self) -> Option<Token> {
        let mut token: Option<Token> = None;
        while let Some(ch) = self.current_char {
            if ch.is_alphabetic() {
                token = Some(self.id());
                break;
            } else if ch == ':' && self.peek() == Some('=') {
                self.advance();
                self.advance();
                trace!("Token::Assign");
                token = Some(Token::Assign);
                break;
            } else if ch == ';' {
                self.advance();
                trace!("Token::Semicolon");
                token = Some(Token::Semicolon);
                break;
            } else if ch.is_whitespace() {
                self.skip_whitespace();
                trace!("Skipping whitespace");
                continue;
            } else if ch.is_digit(10) {
                let integer = self.integer();
                trace!("Token::Integer({})", integer);
                token = Some(Token::Integer(integer));
                break;
            } else if ch == '+' {
                self.advance();
                trace!("Token::Plus");
                token = Some(Token::Plus);
                break;
            } else if ch == '-' {
                self.advance();
                trace!("Token::Minus");
                token = Some(Token::Minus);
                break;
            } else if ch == '*' {
                self.advance();
                trace!("Token::Mul");
                token = Some(Token::Mul);
                break;
            } else if ch == '/' {
                self.advance();
                trace!("Token::Div");
                token = Some(Token::Div);
                break;
            } else if ch == '(' {
                self.advance();
                trace!("Token::Lparen");
                token = Some(Token::Lparen);
                break;
            } else if ch == ')' {
                self.advance();
                trace!("Token::Rparen");
                token = Some(Token::Rparen);
                break;
            } else {
                panic!("Unexpected char in Lexer: {}", ch);
            }
        }
        token
    }
}
