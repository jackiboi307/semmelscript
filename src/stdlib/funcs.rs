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
