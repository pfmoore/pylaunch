use pylaunch::launch;
use anyhow::Result;

fn main() -> Result<()>{
    std::process::exit(launch("py", "py")?);
}
