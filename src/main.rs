use std::{ffi::OsString, os::unix::process::CommandExt, process::Command};

use anyhow::anyhow;
use painless_belt::{
    AppResult,
    cli::{RunArgs, cli},
    config::config,
    dir, http,
    sandbox::profile::Profile,
};

fn main() -> AppResult<()> {
    let cli = cli();
    match &cli.subcommand.as_ref().unwrap() {
        painless_belt::cli::SubCmd::Run(args) => run(args)?,
        painless_belt::cli::SubCmd::Pull(args) => {
            let p = http::pull_profile(&args.name)?;
            println!("Pulled  {}", p.display());
        }
        painless_belt::cli::SubCmd::Remove(args) => {
            let p = dir::remove_profile(&args.name)?;
            println!("Removed {}", p.display());
        }
    }

    Ok(())
}

fn run(args: &RunArgs) -> AppResult<()> {
    let mut profile = if let Some(name) = args.profile.as_ref() {
        Profile::load(name)?
    } else if let Some(cmd) = args.command.first()
        && let Some(name) = config().profile_name(&cmd.as_os_str().to_string_lossy())
    {
        Profile::load(name)?
    } else {
        Profile::default()
    };
    profile = profile.with(&args.rules.rule_set());
    profile.init()?;
    exec(&args.command)?;
    Ok(())
}

fn exec(cmd: &[OsString]) -> AppResult<()> {
    let (program, args) = cmd.split_first().ok_or(anyhow!("Command not found"))?;
    Err(Command::new(program)
        .args(args)
        .envs(std::env::vars_os())
        .exec())?;
    Ok(())
}
