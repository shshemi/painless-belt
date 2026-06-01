use std::{ffi::OsString, os::unix::process::CommandExt, process::Command};

use anyhow::anyhow;
use painless_belt::{
    AppResult,
    cli::{RunArgs, cli},
    dir, sandbox::profile::Profile,
};

fn main() -> AppResult<()> {
    let cli = cli();
    match &cli.subcommand.as_ref().unwrap() {
        painless_belt::cli::SubCmd::Run(args) => run(args)?,
        painless_belt::cli::SubCmd::Pull(args) => {
            let p = dir::pull_profile(&args.name)?;
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
    } else {
        Profile::default()
    };
    profile.push_rules(&args.rules.to_sbpl());
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
