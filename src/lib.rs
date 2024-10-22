use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn setup() -> FnResult<String> {
    let stdout = helpers::setup()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn add(args: String) -> FnResult<String> {
    helpers::setup()?;
    let stdout = dag()
        .pipeline("add")?
        .with_exec(vec!["vfox", "add", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn install(args: String) -> FnResult<String> {
    helpers::setup()?;
    let stdout = dag()
        .pipeline("install")?
        .with_exec(vec!["vfox", "install", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn r#use(args: String) -> FnResult<String> {
    helpers::setup()?;
    let stdout = dag()
        .pipeline("use")?
        .with_exec(vec!["eval \"$(vfox activate bash)\" && vfox use", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn u(args: String) -> FnResult<String> {
    helpers::setup()?;
    let stdout = dag()
        .pipeline("use")?
        .with_exec(vec!["eval \"$(vfox activate bash)\" && vfox use", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn i(args: String) -> FnResult<String> {
    helpers::setup()?;
    let stdout = dag()
        .pipeline("install")?
        .with_exec(vec!["vfox", "install", &args])?
        .stdout()?;
    Ok(stdout)
}
