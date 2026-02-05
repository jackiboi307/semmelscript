use crate::*;
use crate::runtime::*;

pub fn println(runtime: &mut Runtime, scope: &mut Scope) -> Result<Object> {
    let text = expect_type!(scope.get_from(runtime, "text")?, String);
    let text = unsafe { (*text).clone() };
    println!("{text}");
    Ok(Object::Null)
}
