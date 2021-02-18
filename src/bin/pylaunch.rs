use pylaunch::Config;
use anyhow::Result;

fn main() -> Result<()>{
    let cfg = Config {
        exe_name: "python.exe".into(),
        launcher_name: "py.exe".into(),
        lib_location: "lib".into(),
        env_locs: vec![".venv/Scripts".into(), "python".into(), "embedded".into()],
        script_locs: vec!["scripts".into(), "".into()],
        extensions: vec!["py".into(), "pyz".into(), "zip".into()],
    };

    std::process::exit(cfg.launch()?);
}
