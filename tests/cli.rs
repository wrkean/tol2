use assert_cmd::cargo::*;

#[test]
fn shows_version() {
    let version = tol2::VERSION;
    let mut cmd = cargo_bin_cmd!();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains(version));
}

#[test]
fn shows_help() {
    let about = tol2::ABOUT;
    let mut cmd = cargo_bin_cmd!();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains(about));
}
