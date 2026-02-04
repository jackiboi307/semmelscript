use crate::syntax::*;

#[derive(Debug, Clone)]
pub enum Node {
    Expression(Box<Node>),
    Statement(Statement),
    Block(Block),
    ParenArgs(Box<Node>, Vec<Node>),

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
    If(Box<Node>, Box<Node>, Option<Box<Node>>),
}

#[derive(Debug, Clone)]
pub struct Block {
    statements: Vec<Node>,
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

pub trait Format {
    fn format(&self, indent: usize) -> String;
}

impl Block {
    pub fn new(statements: Vec<Node>) -> Self {
        Self {
            statements,
        }
    }
}

impl Format for Block {
    fn format(&self, indent: usize) -> String {
        const INDENT: &'static str = "    ";

        let indent_str = INDENT.repeat(indent);
        let mut string = format!("{{\n");

        for node in &self.statements {
            match node {
                Node::Block(block) => {
                    string += &(indent_str.clone() + INDENT + &block.format(indent + 1));
                }
                _ => {
                    string += &(indent_str.clone() + INDENT + &node.format(indent));
                }
            }

            if string.chars().last().unwrap() != '}' {
                string.push_str(";");
            }

            string.push_str("\n");
        }

        string += &*format!("{indent_str}}}");
        string
    }
}

impl Format for Node {
    fn format(&self, indent: usize) -> String {
        match self {
            Self::Expression(node) => node.format(indent),
            Self::Statement(statement) => statement.format(indent),
            Self::Block(block) => block.format(indent),
            Self::ParenArgs(root, args) => {
                let args_fmt: Vec<_> = args.iter().map(|node| node.format(indent)).collect();
                format!("{}({})", root.format(indent), args_fmt.join(", "))
            }
            Self::BinaryOp(bop) => format!("({} {} {})",
                bop.a.format(indent),
                bop.op.format(indent),
                bop.b.format(indent),
            ),
            Self::Identifier(name) => format!("{name}"),
            Self::String(string) => format!("\"{string}\""),
            Self::Integer(int) => format!("{int}"),
        }
    }
}

impl Format for Statement {
    fn format(&self, indent: usize) -> String {
        match self {
            Self::SetVariable(ident, value) =>
                format!("let {} = {}", ident, value.format(indent)),
            Self::If(condition, block, ext) => {
                let mut fmt = format!("if {} {}", condition.format(indent), block.format(indent + 1));
                if let Some(ext) = ext {
                    fmt.push_str(&*format!(" else {}", ext.format(indent + 1)));
                }
                fmt
            }
        }
    }
}

impl Format for Operator {
    fn format(&self, _indent: usize) -> String {
        format!("{}", match self {
            Self::Add => OP_ADD,
            Self::Sub => OP_SUB,
            Self::Mul => OP_MUL,
            Self::Div => OP_DIV,
            Self::Pow => OP_POW,
            Self::FieldAccess => ".",
            Self::Paren => "(",
        })
    }
}
