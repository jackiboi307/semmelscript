use crate::*;
use crate::parser::node::*;

quick_error! {
    #[derive(Debug)]
    enum RuntimeError {
        ExpectedType(typ: Type) {}
    }
}

use RuntimeError::*;

#[derive(Debug)]
enum Type {
    String,
    Integer,
    Boolean,
}

pub struct Runtime {

}

#[derive(Debug)]
pub enum Value {
    Void,
    String(String),
    Integer(i32),
    Boolean(bool),
}

pub trait Evaluate {
    // evaluates the value of a node
    fn eval(&self, _runtime: &mut Runtime) -> Result<Value> {
        // TODO remove
        unimplemented!()
    }
}

impl Evaluate for Node {
    fn eval(&self, runtime: &mut Runtime) -> Result<Value> {
        match self {
            Self::ParenArgs(root, args) => todo!(),

            // TODO find out if you can write this in a better way
            Self::Statement(node) => node.eval(runtime), 
            Self::Block(node) => node.eval(runtime), 
            Self::BinaryOp(node) => node.eval(runtime), 

            Self::Identifier(ident) => todo!(),
            Self::String(string) => Ok(Value::String(string.clone())),
            Self::Integer(integer) => Ok(Value::Integer(*integer)),
            Self::Boolean(boolean) => Ok(Value::Boolean(*boolean)),
        }
    }
}

impl Evaluate for Block {
    fn eval(&self, runtime: &mut Runtime) -> Result<Value> {
        for statement in &self.statements {
            let _ = statement.eval(runtime)?;
        }

        Ok(Value::Void)
    }
}

impl Evaluate for Statement {
    fn eval(&self, runtime: &mut Runtime) -> Result<Value> {
        match self {
            Self::SetVariable(..) => todo!(),
            Self::If(condition, block, ext) => {
                match condition.eval(runtime)? {
                    Value::Boolean(boolean) => {
                        block.eval(runtime)?;
                        if let Some(ext) = ext {
                            match &**ext {
                                // else statements:
                                Node::Block(ext_block) => {
                                    ext_block.eval(runtime)?;
                                }
                                // elif statements:
                                Node::Statement(ext_statement) => {
                                    match ext_statement {
                                        Statement::If(..) => {
                                            ext_statement.eval(runtime)?;
                                        }
                                        _ => unreachable!()
                                    }
                                }
                                _ => unreachable!()
                            };
                        }
                    }
                    _ => {
                        return Err(ExpectedType(Type::Boolean).into());
                    }
                }

                Ok(Value::Void)
            }
        }
    }
}

impl Evaluate for BinaryOp {
    fn eval(&self, runtime: &mut Runtime) -> Result<Value> {
        use crate::parser::tokens::Operator::*;

        Ok(match self.op {
            Add | Sub | Mul | Div | Pow | Mod |
            Equal | Inequal | Less | LessEqual | Greater | GreaterEqual => {
                let a = match self.a.eval(runtime)? {
                    Value::Integer(integer) => integer,
                    _ => { return Err(ExpectedType(Type::Integer).into()); }
                };
                let b = match self.b.eval(runtime)? {
                    Value::Integer(integer) => integer,
                    _ => { return Err(ExpectedType(Type::Integer).into()); }
                };

                match self.op {
                    Add | Sub | Mul | Div | Pow | Mod => {
                        Value::Integer(match self.op {
                            Add => a + b,
                            Sub => a - b,
                            Mul => a * b,
                            Div => a / b,
                            Pow => a.pow(b.try_into()
                                .expect("expected exponent of type u32")),
                            Mod => a % b,
                            _ => unreachable!()
                        })
                    }
                    Equal | Inequal | Less | LessEqual | Greater | GreaterEqual => {
                        Value::Boolean(match self.op {
                            Equal => a == b,
                            Inequal => a != b,
                            Less => a < b,
                            LessEqual => a <= b,
                            Greater => a >= b,
                            GreaterEqual => a >= b,
                            _ => unreachable!()
                        })
                    }
                    _ => unreachable!()
                }
            }

            And | Or => {
                let a = match self.a.eval(runtime)? {
                    Value::Boolean(boolean) => boolean,
                    _ => { return Err(ExpectedType(Type::Boolean).into()); }
                };
                let b = match self.b.eval(runtime)? {
                    Value::Boolean(boolean) => boolean,
                    _ => { return Err(ExpectedType(Type::Boolean).into()); }
                };

                Value::Boolean(match self.op {
                    And => a && b,
                    Or => a || b,
                    _ => unreachable!()
                })
            }

            FieldAccess | Paren => unreachable!()
        })
    }
}
