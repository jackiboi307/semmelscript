use crate::*;
use crate::parser::node::*;
use std::collections::HashMap;

type IntegerType = i32;

quick_error! {
    #[derive(Debug)]
    pub enum RuntimeError {
        ExpectedType(typ: Type) {}
        ExpectedArgs(len: usize) {}
        NameError(name: Box<str>) {}
    }
}

use RuntimeError::*;

#[macro_export]
macro_rules! expect_type {
    ($value:expr, $type:ident) => {{
        use crate::runtime::{Object, Type, RuntimeError};
        match $value {
            Object::$type(value) => value,
            _ => { return Err( RuntimeError::ExpectedType(Type::$type).into()); }
        }}
    }
}

pub struct Runtime;

#[derive(Debug, Clone)]
pub struct Scope {
    pub objects: Vec<Object>,
    pub names: HashMap<Box<str>, usize>,
    pub parent: Option<*mut Scope>,
}

#[derive(Debug, Clone)]
pub enum Function {
    Pointer(fn(&mut Runtime, &mut Scope) -> Result<Object>),
    Block(Node),
}

#[derive(Debug)]
pub enum Type {
    Null,
    String,
    Integer,
    Boolean,
    Function,
}

#[derive(Debug, Clone)]
pub enum Object {
    Null,
    String(String),
    Integer(IntegerType),
    Boolean(bool),
    Function {
        func: *const Function,
        args: Vec<Box<str>>,
    },
}

impl Runtime {
    pub fn new() -> Self {
        Self {}
    }
}

impl Scope {
    pub fn new(parent: Option<*mut Scope>) -> Self {
        Self {
            names: HashMap::new(),
            objects: Vec::new(),
            parent,
        }
    }

    fn add_object(&mut self, object: Object) -> usize {
        self.objects.push(object);
        self.objects.len() - 1
    }

    fn bind_name(&mut self, name: &str, id: usize) {
        self.names.insert(name.into(), id);
    }

    pub fn define(&mut self, _runtime: &mut Runtime, name: &str, object: Object) {
        assert!(!self.names.contains_key(name)); // TODO fix
        let id = self.add_object(object);
        self.names.insert(name.into(), id);
    }

    pub fn update(&mut self, runtime: &mut Runtime, name: &str, object: Object) {
        // TODO fix
        self.objects.insert(*self.names.get(name).unwrap(), object);
    }

    pub fn get(&mut self, _runtime: &Runtime, ident: &str) -> Result<Object> {
        let id = self.names.get(ident).ok_or_else(|| NameError(ident.into()))?;
        Ok(self.objects[*id].clone())
    }
}

pub trait Evaluate {
    // evaluates the value of a node
    fn eval(&self, _runtime: &mut Runtime, _scope: &mut Scope) -> Result<Object> {
        // TODO remove
        unimplemented!()
    }
}

impl Evaluate for Node {
    fn eval(&self, runtime: &mut Runtime, scope: &mut Scope) -> Result<Object> {
        match self {
            Self::ParenArgs(root, args) => {
                match root.eval(runtime, scope)? {
                    Object::Function { func, args: arg_names } => {
                        if args.len() != arg_names.len() {
                            return Err(ExpectedArgs(arg_names.len()).into());
                        }

                        // TODO this is really shitty, functions should be ran in
                        // isolated scopes, but this makes sourcing possible which
                        // is important functionaliy
                        let mut func_scope = Scope::new(Some(scope));
                        for (i, arg_name) in arg_names.iter().enumerate() {
                            let arg_node: &Node = &args[i];
                            let arg = arg_node.eval(runtime, scope)?;
                            func_scope.define(runtime, arg_name, arg);
                        }

                        let func = unsafe { (*func).clone() };
                        match func {
                            Function::Pointer(ptr) => {
                                ptr(runtime, &mut func_scope)
                            }
                            Function::Block(block) => {
                                block.eval(runtime, &mut func_scope)
                            }
                        }
                    }
                    _ => Err(ExpectedType(Type::Function).into())
                }
            }

            Self::Statement(node) => node.eval(runtime, scope), 
            Self::BinaryOp(node) => node.eval(runtime, scope), 

            Self::Block(node) => {
                node.eval(runtime, &mut scope.clone())
            }

            Self::Identifier(ident) => scope.get(runtime, ident),
            Self::String(string) => Ok(Object::String(string.clone())),
            Self::Integer(integer) => Ok(Object::Integer(*integer)),
            Self::Boolean(boolean) => Ok(Object::Boolean(*boolean)),
        }
    }
}

impl Evaluate for Block {
    fn eval(&self, runtime: &mut Runtime, scope: &mut Scope) -> Result<Object> {
        for statement in &self.statements {
            let _ = statement.eval(runtime, scope)?;
        }

        Ok(Object::Null)
    }
}

impl Evaluate for Statement {
    fn eval(&self, runtime: &mut Runtime, scope: &mut Scope) -> Result<Object> {
        match self {
            Self::DefineVariable(name, value) => {
                let value = value.eval(runtime, scope)?;
                scope.define(runtime, name, value);
                Ok(Object::Null)
            }
            Self::If(condition, block, ext) => {
                if expect_type!(condition.eval(runtime, scope)?, Boolean) {
                    block.eval(runtime, scope)?;
                    if let Some(ext) = ext {
                        match &**ext {
                            // else statements:
                            Node::Block(ext_block) => {
                                ext_block.eval(runtime, scope)?;
                            }
                            // elif statements:
                            Node::Statement(ext_statement) => {
                                match ext_statement {
                                    Statement::If(..) => {
                                        ext_statement.eval(runtime, scope)?;
                                    }
                                    _ => unreachable!()
                                }
                            }
                            _ => unreachable!()
                        };
                    }
                }

                Ok(Object::Null)
            }
        }
    }
}

impl Evaluate for BinaryOp {
    fn eval(&self, runtime: &mut Runtime, scope: &mut Scope) -> Result<Object> {
        use crate::parser::tokens::Operator::*;

        Ok(match self.op {
            Add | Sub | Mul | Div | Pow | Mod |
            Equal | Inequal | Less | LessEqual | Greater | GreaterEqual => {
                let a = self.a.eval(runtime, scope)?;
                let b = self.b.eval(runtime, scope)?;

                match self.op {
                    Add => {
                        // string concatenation
                        match a {
                            Object::String(a) => {
                                let b = expect_type!(b, String);
                                return Ok(Object::String(a + &b))
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }

                let a = expect_type!(a, Integer);
                let b = expect_type!(b, Integer);

                match self.op {
                    Add | Sub | Mul | Div | Pow | Mod => {
                        Object::Integer(match self.op {
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
                        Object::Boolean(match self.op {
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
                let a = expect_type!(self.a.eval(runtime, scope)?, Boolean);
                let b = expect_type!(self.b.eval(runtime, scope)?, Boolean);

                Object::Boolean(match self.op {
                    And => a && b,
                    Or => a || b,
                    _ => unreachable!()
                })
            }

            SetValue => {
                let name = expect_type!(self.a.eval(runtime, scope)?, String);
                let value = self.b.eval(runtime, scope)?;
                scope.update(runtime, &name, value);
                Object::Null
            }

            FieldAccess | Paren => unreachable!()
        })
    }
}
