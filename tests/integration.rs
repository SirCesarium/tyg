#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::env;
use std::fs;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener};
use std::process::{self, Command, Stdio};
use std::thread;

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
fn yaml_to_rust() {
    let (ok, stdout, _) = exec_stdin(b"x: 1\ny: hello\n", &["--format", "yaml"]);
    assert!(ok, "stdout: {stdout}");
    assert!(stdout.contains("x"));
    assert!(stdout.contains("y"));
}

#[test]
fn toml_to_rust() {
    let (ok, stdout, _) = exec_stdin(b"x = 1\ny = \"hello\"\n", &["--format", "toml"]);
    assert!(ok, "stdout: {stdout}");
    assert!(stdout.contains("x"));
    assert!(stdout.contains("y"));
}

#[test]
fn xml_to_rust() {
    let (ok, stdout, _) = exec_stdin(b"<root><x>1</x><y>hello</y></root>", &["--format", "xml"]);
    assert!(ok, "stdout: {stdout}");
}

#[test]
fn properties_to_rust() {
    let (ok, stdout, _) = exec_stdin(b"x=1\ny=hello\n", &["--format", "properties"]);
    assert!(ok, "stdout: {stdout}");
    assert!(stdout.contains("x"));
    assert!(stdout.contains("y"));
}

#[test]
fn json_stream_merges() {
    let (ok, stdout, _) = exec_stdin(b"{\"a\":1}\n{\"b\":2}", &["--lang", "json_schema"]);
    assert!(ok, "stdout: {stdout}");
    assert!(stdout.contains("a"));
    assert!(stdout.contains("b"));
}

#[test]
fn no_input_is_empty_success() {
    let mut cmd = Command::new(BIN)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    drop(cmd.stdin.take());
    let output = cmd.wait_with_output().unwrap();
    assert!(output.status.success());
    assert!(output.stdout.is_empty());
}

/// Serves exactly one HTTP response on a throwaway local port.
fn serve_once(body: &'static str) -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut buf = [0u8; 4096];

        let _ = stream.read(&mut buf);
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );
        stream.write_all(response.as_bytes()).unwrap();
    });

    addr
}

/// URLs and local files must both contribute samples (README: "any mix").
#[test]
fn url_and_file_sources_mix() {
    let addr = serve_once(r#"{"from_url": 1}"#);
    let file = env::temp_dir().join(format!("tyg_mix_{}.json", process::id()));

    fs::write(&file, r#"{"from_file": 2}"#).unwrap();

    let url = format!("http://{addr}/sample.json");
    let output = Command::new(BIN)
        .args(["-u", &url, "--name", "Mix"])
        .arg(&file)
        .output()
        .unwrap();

    let _ = fs::remove_file(&file);

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(stdout.contains("from_url"), "stdout: {stdout}");
    assert!(stdout.contains("from_file"), "stdout: {stdout}");
}
