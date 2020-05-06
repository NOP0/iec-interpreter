use crate::parser::{BinaryOp, BinaryOpType, Node, Num, Parser};
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

    fn visit_binary_op(&self, binary_op: BinaryOp) -> i32 {
        match binary_op.op {
            BinaryOpType::Add => self.visit(*binary_op.left) + self.visit(*binary_op.right),
            BinaryOpType::Subtract => self.visit(*binary_op.left) - self.visit(*binary_op.right),
            BinaryOpType::Multiply => self.visit(*binary_op.left) * self.visit(*binary_op.right),
            BinaryOpType::Divide => self.visit(*binary_op.left) / self.visit(*binary_op.right),
        }
    }

    fn visit_num(&self, num: Num) -> i32 {
        num.value
    }

    fn visit(&self, node: Node) -> i32 {
        match node {
            Node::BinaryOp(binary_op) => self.visit_binary_op(binary_op),
            Node::Num(num) => self.visit_num(num),
        }
    }
}
