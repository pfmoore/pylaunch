#![windows_subsystem = "windows"]

use std::process::Command;

// TODO: Move all common code into a library, only extension and runtime are different.
// QUESTION: can we capture stdout/stderr for error handling?
fn main() {
    let exe = std::env::current_exe().unwrap();
    let stem = exe.file_stem().unwrap();
    let dir = exe.parent().unwrap();
    let lib = dir.join("lib");
    let script = dir.join(stem).with_extension("pyw");
    /*
    println!("Hello, world, from {:?} in {:?}!", stem, dir);
    println!("Running {:?}", script);
    println!("Running {:?}", std::env::args_os());
    */
    let status = Command::new("pyw")
        .arg(script)
        /* Skip the first arg, as it's the exe name! */
        .args(std::env::args_os().skip(1))
        .env("PYTHONPATH", lib)
        .status().unwrap().code().unwrap_or(1);
    std::process::exit(status);
}
