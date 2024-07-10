use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup() -> Result<String, Error> {
    let home = dag().get_env("HOME")?;
    let path = dag().get_env("PATH")?;
    let shims = format!("{}/.version-fox/shims", home);
    dag().set_envs(vec![("PATH".into(), format!("{}:{}", shims, path))])?;

    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["type vfox > /dev/null || pkgx curl -sSL https://raw.githubusercontent.com/version-fox/vfox/main/install.sh | bash"])?
        .stdout()?;
    Ok(stdout)
}
