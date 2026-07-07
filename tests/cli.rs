use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;

#[test]
fn renders_file_when_forced() {
    let mut file = tempfile::NamedTempFile::new().unwrap();
    writeln!(file, "# Title\n\nA paragraph with wrapping words.").unwrap();

    Command::cargo_bin("mdr")
        .unwrap()
        .args(["--render", "always", "--color", "never", "--width", "24"])
        .arg(file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("# Title"));
}

#[test]
fn renders_stdin_when_forced() {
    Command::cargo_bin("mdr")
        .unwrap()
        .args(["--render", "always", "--color", "never", "--width", "24"])
        .write_stdin("hello world")
        .assert()
        .success()
        .stdout(predicate::str::contains("hello world"));
}

#[test]
fn auto_mode_passes_raw_markdown_when_not_tty() {
    let mut file = tempfile::NamedTempFile::new().unwrap();
    write!(file, "**raw**").unwrap();

    Command::cargo_bin("mdr")
        .unwrap()
        .arg(file.path())
        .assert()
        .success()
        .stdout("**raw**");
}

#[test]
fn missing_file_reports_error() {
    Command::cargo_bin("mdr")
        .unwrap()
        .arg("missing.md")
        .assert()
        .failure()
        .stderr(predicate::str::contains("failed to read input file"));
}

#[test]
fn color_never_emits_no_ansi() {
    Command::cargo_bin("mdr")
        .unwrap()
        .args(["--render", "always", "--color", "never"])
        .write_stdin("**bold**")
        .assert()
        .success()
        .stdout(predicate::str::contains("\u{1b}").not());
}

#[test]
fn color_always_emits_ansi() {
    Command::cargo_bin("mdr")
        .unwrap()
        .args(["--render", "always", "--color", "always"])
        .write_stdin("**bold**")
        .assert()
        .success()
        .stdout(predicate::str::contains("\u{1b}"));
}
