use std::process::Command;
use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Context,Result};
use serde::Deserialize;
use serde_json;


#[derive(Deserialize)]
pub struct Config {
    pub exe_name: String,
    pub launcher_name: String,
    pub lib_location: String,
    pub env_locs: Vec<String>,
    pub script_locs: Vec<String>,
    pub extensions: Vec<String>,
}

impl Config {

    pub fn from_file<P: AsRef<Path>> (filename: P) -> Option<Config> {
        let file = filename.as_ref();
        if file.exists() {
            let contents = fs::read_to_string(filename)
                .expect("Something went wrong reading the file");
            Some(serde_json::from_str(&contents).unwrap())
        } else {
            None
        }
    }

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
        for loc in &self.script_locs {
            let mut script = exe.parent()?.join(loc).join(exe.file_stem()?);
            for ext in &self.extensions {
                script.set_extension(ext);
                if script.exists() {
                    return Some(script);
                }
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