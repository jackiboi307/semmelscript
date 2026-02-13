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

fn execute(runtime: &mut Runtime, scope: &mut Scope, filename: String) {
    let buffer = std::fs::read_to_string(&filename)
        .unwrap_or_else(|e| panic!("Could not read file {filename}: {e}"));

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

fn main() {
    let [_, filename]: [String; 2] = std::env::args()
        .collect::<Vec<_>>().try_into().unwrap_or_else(|_| panic!("Expected 1 argument!"));

    let mut runtime = Runtime::new();
    let mut scope = Scope::new(None);

    // add functions
    stdlib::init(&mut runtime.globals);

    execute(&mut runtime, &mut scope, filename);
}
