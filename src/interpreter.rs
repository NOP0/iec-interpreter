use crate::parser::{Parser};
pub struct Interpreter {
    parser: Parser,
}

impl Interpreter {
    pub fn new (parser: Parser) -> Interpreter {
        Interpreter {parser}
    }

    pub fn interpret(self) -> i32 {
        let tree = self.parser.parse();
        self.visit(tree)
    }
}