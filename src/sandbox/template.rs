use std::collections::BTreeMap;
use std::env;
use std::path::Path;
use std::process::Command;

use minijinja::{Environment, Value};

use crate::cli::cli;

pub fn render(template: &str) -> Result<String, minijinja::Error> {
    let cli = cli();
    let command = cli
        .command
        .first()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    let args: Vec<String> = cli
        .command
        .iter()
        .skip(1)
        .map(|s| s.to_string_lossy().into_owned())
        .collect();
    let mut ctx: BTreeMap<String, Value> = BTreeMap::new();

    let pwd = env::current_dir().map(path_to_string).unwrap_or_default();
    ctx.insert("pwd".into(), Value::from(pwd.clone()));

    let path_dirs: Vec<String> = env::var("PATH")
        .unwrap_or_default()
        .split(':')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();
    ctx.insert("path".into(), Value::from(path_dirs));

    let home = dirs::home_dir().map(path_to_string).unwrap_or_default();
    ctx.insert("home".into(), Value::from(home));

    ctx.insert("tmp_system".into(), Value::from("/tmp"));
    ctx.insert("tmp_private".into(), Value::from("/private/tmp"));
    ctx.insert(
        "tmp_user".into(),
        Value::from(env::var("TMPDIR").unwrap_or_default()),
    );

    ctx.insert(
        "user".into(),
        Value::from(env::var("USER").unwrap_or_default()),
    );
    ctx.insert("uid".into(), Value::from(unsafe { libc::getuid() }));
    ctx.insert("gid".into(), Value::from(unsafe { libc::getgid() }));

    ctx.insert("command".into(), Value::from(command));

    ctx.insert("args".into(), Value::from(args.to_vec()));

    ctx.insert("git_root".into(), Value::from(git_root(&pwd)));

    ctx.insert("arch".into(), Value::from(env::consts::ARCH));
    ctx.insert("os_version".into(), Value::from(os_version()));
    ctx.insert("hostname".into(), Value::from(hostname()));
    ctx.insert("ip".into(), Value::from(primary_ip()));

    for (k, v) in env::vars() {
        ctx.insert(format!("env_{k}"), Value::from(v));
    }
    Environment::new().render_str(template, ctx)
}

fn path_to_string<P: AsRef<Path>>(p: P) -> String {
    p.as_ref().to_string_lossy().into_owned()
}

fn git_root(pwd: &str) -> String {
    Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(pwd)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}

fn os_version() -> String {
    Command::new("sw_vers")
        .arg("-productVersion")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}

fn hostname() -> String {
    hostname::get()
        .ok()
        .map(|h| h.to_string_lossy().into_owned())
        .unwrap_or_default()
}

fn primary_ip() -> String {
    local_ip_address::local_ip()
        .ok()
        .map(|ip| ip.to_string())
        .unwrap_or_default()
}
