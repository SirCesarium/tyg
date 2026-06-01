#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::io::Write;
use std::process::{Command, Stdio};

const BIN: &str = env!("CARGO_BIN_EXE_tyg");

fn exec_stdin(input: &[u8], args: &[&str]) -> (bool, String, String) {
    let mut child = Command::new(BIN)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(input).unwrap();
    let output = child.wait_with_output().unwrap();
    (
        output.status.success(),
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(output.stderr).unwrap(),
    )
}

#[test]
fn json_to_rust() {
    let (ok, stdout, _) = exec_stdin(b"{\"x\": 1}", &["--name", "Test", "--lang", "rust"]);
    assert!(ok, "stdout: {stdout}");
    assert!(stdout.contains("struct Test"));
}

#[test]
fn json_to_typescript() {
    let (ok, stdout, _) = exec_stdin(b"{\"x\": 1}", &["--lang", "typescript"]);
    assert!(ok, "stdout: {stdout}");
    assert!(stdout.contains("Root"));
}

#[test]
fn stdin_pipe_works() {
    let (ok, stdout, _) = exec_stdin(b"{\"x\": 1}", &[]);
    assert!(ok, "stdout: {stdout}");
    assert!(stdout.contains("x"));
}

#[test]
fn invalid_format_returns_error() {
    let (ok, _, _) = exec_stdin(b"{invalid}", &[]);
    assert!(!ok);
}

#[test]
fn no_input_is_empty_success() {
    let mut cmd = Command::new(BIN)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    cmd.stdin.take();
    let output = cmd.wait_with_output().unwrap();
    assert!(output.status.success());
    assert!(output.stdout.is_empty());
}
