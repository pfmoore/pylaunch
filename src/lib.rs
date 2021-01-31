use std::process::Command;

pub fn launch (ext: &str, cmd: &str) -> i32 {
    let exe = std::env::current_exe().unwrap();
    let stem = exe.file_stem().unwrap();
    let dir = exe.parent().unwrap();
    let lib = dir.join("lib");
    let script = dir.join(stem).with_extension(ext);
    /*
    println!("Hello, world, from {:?} in {:?}!", stem, dir);
    println!("Running {:?}", script);
    println!("Running {:?}", std::env::args_os());
    */
    let status = Command::new(cmd)
        .arg(script)
        /* Skip the first arg, as it's the exe name! */
        .args(std::env::args_os().skip(1))
        .env("PYTHONPATH", lib)
        .status().unwrap().code().unwrap_or(1);
    status
}
