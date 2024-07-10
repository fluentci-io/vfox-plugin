use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup() -> Result<String, Error> {
    let home = dag().get_env("HOME")?;
    let path = dag().get_env("PATH")?;
    let shims = format!("{}/.version-fox/shims", home);
    dag().set_envs(vec![("PATH".into(), format!("{}:{}", shims, path))])?;

    let gh_token = dag().get_env("GITHUB_ACCESS_TOKEN")?;

    if !gh_token.is_empty() && !gh_token.starts_with("token ") {
        dag().set_envs(vec![(
            "GITHUB_ACCESS_TOKEN".into(),
            format!("token {}", gh_token),
        )])?;
    }

    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["type vfox > /dev/null || pkgx curl -sSL https://raw.githubusercontent.com/fluentci-io/vfox-installer/main/install.sh | bash"])?
        .stdout()?;
    Ok(stdout)
}
