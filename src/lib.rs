use std::process::Command;
use std::path::{Path, PathBuf};
use anyhow::{Context,Result};

fn find_python(dir: &Path, default: &str) -> PathBuf {
    let locs = &[".venv/Scripts", "embedded", "python"];
    for loc in locs {
        // TODO: Should be pythonw for GUI version
        let interp = dir.join(loc).join("python.exe");
        // println!("{:?}", interp);
        if interp.exists() {
            return interp;
        }
    }
    PathBuf::from(default)
}

fn find_script(exe: &Path, ext: &str) -> PathBuf {
    let mut script = PathBuf::from(exe);
    script.set_extension(ext);
    script
}

fn find_lib(dir: &Path) -> PathBuf {
    dir.join("lib")
}

pub fn launch (ext: &str, cmd: &str) -> Result<i32> {
    // Result<PathBuf, std::io::Error>
    let exe = std::env::current_exe().context("Could not get current exe name")?;
    // Option<&Path>
    let dir = exe.parent().context("Current exe does not have a directory")?;
    let interpreter = find_python(&dir, cmd);
    let script = find_script(&exe, ext);
    let lib = find_lib(&dir);
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
        .context("Could not run command")?
        .code() // Option<i32>
        .unwrap_or(1);
    Ok(status)
}
