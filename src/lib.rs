use std::process::Command;
use std::path::{Path, PathBuf};
use anyhow::{Context,Result};

pub struct Config {
    pub exe_name: String,
    pub launcher_name: String,
    pub lib_location: String,
    pub env_locs: Vec<String>,
    pub extensions: Vec<String>,
}

impl Config {

    fn find_python(&self, dir: &Path) -> PathBuf {
        for loc in &self.env_locs {
            // TODO: Should be pythonw for GUI version
            let interp = dir.join(loc).join(&self.exe_name);
            // println!("{:?}", interp);
            if interp.exists() {
                return interp;
            }
        }
        PathBuf::from(&self.launcher_name)
    }

    fn find_script(&self, exe: &Path) -> Option<PathBuf> {
        let mut script = PathBuf::from(exe);
        for ext in &self.extensions {
            script.set_extension(ext);
            if script.exists() {
                return Some(script);
            }
        }
        None
    }

    fn find_lib(&self, dir: &Path) -> Option<PathBuf> {
        let lib = dir.join(&self.lib_location);
        if lib.exists() {
            Some(lib)
        } else {
            None
        }
    }

    pub fn launch (&self) -> Result<i32> {
        // Result<PathBuf, std::io::Error>
        let exe = std::env::current_exe().context("Could not get current exe name")?;
        // Option<&Path>
        let dir = exe.parent().context("Current exe does not have a directory")?;
        let interpreter = self.find_python(&dir);
        let script = self.find_script(&exe).unwrap();
        let lib = self.find_lib(dir);
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
}