use crate::parser::{BinaryOp, Node, Num, Parser, UnaryOp};
use crate::token::Token;
pub struct Interpreter {
    parser: Parser,
}

impl Interpreter {
    pub fn new(parser: Parser) -> Interpreter {
        Interpreter { parser }
    }

    pub fn interpret(&mut self) -> i32 {
        let tree = self.parser.parse();
        let result = self.visit(tree);
        result
    }

    fn visit_unary_op(&self, unary_op: UnaryOp) -> i32 {
        match unary_op.op {
            Token::Plus => self.visit(*unary_op.expr),
            Token::Minus => -self.visit(*unary_op.expr),
            _ => panic!("Incorrect token in visit_unary_op"),
        }
    }
    fn visit_binary_op(&self, binary_op: BinaryOp) -> i32 {
        match binary_op.op {
            Token::Plus => self.visit(*binary_op.left) + self.visit(*binary_op.right),
            Token::Minus => self.visit(*binary_op.left) - self.visit(*binary_op.right),
            Token::Mul => self.visit(*binary_op.left) * self.visit(*binary_op.right),
            Token::Div => self.visit(*binary_op.left) / self.visit(*binary_op.right),
            _ => panic!("Incorrect token in visit_binary_op"),
        }
    }

    fn visit_num(&self, num: Num) -> i32 {
        num.value
    }

    fn visit(&self, node: Node) -> i32 {
        match node {
            Node::UnaryOp(unary_op) => self.visit_unary_op(unary_op),
            Node::BinaryOp(binary_op) => self.visit_binary_op(binary_op),
            Node::Num(num) => self.visit_num(num),
        }
    }
}
