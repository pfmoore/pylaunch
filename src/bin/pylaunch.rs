use pylaunch::Config;
use anyhow::Result;

const CFG: Config = Config {
    exe_name: "python.exe",
    launcher_name: "py.exe",
    lib_location: "lib",
    env_locs: &[".venv/Scripts", "python", "embedded"],
    extensions: &["py", "pyz", "zip"],
};


fn main() -> Result<()>{
    std::process::exit(CFG.launch()?);
}
