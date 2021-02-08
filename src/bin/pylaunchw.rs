#![windows_subsystem = "windows"]

use pylaunch::Config;
use anyhow::Result;

const CFG: Config = Config {
    exe_name: "pythonw.exe",
    launcher_name: "pyw.exe",
    lib_location: "lib",
    env_locs: &[".venv/Scripts", "python", "embedded"],
    extensions: &["pyw", "pyzw"],
};

// TODO: Move all common code into a library, only extension and runtime are different.
// QUESTION: can we capture stdout/stderr for error handling?
fn main() -> Result<()> {
    std::process::exit(CFG.launch()?);
}
