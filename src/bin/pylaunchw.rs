#![windows_subsystem = "windows"]

use pylaunch::Config;
use anyhow::Result;

// QUESTION: can we capture stdout/stderr for error handling?
fn main() -> Result<()> {
    let default = Config {
        exe_name: "pythonw.exe".into(),
        launcher_name: "pyw.exe".into(),
        lib_location: "lib".into(),
        env_locs: vec![".venv/Scripts".into(), "python".into(), "embedded".into()],
        script_locs: vec!["scripts".into(), "".into()],
        extensions: vec!["pyw".into(), "pyzw".into()],
    };
    let cfg = Config::from_file("pylaunch.cfg", default);
    std::process::exit(cfg.launch()?);
}
