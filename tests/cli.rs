use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello_world").unwrap();
    cmd.assert().success();
}
