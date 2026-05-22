use std::env::home_dir;
use std::os::unix::process::CommandExt;
use std::process::Command;

use clap::Parser;
use painless_belt::cli::Cli;
use painless_belt::policy::connect_to_internet::ConnectToInternet;
use painless_belt::policy::execute_system_binaries::ExecuteSystemBinaries;
use painless_belt::policy::protect_dir::ProtectDirectory;
use painless_belt::policy::read_write_dir::ReadWriteDirectory;
use painless_belt::profile::Profile;

fn main() {
    let cli = Cli::parse();
    let mut sb = Profile::deny_by_default();

    if !cli.no_cwd {
        let cwd = std::env::current_dir().expect("current_dir");
        sb = sb.with_policy(ReadWriteDirectory::new(cwd));
    }

    if !cli.no_tmp {
        sb = sb
            .with_policy(ReadWriteDirectory::new("/tmp"))
            .with_policy(ReadWriteDirectory::new("/private/tmp"));
    }

    if !cli.no_system_binaries {
        sb = sb.with_policy(ExecuteSystemBinaries);
    }

    if !cli.no_internet {
        sb = sb.with_policy(ConnectToInternet);
    }

    let home = home_dir().expect("No home_dir");
    sb = sb
        .with_policy(ReadWriteDirectory::new(home.join(".claude")))
        .with_policy(ReadWriteDirectory::new(home.join(".claude.json")))
        .with_policy(ReadWriteDirectory::new(home.join(".config/claude")))
        .with_policy(ReadWriteDirectory::new(home.join(".local/share/claude")))
        .with_policy(ProtectDirectory::new(home.join(".ssh")));

    match sb.init() {
        Ok(()) => (),
        Err(e) => {
            eprintln!("[pb] sandbox_init failed: {e}");
            std::process::exit(1);
        }
    }

    let (program, args) = cli.command.split_first().expect("command required");
    let err = Command::new(program)
        .args(args)
        .envs(std::env::vars_os())
        .exec();
    eprintln!("[pb] exec {program:?} failed: {err}");
    std::process::exit(1);
}
