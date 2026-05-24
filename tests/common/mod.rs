use nix::sys::wait::{WaitStatus, waitpid};
use nix::unistd::{ForkResult, fork};
use painless_belt::sandbox::profile::Profile;

pub const EXIT_OK: i32 = 0;
pub const EXIT_SANDBOX_INIT_FAILED: i32 = 100;

pub fn fork_run<F: FnOnce() -> i32>(body: F) -> i32 {
    match unsafe { fork() }.expect("fork") {
        ForkResult::Parent { child } => match waitpid(child, None).expect("waitpid") {
            WaitStatus::Exited(_, code) => code,
            other => panic!("child did not exit normally: {other:?}"),
        },
        ForkResult::Child => std::process::exit(body()),
    }
}

pub fn assert_profile_initializes(profile: Profile) {
    let rendered = profile.as_ref().to_owned();
    let code = fork_run(move || match profile.init() {
        Ok(()) => EXIT_OK,
        Err(e) => {
            eprintln!("sandbox_init error: {e}");
            EXIT_SANDBOX_INIT_FAILED
        }
    });
    assert_eq!(
        code, EXIT_OK,
        "sandbox_init failed (exit={code}). Rendered SBPL:\n{rendered}"
    );
}
