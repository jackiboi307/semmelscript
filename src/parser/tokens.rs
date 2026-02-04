use super::Format;
use super::syntax::*;

#[derive(Debug)]
pub enum Keyword {
    Let,
    If,
    Else,
    Elif,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    // math
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,

    // compare
    Equal,
    Inequal,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // logic
    And,
    Or,

    FieldAccess,
    Paren,
}

impl Format for Operator {
    fn format(&self, _indent: usize) -> String {
        format!("{}", match self {
            Self::Add => OP_ADD,
            Self::Sub => OP_SUB,
            Self::Mul => OP_MUL,
            Self::Div => OP_DIV,
            Self::Pow => OP_POW,
            Self::Mod => OP_MOD,
            Self::Equal => OP_EQUAL,
            Self::Inequal => OP_INEQUAL,
            Self::Less => OP_LESS,
            Self::LessEqual => OP_LESSEQUAL,
            Self::Greater => OP_GREATER,
            Self::GreaterEqual => OP_GREATEREQUAL,
            Self::And => OP_AND,
            Self::Or => OP_OR,
            Self::FieldAccess => ".",
            Self::Paren => "(",
        })
    }
}
