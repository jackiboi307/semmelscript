mod parser;
use parser::{
    Parser,
    node::{Node, Format},
};

mod runtime;
use runtime::*;

mod stdlib;

pub use quick_error::quick_error;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const DEBUG: bool = false;

#[allow(unused)]
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

fn main() {
    let [_, path]: [String; 2] = std::env::args()
        .collect::<Vec<_>>().try_into().unwrap_or_else(|_| panic!("Expected 1 argument!"));

    let buffer = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Could not read file {path}: {e}"));

    let mut runtime = Runtime::new();
    let mut scope = Scope::new(None);

    // add functions
    stdlib::init(&mut runtime.globals);

    execute(&mut runtime, &mut scope, buffer);

    // let mut my_rust_var = 5;
    // source_with_vars! {
    //     buffer: buffer,
    //     runtime: &mut runtime,
    //     my_rust_var, x, Integer(my_rust_var);
    // }
    // println!("my_rust_var: {my_rust_var}");
}

fn execute(runtime: &mut Runtime, scope: &mut Scope, buffer: String) {
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
    if DEBUG { println!("parsed code:\n{}", node.format(0)); }

    // execute the code
    let res = match node {
        Node::Block(block) => {
            block.eval(runtime, scope)
        }
        _ => unreachable!()
    };

    match res {
        Ok(_) => {
            if DEBUG {
                println!("final objects: {:#?}", scope.objects);
            }
        }
        Err(err) => {
            eprintln!("runtime error: {err}");
        }
    }
}
