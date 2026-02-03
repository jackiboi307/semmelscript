pub mod parser;
pub mod node;
pub mod syntax;
pub mod error;
pub mod prelude;

use parser::Parser;

fn main() {
    let [_, file] = std::env::args()
        .collect::<Vec<_>>().try_into().unwrap_or_else(|_| panic!("Expected 1 argument!"));

    let buffer = std::fs::read_to_string(&file).unwrap_or_else(|e| panic!("Could not read file {file}: {e}"));
    let mut parser = Parser::new(buffer);

    if let Err(err) = parser.parse() {
        eprintln!("Error while parsing: {} ({}:{})", err, parser.row(), parser.col());
    }
}
