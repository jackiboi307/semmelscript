use super::tokens::*;

#[derive(Debug, Clone)]
pub enum Node {
    Statement(Statement),
    Block(Block),
    ParenArgs(Box<Node>, Vec<Node>),

    // Operators
    BinaryOp(Box<BinaryOp>),

    // Literals
    Identifier(String),
    String(String),
    Integer(i32),
    Boolean(bool),
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
    pub statements: Vec<Node>,
}

impl Block {
    pub fn new(statements: Vec<Node>) -> Self {
        Self {
            statements,
        }
    }
}

pub trait Format {
    fn format(&self, indent: usize) -> String;
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
            Self::Boolean(boolean) => format!("{boolean}"),
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
                    let indent = match **ext {
                        Node::Statement(..) => indent, // elif
                        Node::Block(..) => indent + 1, // else
                        _ => unreachable!()
                    };
                    fmt.push_str(&*format!(" else {}", ext.format(indent)));
                }
                fmt
            }
        }
    }
}
