#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::io::Write;
use std::process::Command;

const BIN: &str = env!("CARGO_BIN_EXE_type-forge");

fn run_stdin(input: &[u8], args: &[&str]) -> Command {
    let mut cmd = Command::new(BIN);
    cmd.args(args)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());
    cmd
}

fn exec_stdin(input: &[u8], args: &[&str]) -> (bool, String, String) {
    let mut cmd = run_stdin(input, args);
    let mut child = cmd.spawn().unwrap();
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
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    cmd.stdin.take();
    let output = cmd.wait_with_output().unwrap();
    assert!(output.status.success());
    assert!(output.stdout.is_empty());
}
