use log::trace;
use std::collections::HashMap;

use crate::ast::{Assignment, BinaryOp, CompoundStatement, Node, Num, UnaryOp, Variable};

use crate::parser::Parser;
use crate::token::Token;

pub fn walk_unary_op<V: Visitor + ?Sized>(visitor: &mut V, unary_op: &UnaryOp) {
    visitor.visit(&unary_op.expr);
}

pub fn walk_binary_op<V: Visitor + ?Sized>(visitor: &mut V, binary_op: &BinaryOp) {
    visitor.visit(&binary_op.left);
    visitor.visit(&binary_op.right);
}

pub fn walk_assignment<V: Visitor + ?Sized>(visitor: &mut V, assignment: &Assignment) {
    visitor.visit(&assignment.left);
    visitor.visit(&assignment.right);
}

pub trait Visitor {
    fn visit(&mut self, node: &Node) {
        match node {
            Node::UnaryOp(unary_op) => self.visit_unary_op(&unary_op),
            Node::BinaryOp(binary_op) => self.visit_binary_op(&binary_op),
            Node::Num(num) => self.visit_num(&num),
            Node::Assignment(assignment) => self.visit_assignment(&assignment),
            Node::Variable(variable) => self.visit_variable(&variable),
            Node::CompoundStatement(compound_statement) => {
                self.visit_compound_statement(&compound_statement)
            }
            Node::NoOp => {}
        }
    }

    fn visit_unary_op(&mut self, unary_op: &UnaryOp) {
        walk_unary_op(self, unary_op);
    }

    fn visit_binary_op(&mut self, binary_op: &BinaryOp) {
        walk_binary_op(self, binary_op);
    }

    fn visit_assignment(&mut self, assignment: &Assignment) {
        walk_assignment(self, assignment);
    }

    #[allow(unused_variables)]
    fn visit_num(&mut self, num: &Num) {}

    #[allow(unused_variables)]
    fn visit_variable(&mut self, variable: &Variable) {}

    fn visit_compound_statement(&mut self, compound_statement: &CompoundStatement) {
        trace!("Visiting compound statement");
        for node in &compound_statement.statements {
            match node {
                Node::Assignment(assignment) => {
                    self.visit_assignment(&assignment);
                }
                Node::NoOp => trace!("Visited NoOp!"),
                _ => {
                    panic!("No valid node found in statement list {:?}", node);
                }
            }
        }
    }
}

enum Object {
    Integer(i32),
}
pub struct Interpreter {
    parser: Parser,
    object: i32,
    pub global_scope: HashMap<String, i32>,
}

impl Interpreter {
    pub fn new(parser: Parser) -> Interpreter {
        Interpreter {
            parser,
            object: 0,
            global_scope: HashMap::new(),
        }
    }

    pub fn interpret(&mut self) {
        self.interpreter_writer(&mut std::io::stdout());
    }

    pub fn interpreter_writer(&mut self, mut writer: &mut impl std::io::Write) {
        trace! {"Start interpreting"}
        let tree = self.parser.parse();
        trace!("Start visiting");
        self.visit(&tree);
        trace!("End visiting");

        match writeln!(&mut writer, "{}", self.object) {
            Ok(_) => {}
            Err(error) => panic!("Error in writeln! interpreter_writer: {}", error),
        }
    }
}

impl Visitor for Interpreter {
    fn visit_unary_op(&mut self, unary_op: &UnaryOp) {
        trace!("Visiting unary op");
        self.visit(&unary_op.expr);
        match unary_op.op {
            Token::Plus => {}
            Token::Minus => self.object *= -1,
            _ => panic!("Incorrect token in visit_unary_op"),
        }
    }
    fn visit_binary_op(&mut self, binary_op: &BinaryOp) {
        trace!("Visiting binary op");
        let lhs: i32;
        let rhs: i32;
        self.visit(&binary_op.left);
        lhs = self.object;
        self.visit(&binary_op.right);
        rhs = self.object;

        match binary_op.op {
            Token::Plus => self.object = lhs + rhs,
            Token::Minus => self.object = lhs - rhs,
            Token::Mul => self.object = lhs * rhs,
            Token::Div => self.object = lhs / rhs,
            _ => panic!("Incorrect token in visit_binary_op"),
        }
    }

    fn visit_num(&mut self, num: &Num) {
        trace!("Visiting num");
        self.object = num.value;
    }

    fn visit_assignment(&mut self, assignment: &Assignment) {
        trace!("Visiting assignment");
        self.visit(&assignment.right);
        match &*assignment.left {
            Node::Variable(variable) => {
                trace!("Variable {:?}, inserted in global scope", variable);
                self.global_scope.insert(variable.id.clone(), self.object);
            }

            _ => panic!("Incorrect node in visit_assignment"),
        }
    }

    fn visit_variable(&mut self, variable: &Variable) {
        trace!("Visiting variable");
        if let Some(value) = self.global_scope.get(&variable.id) {
            self.object = *value;
        } else {
            panic!("Variable id not in scope");
        }
    }
}
