#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Integer(i32),
    Plus,
    Minus,
    Mul,
    Div,
    Rparen,
    Lparen,
    Program,
    End_Program,
    Assign,
    Semicolon,
    Id(String),
    NoOp,
}

impl Token {
    pub fn variant_eq(left: Token, right: &Token) -> bool {
        use Token::*;
        match (left, right) {
            (Integer(_), Integer(_)) => true,
            (Plus, Plus) => true,
            (Minus, Minus) => true,
            (Mul, Mul) => true,
            (Div, Div) => true,
            (Rparen, Rparen) => true,
            (Lparen, Lparen) => true,
            (Program, Program) => true,
            (End_Program, End_Program) => true,
            (Assign, Assign) => true,
            (Semicolon, Semicolon) => true,
            (Id(_), Id(_)) => true,
            (_, _) => false,
        }
    }
}
