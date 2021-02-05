use std::process::Command;
use std::path::{Path, PathBuf};
use anyhow::{Context,Result};

pub struct Config {
    pub exe_name: &'static str,
    pub launcher_name: &'static str,
    pub lib_location: &'static str,
    pub env_locs: &'static [&'static str],
    pub extensions: &'static [&'static str],
}

fn find_python(dir: &Path, cfg: &Config) -> PathBuf {
    for loc in cfg.env_locs {
        // TODO: Should be pythonw for GUI version
        let interp = dir.join(loc).join(cfg.exe_name);
        // println!("{:?}", interp);
        if interp.exists() {
            return interp;
        }
    }
    PathBuf::from(cfg.launcher_name)
}

fn find_script(exe: &Path, cfg: &Config) -> Option<PathBuf> {
    let mut script = PathBuf::from(exe);
    for ext in cfg.extensions {
        script.set_extension(ext);
        if script.exists() {
            return Some(script);
        }
    }
    None
}

fn find_lib(dir: &Path, cfg: &Config) -> Option<PathBuf> {
    let lib = dir.join(cfg.lib_location);
    if lib.exists() {
        Some(lib)
    } else {
        None
    }
}

pub fn launch (cfg: &Config) -> Result<i32> {
    // Result<PathBuf, std::io::Error>
    let exe = std::env::current_exe().context("Could not get current exe name")?;
    // Option<&Path>
    let dir = exe.parent().context("Current exe does not have a directory")?;
    let interpreter = find_python(&dir, &cfg);
    let script = find_script(&exe, &cfg).unwrap();
    let lib = find_lib(dir, &cfg);
    /*
    println!("Hello, world, from {:?} in {:?}!", stem, dir);
    println!("Running {:?}", script);
    println!("Running {:?}", std::env::args_os());
    */
    let mut cmd = Command::new(interpreter);
    cmd.arg(script)
        /* Skip the first arg, as it's the exe name! */
        .args(std::env::args_os().skip(1));
    if let Some(l) = lib {
        cmd.env("PYTHONPATH", l);
    }
    let status = cmd.status() // std::io::Result<ExitStatus>
        .context("Could not run command")?
        .code() // Option<i32>
        .unwrap_or(1);
    Ok(status)
}
