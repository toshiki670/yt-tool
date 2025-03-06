use assert_cmd::Command;
use predicates::prelude::*;
use test_case::test_case;

fn subject() -> Command {
    Command::cargo_bin("yt-tool").unwrap()
}

#[test_case("-h", "yt-tool"; "when a flag is -h")]
#[test_case("--help", "yt-tool"; "when a flag is --help")]
#[test_case("-V", "yt-tool"; "when a flag is -V")]
#[test_case("--version", "yt-tool"; "when a flag is --version")]
fn it_works_as_stdout(arg: &str, expected: &str) {
    subject()
        .arg(arg)
        .assert()
        .success()
        .stdout(predicate::str::contains(expected));
}

#[test_case("-v", "INFO"; "when a flag is -v")]
#[test_case("--verbose", "INFO"; "when a flag is --verbose")]
fn it_works_as_stderr(arg: &str, expected: &str) {
    subject()
        .arg(arg)
        .assert()
        .success()
        .stderr(predicate::str::contains(expected));
}

#[test]
fn it_works_as_completion_generation() {
    subject()
        .arg("--generate-completions")
        .arg("bash")
        .assert()
        .success();
}

#[test_case("-h", "yt-tool youtube"; "when a flag is -h")]
#[test_case("--help", "yt-tool youtube"; "when a flag is --help")]
fn it_works_as_youtube_subcommand(arg: &str, expected: &str) {
    subject()
        .arg("youtube")
        .arg(arg)
        .assert()
        .success()
        .stdout(predicate::str::contains(expected));
}

#[test_case("-h", "yt-tool youtube chat"; "when a flag is -h")]
#[test_case("--help", "yt-tool youtube chat"; "when a flag is --help")]
fn it_works_as_youtube_chat_subcommand(arg: &str, expected: &str) {
    subject()
        .arg("youtube")
        .arg("chat")
        .arg(arg)
        .assert()
        .success()
        .stdout(predicate::str::contains(expected));
}
