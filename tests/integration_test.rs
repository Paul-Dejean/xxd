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
