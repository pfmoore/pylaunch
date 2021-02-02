use std::process::Command;
use std::path::{Path, PathBuf};

fn find_python(exe: &Path, default: &str) -> PathBuf {
    let dir = exe.parent().unwrap();
    let locs = &[".venv/Scripts", "embedded", "python"];
    for loc in locs {
        let interp = dir.join(loc).join("python.exe");
        // println!("{:?}", interp);
        if interp.exists() {
            return interp;
        }
    }
    PathBuf::from(default)
}

fn find_script(exe: &Path, ext: &str) -> PathBuf {
    // Option<&OsStr>
    let stem = exe.file_stem().unwrap();
    // Option<&Path>
    let dir = exe.parent().unwrap();
    let mut script = dir.join(stem);
    script.set_extension(ext);
    script
}

fn find_lib(exe: &Path) -> PathBuf {
    // Option<&Path>
    let dir = exe.parent().unwrap();
    dir.join("lib")
}

pub fn launch (ext: &str, cmd: &str) -> i32 {
    // Result<PathBuf, std::io::Error>
    let exe = std::env::current_exe().unwrap();
    let interpreter = find_python(&exe, cmd);
    let script = find_script(&exe, ext);
    let lib = find_lib(&exe);
    /*
    println!("Hello, world, from {:?} in {:?}!", stem, dir);
    println!("Running {:?}", script);
    println!("Running {:?}", std::env::args_os());
    */
    let status = Command::new(interpreter)
        .arg(script)
        /* Skip the first arg, as it's the exe name! */
        .args(std::env::args_os().skip(1))
        .env("PYTHONPATH", lib)
        .status() // std::io::Result<ExitStatus>
        .unwrap().code() // Option<i32>
        .unwrap_or(1);
    status
}
