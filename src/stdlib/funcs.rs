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

pub fn get(runtime: &mut Runtime, scope: &mut Scope) -> Result<Object> {
    Ok(Object::String(get!(runtime, scope, string, String)))
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

    let stdout: String = stdout.iter().map(|b| *b as char).collect();

    Ok(Object::String(stdout))
}
