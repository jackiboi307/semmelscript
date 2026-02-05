use super::*;

pub fn println(runtime: &mut Runtime, scope: &mut Scope) -> Result<Object> {
    println!("{}", get!(runtime, scope, text, String));
    Ok(Object::Null)
}


// pub fn call(runtime: &mut Runtime, scope: &mut Scope) -> Result<Object> {
