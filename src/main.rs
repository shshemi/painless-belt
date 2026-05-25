use std::{ffi::OsString, os::unix::process::CommandExt, process::Command};

use anyhow::anyhow;
use painless_belt::{AppResult, cli::cli, sandbox::profile::Profile};

fn main() -> AppResult<()> {
    let cli = cli();
    match &cli.subcommand.as_ref().unwrap() {
        painless_belt::cli::SubCmd::Run(args) => {
            //
            let profile = if let Some(name) = args.profile.as_ref() {
                Profile::load(name)?
            } else {
                Profile::default()
            };
            profile.init()?;
            exec(&args.command)?;
        }
        painless_belt::cli::SubCmd::Pull(pull_args) => todo!(),
        painless_belt::cli::SubCmd::Remove(remove_args) => todo!(),
    }

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
