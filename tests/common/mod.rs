use std::ffi::{CStr, CString, c_char};
use std::ptr;

use painless_belt::sandbox::ToSbpl;

type SandboxCompile =
    unsafe extern "C" fn(*const c_char, *const *const c_char, *mut *mut c_char) -> *mut c_char;

type SandboxFreeError = unsafe extern "C" fn(*mut c_char);

fn sandbox_compile(profile: &str) -> Result<(), String> {
    let handle = unsafe { libc::dlopen(c"/usr/lib/libsandbox.1.dylib".as_ptr(), libc::RTLD_LAZY) };
    if handle.is_null() {
        return Err("libsandbox not loadable".into());
    }
    let compile_sym = unsafe { libc::dlsym(handle, c"sandbox_compile_string".as_ptr()) };
    if compile_sym.is_null() {
        return Err("sandbox_compile_string not available".into());
    }
    let free_sym = unsafe { libc::dlsym(handle, c"sandbox_free_error".as_ptr()) };
    if free_sym.is_null() {
        return Err("sandbox_free_error not available".into());
    }
    let compile: SandboxCompile = unsafe { std::mem::transmute(compile_sym) };
    let free_error: SandboxFreeError = unsafe { std::mem::transmute(free_sym) };

    let profile_c = CString::new(profile).map_err(|_| "profile contained NUL byte".to_string())?;
    let mut errorbuf: *mut c_char = ptr::null_mut();
    let compiled = unsafe { compile(profile_c.as_ptr(), ptr::null(), &mut errorbuf) };
    if !compiled.is_null() {
        unsafe { libc::free(compiled as *mut _) };
        if !errorbuf.is_null() {
            unsafe { free_error(errorbuf) };
        }
        return Ok(());
    }
    let msg = if errorbuf.is_null() {
        String::new()
    } else {
        let s = unsafe { CStr::from_ptr(errorbuf) }
            .to_string_lossy()
            .into_owned();
        unsafe { free_error(errorbuf) };
        s
    };
    Err(msg)
}

pub fn assert_rules_compile(rules: &impl ToSbpl) {
    let body = format!("(version 1)\n(allow default)\n{}", rules.to_sbpl());
    if let Err(e) = sandbox_compile(&body) {
        panic!("sandbox_compile failed: {e}\nRendered SBPL:\n{body}");
    }
}
