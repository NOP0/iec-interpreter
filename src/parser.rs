use crate::lexer::Lexer;
use crate::token::Token;

pub enum Node {
    UnaryOp(UnaryOp),
    BinaryOp(BinaryOp),
    Num(Num),
    Assignment(Assignment),
    Variable(Variable),
    Statement(Statement),
    CompoundStatement(CompoundStatement),
    NoOp,
}

pub enum Statement {
    NoOp,
    Assignment(Assignment),
}

pub struct CompoundStatement {
    statements: Vec<Node>,
}

impl CompoundStatement {
    pub fn new() -> CompoundStatement {
        CompoundStatement {
            statements: Vec::new(),
        }
    }
}

pub struct Variable {
    token: Token,
    id: String,
}

impl Variable {
    pub fn new (token: Token) -> Variable {
        match token {
            Token::Id(id) => {
                Variable {
                    token: token.clone(),
                    id: id, 
                }

            }
            _ => panic!("Wrong token in Variable constructor")
        }
    }
}

pub struct Assignment {
    token: Token,
    pub left: Box<Node>,
    pub right: Box<Node>,
    pub op: Token,
}

impl Assignment {
    pub fn new(op: Token, left: Node, right: Node) -> Assignment {
        Assignment {
            token: op.clone(),
            left: Box::new(left),
            right: Box::new(right),
            op: op,
        }
    }
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
                    node = Node::BinaryOp(BinaryOp::new(node, self.factor(), Token::Mul));
                }
                Token::Div => {
                    self.eat(Token::Div);
                    node = Node::BinaryOp(BinaryOp::new(node, self.factor(), Token::Div));
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
                    node = Node::BinaryOp(BinaryOp::new(node, self.term(), Token::Plus));
                }
                Token::Minus => {
                    self.eat(Token::Minus);
                    node = Node::BinaryOp(BinaryOp::new(node, self.factor(), Token::Minus));
                }
                _ => panic!(),
            }
        }
        node
    }
    fn no_op(&mut self) -> Node {
        self.eat(Token::NoOp);
        Node::NoOp
    }

    fn variable(&mut self) -> Node {
        let node = Node::Variable(Variable::new(self.current_token.clone()));
        node
    }
    fn assignment(&mut self) -> Node {
        let left = self.variable();
        let token = self.current_token.clone();
        self.eat(Token::Assign);
        let right = self.expr();
        Node::Assignment(Assignment::new(token, left, right))
    }

    fn statement(&mut self) -> Node {
        let node: Node;
        match self.current_token {
            Token::Program => {
                node = self.compound_statement();
            }
        
                Token::Id(_) => {
                node = self.assignment();
            }
                _ => {
            node = self.no_op();
        }
        }
        node
    }

    fn statement_list(&mut self) -> Vec<Node> {
        let mut list : Vec<Node> = Vec::new();
        list.push(self.statement());

        while self.current_token == Token::Semicolon {
            self.eat(Token::Semicolon);
            list.push(self.statement());
        }
        list
    }

     fn compound_statement(&mut self) -> Node {
        let nodes = self.statement_list();
        let mut compound_statement = CompoundStatement::new();
        for node in nodes {
            compound_statement.statements.push(node);
        }

        let root_node = Node::CompoundStatement(compound_statement);
        root_node
    }
   
    fn program(&mut self) -> Node {
        self.eat(Token::Program);
        let node = self.compound_statement();
        self.eat(Token::End_Program);
        node
    }
}
