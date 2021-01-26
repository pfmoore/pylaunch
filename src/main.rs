use std::process::Command;

fn main() {
    let exe = std::env::current_exe().unwrap();
    let stem = exe.file_stem().unwrap();
    let dir = exe.parent().unwrap();
    let script = dir.join(stem).with_extension("py");
    println!("Hello, world, from {:?} in {:?}!", stem, dir);
    println!("Running {:?}", script);
    let status = Command::new("py").arg(script).status().expect("Failed");
    println!("Status: {:?}", status);
}
