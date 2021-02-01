use std::process::Command;

pub fn launch (ext: &str, cmd: &str) -> i32 {
    // Result<PathBuf, std::io::Error>
    let exe = std::env::current_exe().unwrap();
    // Option<&OsStr>
    let stem = exe.file_stem().unwrap();
    // Option<&Path>
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
        .status() // std::io::Result<ExitStatus>
        .unwrap().code() // Option<i32>
        .unwrap_or(1);
    status
}
