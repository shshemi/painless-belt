use std::{ffi::OsString, sync::OnceLock};

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "painless-belt",
    about = "Run a command in a macOS sandbox with sensible defaults."
)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Option<SubCmd>,
}

#[derive(Subcommand, Debug)]
pub enum SubCmd {
    /// Run a command inside the sandbox (default).
    Run(RunArgs),
    /// Pull a template.
    Pull(PullArgs),
    /// Remove a previously-pulled template.
    Remove(RemoveArgs),
}

impl Default for SubCmd {
    fn default() -> Self {
        SubCmd::Run(Default::default())
    }
}

#[derive(Args, Debug, Default)]
pub struct RunArgs {
    /// Sanbox profile name.
    pub profile: Option<String>,

    /// Disable outbound network access.
    #[arg(long = "no-internet")]
    pub no_internet: bool,

    /// Disable execution of system binaries (/usr/bin, /bin).
    #[arg(long = "no-system-binaries")]
    pub no_system_binaries: bool,

    /// Disable read/write access to the current working directory.
    #[arg(long = "no-cwd")]
    pub no_cwd: bool,

    /// Disable read/write access to /tmp (and /private/tmp).
    #[arg(long = "no-tmp")]
    pub no_tmp: bool,

    /// Command and args to run inside the sandbox (after `--`).
    #[arg(last = true)]
    pub command: Vec<OsString>,
}

#[derive(Args, Debug)]
pub struct PullArgs {
    /// Name of the template to pull.
    pub name: String,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// Name of the template to remove.
    pub name: String,
}

impl Cli {
    /// Returns the run args if the active subcommand is `Run`, else `None`.
    pub fn run_args(&self) -> Option<&RunArgs> {
        match &self.subcommand {
            Some(SubCmd::Run(args)) => Some(args),
            _ => None,
        }
    }
}

pub fn cli() -> &'static Cli {
    static OL: OnceLock<Cli> = OnceLock::<Cli>::new();
    OL.get_or_init(|| {
        let mut cli = Cli::parse();
        if cli.subcommand.is_none() {
            cli.subcommand = Some(Default::default());
        }
        cli
    })
}
