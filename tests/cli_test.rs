use assert_cmd::Command;
use predicates::prelude::*;
use test_case::test_case;

#[test_case("-h", "yt-tool"; "when a flag is -h")]
#[test_case("--help", "yt-tool"; "when a flag is --help")]
#[test_case("-V", "yt-tool"; "when a flag is -V")]
#[test_case("--version", "yt-tool"; "when a flag is --version")]
fn it_works_as_stdout(arg: &str, expected: &str) {
    let mut cmd = Command::cargo_bin("yt-tool").unwrap();
    cmd.arg(arg);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(expected));
}

#[test_case("-v", "INFO"; "when a flag is -v")]
#[test_case("--verbose", "INFO"; "when a flag is --verbose")]
fn it_works_as_stderr(arg: &str, expected: &str) {
    let mut cmd = Command::cargo_bin("yt-tool").unwrap();
    cmd.arg(arg);
    cmd.assert()
        .success()
        .stderr(predicate::str::contains(expected));
}

#[test]
fn test_completion_generation() {
    let mut cmd = Command::cargo_bin("yt-tool").unwrap();
    cmd.arg("--generate-completions").arg("bash");
    cmd.assert().success();
}

#[test]
fn test_youtube_subcommand() {
    let mut cmd = Command::cargo_bin("yt-tool").unwrap();
    cmd.arg("youtube").arg("--help");
    cmd.assert().success();
}
