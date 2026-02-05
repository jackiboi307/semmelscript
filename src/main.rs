mod parser;
use parser::{
    Parser,
    node::Format,
};

mod runtime;
use runtime::*;

mod stdlib;

pub use quick_error::quick_error;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    let [_, file]: [String; 2] = std::env::args()
        .collect::<Vec<_>>().try_into().unwrap_or_else(|_| panic!("Expected 1 argument!"));

    // other args
    // TODO add cli params
    let debug = false;

    let buffer = std::fs::read_to_string(&file)
        .unwrap_or_else(|e| panic!("Could not read file {file}: {e}"));

    let mut parser = Parser::new(buffer);
    let parsed = parser.parse();

    let block = match parsed {
        Ok(node) => node,
        Err(err) => {
            eprintln!("syntax error: {err} ({}:{})", parser.row(), parser.col());
            return
        }
    };

    let mut runtime = Runtime::new();
    let mut scope = Scope::new();

    // add functions
    scope.add_in(&mut runtime, "println",
        Object::Function {
            func: &Function::Pointer(stdlib::println) as *const Function,
            args: vec!["text".into()],
        });

    // print parsed output
    if debug { println!("parsed code:\n{}", block.format(0)); }

    // execute the code
    let res = block.eval(&mut runtime, &mut scope);

    match res {
        Ok(value) => {
            if debug {
                println!("final objects: {:#?}", runtime.objects);
            }
        }
        Err(err) => {
            eprintln!("runtime error: {err}");
        }
    }
}
