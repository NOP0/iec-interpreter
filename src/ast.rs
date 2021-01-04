use crate::token::Token;
#[derive(Debug, PartialEq)]
pub enum Node {
    UnaryOp(UnaryOp),
    BinaryOp(BinaryOp),
    Num(Num),
    Assignment(Assignment),
    Variable(Variable),
    CompoundStatement(CompoundStatement),
    NoOp,
}

#[derive(Debug, PartialEq)]
pub struct CompoundStatement {
    pub statements: Vec<Node>,
}

impl CompoundStatement {
    pub fn new() -> CompoundStatement {
        CompoundStatement {
            statements: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Variable {
    token: Token,
    pub id: String,
}

impl Variable {
    pub fn new(token: Token) -> Variable {
        match token.clone() {
            Token::Id(id) => Variable {
                token: token,
                id: id,
            },
            _ => panic!("Wrong token in Variable constructor: {:?}", token),
        }
    }
}

#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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
