use super::*;
use std::process::Command;

pub fn println(runtime: &mut Runtime, scope: &mut Scope) -> Result<Object> {
    println!("{}", get!(runtime, scope, text, String));
    Ok(Object::Null)
}

pub fn print(runtime: &mut Runtime, scope: &mut Scope) -> Result<Object> {
    print!("{}", get!(runtime, scope, text, String));
    Ok(Object::Null)
}

pub fn source(runtime: &mut Runtime, scope: &mut Scope) -> Result<Object> {
    let path = get!(runtime, scope, path, String);
    let scope: &mut Scope = unsafe { &mut *scope.parent.unwrap() };
    execute(runtime, scope, path);
    Ok(Object::Null)
}

pub fn tostring(runtime: &mut Runtime, scope: &mut Scope) -> Result<Object> {
    Ok(Object::String(match scope.get(runtime, "value").unwrap() {
        Object::String(string) => string,
        Object::Integer(integer) => integer.to_string(),
        _ => unimplemented!()
    }))
}

pub fn call(runtime: &mut Runtime, scope: &mut Scope) -> Result<Object> {
    let (shell, flag) = if cfg!(target_os = "windows") {
            ("cmd", "/C")
        } else {
            ("sh", "-c")
        };

    let stdout = Command::new(shell).arg(flag)
        .arg(get!(runtime, scope, cmd, String))
        .output()
        .expect("command failed")
        .stdout; // TODO fix

    let mut stdout: String = stdout.iter().map(|b| *b as char).collect();
    
    // remove trailing newline
    if let Some(ch) = stdout.bytes().last() {
        if ch == b'\n' {
            stdout.remove(stdout.len() - 1);
        }
    }

    Ok(Object::String(stdout))
}
