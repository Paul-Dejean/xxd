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
    assert_eq!(lines[1], "00000004: 6f20 776f o wo");
    assert_eq!(lines[2], "00000008: 726c 64 rld");
}

#[test]
fn test_should_skip_bytes_with_positive_seek_option() {
    let mut tmp_file = NamedTempFile::new().expect("failed to create temp file");
    write!(tmp_file, "hello world").expect("failed to write to temp file");

    let mut cmd = Command::cargo_bin("cxxd").expect("binary exists");
    cmd.arg(tmp_file.path())
        .args(["-s", "2"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "00000002: 6c6c 6f20 776f 726c 64 llo world",
        ));
}

#[test]
fn test_should_skip_bytes_with_negative_seek_option() {
    let mut tmp_file = NamedTempFile::new().expect("failed to create temp file");
    write!(tmp_file, "hello world").expect("failed to write to temp file");

    let mut cmd = Command::cargo_bin("cxxd").expect("binary exists");
    cmd.arg(tmp_file.path())
        .args(["-s=-5"])
        .assert()
        .success()
        .stdout(predicate::str::contains("00000006: 776f 726c 64 world"));
}

#[test]
fn test_should_display_exact_content_with_seek_and_length_options() {
    let mut tmp_file = NamedTempFile::new().expect("failed to create temp file");
    write!(tmp_file, "hello world").expect("failed to write to temp file");

    let mut cmd = Command::cargo_bin("cxxd").expect("binary exists");
    cmd.arg(tmp_file.path())
        .args(["-s", "2", "-l", "5"])
        .assert()
        .success()
        .stdout(predicate::str::contains("00000002: 6c6c 6f20 77 llo w"));
}
