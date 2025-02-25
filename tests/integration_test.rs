use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_should_print_hex_dump_for_a_file() {
    let mut tmp_file = NamedTempFile::new().expect("failed to create temp file");
    write!(tmp_file, "hello world").expect("failed to write to temp file");

    let mut cmd = Command::cargo_bin("cxxd").expect("binary exists");
    cmd.arg(tmp_file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "00000000: 6865 6c6c 6f20 776f 726c 64 hello world",
        ));
}

#[test]
fn test_should_print_little_endian_hex_dump_for_a_file() {
    let mut tmp_file = NamedTempFile::new().expect("failed to create temp file");
    write!(tmp_file, "hello world").expect("failed to write to temp file");

    let mut cmd = Command::cargo_bin("cxxd").expect("binary exists");
    cmd.arg(tmp_file.path())
        .arg("-e")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "00000000: 6c6c6568 6f77206f 646c72 hello world",
        ));
}

#[test]
fn test_should_be_able_to_change_group_size() {
    let mut tmp_file = NamedTempFile::new().expect("failed to create temp file");
    write!(tmp_file, "hello world").expect("failed to write to temp file");

    let mut cmd = Command::cargo_bin("cxxd").expect("binary exists");
    cmd.arg(tmp_file.path())
        .args(["-g", "3"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "00000000: 68656c 6c6f20 776f72 6c64 hello world",
        ));
}

#[test]
fn test_should_be_able_to_change_group_size_in_little_endian() {
    let mut tmp_file = NamedTempFile::new().expect("failed to create temp file");
    write!(tmp_file, "hello world").expect("failed to write to temp file");

    let mut cmd = Command::cargo_bin("cxxd").expect("binary exists");
    cmd.arg(tmp_file.path())
        .args(["-e", "-g", "6"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "00000000: 206f6c6c6568 646c726f77 hello world",
        ));
}

#[test]
fn test_should_limit_output_length_with_len_option() {
    let mut tmp_file = NamedTempFile::new().expect("failed to create temp file");
    write!(tmp_file, "hello world").expect("failed to write to temp file");

    let mut cmd = Command::cargo_bin("cxxd").expect("binary exists");
    cmd.arg(tmp_file.path())
        .args(["-l", "5"])
        .assert()
        .success()
        .stdout(predicate::str::contains("00000000: 6865 6c6c 6f hello"));
}

#[test]
fn test_should_split_output_based_on_cols_option() {
    let mut tmp_file = NamedTempFile::new().expect("failed to create temp file");
    write!(tmp_file, "hello world").expect("failed to write to temp file");

    let mut cmd = Command::cargo_bin("cxxd").expect("binary exists");
    let output = cmd
        .arg(tmp_file.path())
        .args(["-c", "4"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let output_str = String::from_utf8_lossy(&output);

    let lines: Vec<&str> = output_str.lines().collect();

    assert_eq!(lines[0], "00000000: 6865 6c6c hell");
    assert_eq!(lines[1], "00000001: 6f20 776f o wo");
    assert_eq!(lines[2], "00000002: 726c 64 rld");
}
