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

mod completion_generation {
    use super::*;

    #[test]
    fn it_works_as_completion_generation() {
        subject()
            .arg("--generate-completions")
            .arg("bash")
            .assert()
            .success();
    }
}

mod youtube {
    use super::*;
    use test_case::test_case;

    #[test_case("-h", "yt-tool youtube"; "when a flag is -h")]
    #[test_case("--help", "yt-tool youtube"; "when a flag is --help")]
    fn it_works(arg: &str, expected: &str) {
        subject()
            .arg("youtube")
            .arg(arg)
            .assert()
            .success()
            .stdout(predicate::str::contains(expected));
    }
}

mod youtube_chat {
    use super::*;
    use tempfile::tempdir;
    use test_case::test_case;

    #[test_case("-h", "yt-tool youtube chat"; "when a flag is -h")]
    #[test_case("--help", "yt-tool youtube chat"; "when a flag is --help")]
    fn it_works(arg: &str, expected: &str) {
        subject()
            .arg("youtube")
            .arg("chat")
            .arg(arg)
            .assert()
            .success()
            .stdout(predicate::str::contains(expected));
    }

    #[test_case("", "live_chat.json"; "when a valid json file is provided")]
    #[test_case("-r", "live_chat.json"; "when a valid json file is provided with -r")]
    #[test_case("--rename-with-timestamp", "live_chat.json"; "when a valid json file is provided with --rename-with-timestamp")]
    fn it_works_with_json_file(arg: &str, input: &str) {
        let temp_dir = tempdir().unwrap();
        let temp_file = temp_dir.path().join(input);
        let temp_file_path = temp_file.to_str().unwrap();
        std::fs::copy("tests/live_chat.json", temp_file_path).unwrap();

        subject()
            .arg("youtube")
            .arg("chat")
            .arg(arg)
            .arg(temp_file_path)
            .assert()
            .success();
    }

    #[test_case("", "invalid.json"; "when a invalid json file is provided")]
    #[test_case("-r", "invalid.json"; "when a invalid json file is provided with -r")]
    #[test_case("--rename-with-timestamp", "invalid.json"; "when a invalid json file is provided with --rename-with-timestamp")]
    fn it_works_with_invalid_json_file(arg: &str, input: &str) {
        let temp_dir = tempdir().unwrap();
        let temp_file = temp_dir.path().join(input);
        let temp_file_path = temp_file.to_str().unwrap();

        std::fs::write(temp_file_path, "invalid json").unwrap();

        subject()
            .arg("youtube")
            .arg("chat")
            .arg(arg)
            .arg(temp_file_path)
            .assert()
            .failure()
            .stderr(predicate::str::contains("Error: Failed to convert at"));
    }

    #[test_case("", "***"; "when a invalid file name is provided")]
    #[test_case("-r", "***"; "when a invalid file name is provided with -r")]
    #[test_case("--rename-with-timestamp", "***"; "when a invalid file name is provided with --rename-with-timestamp")]
    fn it_works_with_invalid_file_name(arg: &str, input: &str) {
        subject()
            .arg("youtube")
            .arg("chat")
            .arg(arg)
            .arg(input)
            .assert()
            .failure()
            .stderr(predicate::str::contains("Error: invalid patterns"));
    }
}
