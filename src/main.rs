use semmel::{
    runtime::*,
    stdlib,
    execute,
};

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
}
