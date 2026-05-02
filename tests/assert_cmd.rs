use assert_cmd::Command;

#[test]
fn run_once_executes_command_and_reports_success() {
    let assert = Command::cargo_bin("shell_command_menu")
        .expect("binary should build")
        .arg("--run-once")
        .arg("printf assert_cmd_ok")
        .assert()
        .success();

    let stdout = String::from_utf8(assert.get_output().stdout.clone()).expect("stdout is UTF-8");
    assert!(stdout.contains("assert_cmd_ok"));
    assert!(stdout.contains("Command executed successfully."));
}
