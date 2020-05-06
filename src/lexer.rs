use crate::token::Token;
pub struct Lexer {
    text: Vec<char>,
    pos: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(text: String) -> Lexer {
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

    pub fn get_next_token(&mut self) -> Option<Token> {
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
