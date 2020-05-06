use crate::lexer::Lexer;
use crate::token::Token;

pub enum Node {
    BinaryOp(BinaryOp),
    Num(Num),
}

pub enum BinaryOpType {
    Add,
    Subtract,
    Divide,
    Multiply,
}

pub struct BinaryOp {
    token: Token,
    pub left: Box<Node>,
    pub right: Box<Node>,
    pub op: BinaryOpType,
}

impl BinaryOp {
    pub fn new(token: Token, left: Node, right: Node, op: BinaryOpType) -> BinaryOp {
        BinaryOp {
            token: token,
            left: Box::new(left),
            right: Box::new(right),
            op,
        }
    }
}

pub struct Num {
    token: Token,
    pub value: i32,
}

impl Num {
    pub fn new(token: Token) -> Num {
        match token {
            Token::Integer(value) => Num { token, value },
            _ => panic!(),
        }
    }
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

    pub fn parse(&mut self) -> Node {
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

    fn factor(&mut self) -> Node {
        let result: Node;
        let token = self.current_token.clone();

        match token {
            Token::Integer(_) => {
                self.eat(Token::Integer(0));
                result = Node::Num(Num::new(token));
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

    fn term(&mut self) -> Node {
        let mut node = self.factor();

        while (self.current_token == Token::Mul) | (self.current_token == Token::Div) {
            let token = self.current_token.clone();
            match self.current_token {
                Token::Mul => {
                    self.eat(Token::Mul);
                    node = Node::BinaryOp(BinaryOp::new(
                        token,
                        node,
                        self.factor(),
                        BinaryOpType::Multiply,
                    ));
                }
                Token::Div => {
                    self.eat(Token::Div);
                    node = Node::BinaryOp(BinaryOp::new(
                        token,
                        node,
                        self.factor(),
                        BinaryOpType::Divide,
                    ));
                }
                _ => panic!(),
            }
        }
        node
    }

    fn expr(&mut self) -> Node {
        let mut node = self.term();

        while (self.current_token == Token::Plus) | (self.current_token == Token::Minus) {
            let token = self.current_token.clone();
            match self.current_token {
                Token::Plus => {
                    self.eat(Token::Plus);
                    node =
                        Node::BinaryOp(BinaryOp::new(token, node, self.term(), BinaryOpType::Add));
                }
                Token::Minus => {
                    self.eat(Token::Minus);
                    node = Node::BinaryOp(BinaryOp::new(
                        token,
                        node,
                        self.factor(),
                        BinaryOpType::Subtract,
                    ));
                }
                _ => panic!(),
            }
        }
        node
    }
}
