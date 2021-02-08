#![windows_subsystem = "windows"]

use pylaunch::Config;
use anyhow::Result;

// QUESTION: can we capture stdout/stderr for error handling?
fn main() -> Result<()> {
    let cfg = Config {
        exe_name: "pythonw.exe".into(),
        launcher_name: "pyw.exe".into(),
        lib_location: "lib".into(),
        env_locs: vec![".venv/Scripts".into(), "python".into(), "embedded".into()],
        extensions: vec!["pyw".into(), "pyzw".into()],
    };

    std::process::exit(cfg.launch()?);
}
