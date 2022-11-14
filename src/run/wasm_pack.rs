use crate::{config::Config, util};
use anyhow::{Context, Result};
use simplelog as log;
use std::path::Path;
use xshell::{cmd, Shell};

pub fn run(command: &str, path: &str, config: &Config) -> Result<()> {
    try_build(command, &path, config).context(format!("wasm-pack {command} {path}"))?;

    util::rm_file(format!("target/site/pkg/.gitignore"))?;
    util::rm_file(format!("target/site/pkg/package.json"))?;
    Ok(())
}

pub fn try_build(command: &str, path: &str, config: &Config) -> Result<()> {
    let path_depth = Path::new(path).components().count();
    let to_root = (0..path_depth).map(|_| "..").collect::<Vec<_>>().join("/");

    let dest = format!("{to_root}/target/site/pkg");

    let sh = Shell::new()?;

    log::debug!("Running sh in path: <bold>{path}</>");
    sh.change_dir(path);

    let release = config.release.then(|| "--release").unwrap_or("--dev");
    let features = config.csr.then(|| "csr").unwrap_or("hydrate");

    cmd!(
        sh,
        "wasm-pack {command} --target web --out-dir {dest} --out-name app --no-typescript {release} -- --no-default-features --features={features}"
    )
    .run()?;
    Ok(())
}
