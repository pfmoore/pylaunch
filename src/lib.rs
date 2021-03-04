use std::process::Command;
use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Context,Result};
use serde::Deserialize;
use serde_json;


#[derive(Deserialize)]
struct UserConfig {
    pub lib_location: Option<Vec<String>>,
    pub env_locs: Option<Vec<String>>,
    pub script_locs: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct Config {
    pub exe_name: String,
    pub launcher_name: String,
    pub extensions: Vec<String>,
    pub lib_location: Vec<String>,
    pub env_locs: Vec<String>,
    pub script_locs: Vec<String>,
}

impl Config {

    pub fn from_file<P: AsRef<Path>> (filename: P, default: Config) -> Config {
        let exe = std::env::current_exe().expect("Could not get current exe name");
        let mut result = default;
        let file = filename.as_ref();
        let file = exe.parent().unwrap().join(file);
        if file.exists() {
            let contents = fs::read_to_string(file)
                .expect("Something went wrong reading the file");
            let config: UserConfig = serde_json::from_str(&contents).unwrap();
            if let Some(lib_location) = config.lib_location { result.lib_location = lib_location }
            if let Some(env_locs) = config.env_locs { result.env_locs = env_locs }
            if let Some(script_locs) = config.script_locs { result.script_locs = script_locs }
        }
        result
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

    pub fn launch (&self) -> Result<i32> {
        // Result<PathBuf, std::io::Error>
        let exe = std::env::current_exe().context("Could not get current exe name")?;
        // Option<&Path>
        let dir = exe.parent().context("Current exe does not have a directory")?;
        let interpreter = self.find_python(&dir);
        let script = self.find_script(&exe).unwrap();
        /*
        println!("Hello, world, from {:?} in {:?}!", stem, dir);
        println!("Running {:?}", script);
        println!("Running {:?}", std::env::args_os());
        */
        let mut cmd = Command::new(interpreter);
        cmd.arg(script)
            /* Skip the first arg, as it's the exe name! */
            .args(std::env::args_os().skip(1));

        let pythonpath = std::env::join_paths(
            self.lib_location.iter()
                .map(|d| dir.join(&d))
                .filter(|d| d.exists())
            ).unwrap();
        if pythonpath.len() > 0 {
            cmd.env("PYTHONPATH", pythonpath);
        }
        let status = cmd.status() // std::io::Result<ExitStatus>
            .context("Could not run command")?
            .code() // Option<i32>
            .unwrap_or(1);
        Ok(status)
    }
}