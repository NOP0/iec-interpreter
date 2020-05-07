use crate::lexer::Lexer;
use crate::token::Token;

pub enum Node {
    UnaryOp(UnaryOp),
    BinaryOp(BinaryOp),
    Num(Num),
}

pub struct UnaryOp {
    token: Token,
    pub expr: Box<Node>,
    pub op: Token,
}

impl UnaryOp {
    pub fn new(op: Token, expr: Node) -> UnaryOp {
        UnaryOp {
            token: op.clone(),
            expr: Box::new(expr),
            op: op,
        }
    }
}
pub struct BinaryOp {
    token: Token,
    pub left: Box<Node>,
    pub right: Box<Node>,
    pub op: Token,
}

impl BinaryOp {
    pub fn new(left: Node, right: Node, op: Token) -> BinaryOp {
        BinaryOp {
            token: op.clone(),
            left: Box::new(left),
            right: Box::new(right),
            op: op,
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
        let node: Node;

        match self.current_token {
            Token::Plus => {
                self.eat(Token::Plus);
                node = Node::UnaryOp(UnaryOp::new(Token::Plus, self.factor()));
            }
            Token::Minus => {
                self.eat(Token::Minus);
                node = Node::UnaryOp(UnaryOp::new(Token::Minus, self.factor()));

            }
            Token::Integer(value) => {
                self.eat(Token::Integer(0));
                node = Node::Num(Num::new(Token::Integer(value)));
            }
            Token::Lparen => {
                self.eat(Token::Lparen);
                node = self.expr();
                self.eat(Token::Rparen);
            }
            _ => panic!("Unexpected token in factor"),
        }
        node
    }

    fn term(&mut self) -> Node {
        let mut node = self.factor();

        while (self.current_token == Token::Mul) | (self.current_token == Token::Div) {
            match self.current_token {
                Token::Mul => {
                    self.eat(Token::Mul);
                    node = Node::BinaryOp(BinaryOp::new(
                        node,
                        self.factor(),
                        Token::Mul,
                    ));
                }
                Token::Div => {
                    self.eat(Token::Div);
                    node = Node::BinaryOp(BinaryOp::new(
                        node,
                        self.factor(),
                        Token::Div,
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
            match self.current_token {
                Token::Plus => {
                    self.eat(Token::Plus);
                    node =
                        Node::BinaryOp(BinaryOp::new(node, self.term(), Token::Plus));
                }
                Token::Minus => {
                    self.eat(Token::Minus);
                    node = Node::BinaryOp(BinaryOp::new(
                        node,
                        self.factor(),
                        Token::Minus,
                    ));
                }
                _ => panic!(),
            }
        }
        node
    }
}
