use assert_cmd::Command;

#[test]
fn success() {
    let mut cmd = Command::cargo_bin("sire").unwrap();
    cmd.assert().success();
}