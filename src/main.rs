use std::{ffi::OsString, os::unix::process::CommandExt, process::Command};

use anyhow::anyhow;
use clap::CommandFactory;
use clap_complete::generate;
use painless_belt::{
    AppResult,
    cli::{Cli, RunArgs, cli},
    config::config,
    dir, http,
    sandbox::profile::Profile,
};

fn main() -> AppResult<()> {
    if std::env::args().len() == 1 {
        Cli::command().print_help()?;
        return Ok(());
    }
    let cli = cli();

    if let Some(shell) = cli.generate_completion {
        let mut cmd = Cli::command();
        generate(shell, &mut cmd, "pb", &mut std::io::stdout());
        return Ok(());
    }

    match &cli.subcommand.as_ref().unwrap() {
        painless_belt::cli::SubCmd::Run(args) => run(args)?,
        painless_belt::cli::SubCmd::Pull(args) => {
            let p = http::pull_profile(&args.name, &args.name)?;
            println!("Pulled  {}", p.display());
        }
        painless_belt::cli::SubCmd::Clone(args) => {
            if dir::copy_profile(&args.src, &args.dst).is_err() {
                http::pull_profile(&args.src, &args.dst)?;
            }
            println!("Cloned {}", &args.dst);
        }
        painless_belt::cli::SubCmd::Rm(args) => {
            let p = dir::remove_profile(&args.name)?;
            println!("Removed {}", p.display());
        }
        painless_belt::cli::SubCmd::Edit(args) => {
            let path = dir::profile_path(&args.name)?;
            let editor = std::env::var("VISUAL")
                .or_else(|_| std::env::var("EDITOR"))
                .unwrap_or_else(|_| "vi".to_string());
            Command::new(&editor).arg(&path).status()?;
        }
        painless_belt::cli::SubCmd::Ls => {
            for name in dir::list_profiles()? {
                println!("{name}");
            }
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
