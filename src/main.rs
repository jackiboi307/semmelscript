mod parser;
use parser::{
    Parser,
    node::Format,
};

mod runtime;
use runtime::{Runtime, Evaluate};

pub use quick_error::quick_error;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    let [_, file]: [String; 2] = std::env::args()
        .collect::<Vec<_>>().try_into().unwrap_or_else(|_| panic!("Expected 1 argument!"));

    let buffer = std::fs::read_to_string(&file).unwrap_or_else(|e| panic!("Could not read file {file}: {e}"));

    let mut parser = Parser::new(buffer);
    println!("parsing...");
    let parsed = parser.parse();

    match parsed {
        Ok(node) => {
            // print parsed output
            println!("parsed code:\n{}", node.format(0));

            // execute the code
            println!("executing...");
            let mut runtime = Runtime {};
            let res = node.eval(&mut runtime);

            match res {
                Ok(value) => {
                    println!("final value (expected to be void): {value:?}");
                }
                Err(err) => {
                    eprintln!("runtime error: {err}");
                }
            }
        }
        Err(err) => {
            eprintln!("syntax error: {err} ({}:{})", parser.row(), parser.col());
        }
    }
}
