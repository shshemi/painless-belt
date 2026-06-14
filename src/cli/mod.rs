pub mod rule_args;

use std::{ffi::OsString, sync::OnceLock};

use clap::{Args, Parser, Subcommand};
use clap_complete::Shell;

use rule_args::RuleArgs;

#[derive(Parser, Debug)]
#[command(
    name = "painless-belt",
    about = "Run a command in a macOS sandbox with sensible defaults.",
    args_conflicts_with_subcommands = true
)]
pub struct Cli {
    /// Ignore other arguments and print a shell-completion script to stdout.
    #[arg(long, value_name = "SHELL")]
    pub generate_completion: Option<Shell>,

    #[command(subcommand)]
    pub subcommand: Option<SubCmd>,

    #[command(flatten)]
    pub run: RunArgs,
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

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand, Debug)]
pub enum SubCmd {
    /// Run a command inside the sandbox (default).
    Run(RunArgs),
    /// Pull a template.
    Pull(PullArgs),
    /// Clone a profile (local copy, falls back to fetching upstream).
    Clone(CloneArgs),
    /// Open a profile in $EDITOR.
    Edit(EditArgs),
    /// Remove a previously-pulled template.
    Rm(RemoveArgs),
    /// List all downloaded profiles (including default and empty).
    Ls,
}

impl Default for SubCmd {
    fn default() -> Self {
        SubCmd::Run(Default::default())
    }
}

#[derive(Args, Debug, Default)]
pub struct RunArgs {
    /// Sandbox profile name(s). Pass more than one to run against their
    /// union, e.g. `-p base python`.
    #[arg(short, long, value_name = "PROFILE", num_args = 1..)]
    pub profile: Vec<String>,

    #[command(flatten)]
    pub rules: RuleArgs,

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
pub struct CloneArgs {
    /// Source profile name (local file or upstream).
    pub src: String,
    /// Destination profile name (saved under ~/.painless-belt/profiles).
    pub dst: String,
}

#[derive(Args, Debug)]
pub struct EditArgs {
    /// Name of the profile to open in $EDITOR.
    pub name: String,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// Name of the template to remove.
    pub name: String,
}

pub fn cli() -> &'static Cli {
    static OL: OnceLock<Cli> = OnceLock::<Cli>::new();
    OL.get_or_init(|| {
        let mut cli = Cli::parse();
        if cli.subcommand.is_none() {
            cli.subcommand = Some(SubCmd::Run(std::mem::take(&mut cli.run)));
        }
        cli
    })
}
