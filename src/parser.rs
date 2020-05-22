use crate::ast::{
    Assignment, BinaryOp, CompoundStatement, Node, Num, Statement, UnaryOp, Variable,
};
use crate::lexer::Lexer;
use crate::token::Token;

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
            Token::Id(_) => {
                node = self.variable();
                self.eat(Token::Id("".to_string()));
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
        let mut list: Vec<Node> = Vec::new();
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
        self.eat(Token::EndProgram);
        node
    }
}


#[test] 
fn parse_program() {

}

#[test]
fn parse_addition() {
    let text = "1+2".to_string();
    let lexer = Lexer::new(text);
    let mut parser = Parser::new(lexer);
    if let Node::BinaryOp(binary_op) = parser.parse() {
        assert_eq!(*binary_op.left, Node::Num(Num::new(Token::Integer(1))));
        assert_eq!(*binary_op.right, Node::Num(Num::new(Token::Integer(2))));
        assert_eq!(binary_op.op, Token::Plus);
    }    
}
    
