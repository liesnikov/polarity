use assert_cmd::Command;

/// The name of the CLI binary
const BINARY: &str = "pol";

/// Check that "pol --version" works correctly
#[test]
fn version_command() {
    println!("{BINARY:?}");
    let mut cmd = Command::cargo_bin(BINARY).unwrap();
    let assert = cmd.arg("--version").assert();
    assert.success().stdout("polarity 0.1.0\n");
}

/// Check that "pol check" works correctly
#[test]
fn check_command() {
    let mut cmd = Command::cargo_bin(BINARY).unwrap();
    let assert = cmd.args(vec!["check", "../examples/encoding_scott.pol"]).assert();
    assert.success().stdout("../examples/encoding_scott.pol typechecked successfully!\n");
}

/// Check that "pol check" works correctly
#[test]
fn check_command_2() {
    let mut cmd = Command::cargo_bin(BINARY).unwrap();
    let assert = cmd.args(vec!["check", "../examples/encoding_church.pol"]).assert();
    assert.success().stdout("../examples/encoding_church.pol typechecked successfully!\n");
}

/// Check that "pol run" works correctly
#[test]
fn run_command() {
    let mut cmd = Command::cargo_bin(BINARY).unwrap();
    let assert =
        cmd.env("NO_COLOR", "1").args(vec!["run", "../test/suites/success/037-vect.pol"]).assert();
    assert
        .success()
        .stdout("Cons(S(S(S(Z))), Z, Cons(S(S(Z)), Z, Cons(S(Z), Z, Cons(Z, Z, Nil))))\n");
}
