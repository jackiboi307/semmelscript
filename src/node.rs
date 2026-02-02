use crate::syntax::*;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Node {
    Expression(Box<Node>),
    Statement(Statement),

    // Operators
    BinaryOp(Box<BinaryOp>),

    // Literals
    Identifier(String),
    String(String),
    Integer(u128),
}

#[derive(Debug, Clone)]
pub struct BinaryOp {
    pub op: Operator,
    pub a: Node,
    pub b: Node,
}

#[derive(Debug, Clone)]
pub enum Statement {
    SetVariable(String, Box<Node>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    // math
    // TODO create separate enum?
    Add,
    Sub,
    Mul,
    Div,
    Pow,

    FieldAccess,
    Paren,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expression(node) => Display::fmt(node, f),
            Self::Statement(statement) => statement.fmt(f),
            Self::BinaryOp(bop) => write!(f, "({} {} {})", bop.a, bop.op, bop.b),
            Self::Identifier(name) => write!(f, "{name}"),
            Self::String(string) => write!(f, "\"{string}\""),
            Self::Integer(int) => write!(f, "{int}"),
        }
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SetVariable(ident, value) => write!(f, "let {} = {}", ident, value),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Add => OP_ADD,
            Self::Sub => OP_SUB,
            Self::Mul => OP_MUL,
            Self::Div => OP_DIV,
            Self::Pow => OP_POW,
            Self::FieldAccess => OP_FIELD_ACCESS,
            Self::Paren => OP_PAREN,
        })
    }
}
