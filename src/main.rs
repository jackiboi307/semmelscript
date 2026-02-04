mod parser;
use parser::Parser;

pub use quick_error::quick_error;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    let [_, file]: [String; 2] = std::env::args()
        .collect::<Vec<_>>().try_into().unwrap_or_else(|_| panic!("Expected 1 argument!"));

    let buffer = std::fs::read_to_string(&file).unwrap_or_else(|e| panic!("Could not read file {file}: {e}"));
    let mut parser = Parser::new(buffer);

    if let Err(err) = parser.parse() {
        eprintln!("Error while parsing: {} ({}:{})", err, parser.row(), parser.col());
    }
}
