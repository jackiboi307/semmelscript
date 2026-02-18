pub mod parser;
pub mod runtime;
pub mod stdlib;

pub use runtime::{
    Runtime,
    Scope,
    Object,
};

use parser::*;
use parser::node::Node;
use runtime::*;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn execute(runtime: &mut Runtime, scope: &mut Scope, buffer: String) {
    let mut parser = Parser::new(buffer);
    let parsed = parser.parse();

    let node = match parsed {
        Ok(node) => node,
        Err(err) => {
            eprintln!("syntax error: {err} ({}:{})", parser.row(), parser.col());
            return
        }
    };

    // print parsed output
    // if DEBUG { println!("parsed code:\n{}", node.format(0)); }

    // execute the code
    let res = match node {
        Node::Block(block) => {
            block.eval(runtime, scope)
        }
        _ => unreachable!()
    };

    match res {
        Ok(_) => {
            // if DEBUG {
            //     println!("final objects: {:#?}", scope.objects);
            // }
        }
        Err(err) => {
            eprintln!("runtime error: {err}");
        }
    }
}

#[macro_export]
macro_rules! source_with_vars {
    (
        buffer: $buffer:expr,
        runtime: $runtime:expr,
        $($var:expr, $name:ident, $type:ident ($value:expr);)*
    ) => {
        let mut scope = Scope::new(None);
        $(
            scope.define(stringify!($name), Object::$type($value));
        )*
        execute($runtime, &mut scope, $buffer);
        $(
            $var = {
                let value = scope.get($runtime, stringify!($name)).unwrap();
                match value {
                    Object::$type(value) => value,
                    _ => panic!()
                }
            };
        )*
    }
}
