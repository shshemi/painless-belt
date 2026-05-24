use std::{ffi::OsString, sync::OnceLock};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "painless-belt",
    about = "Run a command in a macOS sandbox with sensible defaults."
)]
pub struct Cli {
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
    #[arg(last = true, required = true)]
    pub command: Vec<OsString>,
}

pub fn cli() -> &'static Cli {
    static OL: OnceLock<Cli> = OnceLock::<Cli>::new();
    OL.get_or_init(Cli::parse)
}
