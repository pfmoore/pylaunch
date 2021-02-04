#![windows_subsystem = "windows"]

use pylaunch::launch;
use anyhow::Result;

// TODO: Move all common code into a library, only extension and runtime are different.
// QUESTION: can we capture stdout/stderr for error handling?
fn main() -> Result<()> {
    std::process::exit(launch("pyw", "pyw")?);
}
