use std::path::Path;
use std::process::Command;

fn fixtures_dir() -> &'static Path {
    Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures"))
}

fn run_with(fixture: &str) -> std::process::Output {
    let path = fixtures_dir().join(fixture);
    Command::new(env!("CARGO_BIN_EXE_lab2"))
        .arg(path)
        .output()
        .expect("failed to execute binary")
}

#[test]
fn valid_integers_succeeds() {
    let out = run_with("valid_integers.csv");
    assert!(
        out.status.success(),
        "expected success for valid_integers.csv"
    );
    assert!(
        !out.stdout.is_empty(),
        "expected non-empty stdout for valid file"
    );
}

#[test]
fn valid_mixed_succeeds() {
    let out = run_with("valid_mixed.csv");
    assert!(out.status.success(), "expected success for valid_mixed.csv");
    assert!(!out.stdout.is_empty());
}

#[test]
fn valid_integers_output_contains_values() {
    let out = run_with("valid_integers.csv");
    let stdout = String::from_utf8_lossy(&out.stdout);
    // The output format is free, but the values must appear somewhere
    for val in ["1", "2", "3", "4", "5", "6", "7", "8", "9"] {
        assert!(
            stdout.contains(val),
            "output should contain '{val}', got:\n{stdout}"
        );
    }
}

#[test]
fn valid_mixed_output_contains_values() {
    let out = run_with("valid_mixed.csv");
    let stdout = String::from_utf8_lossy(&out.stdout);
    for val in ["Alice", "Bob", "Charlie", "25", "30", "22"] {
        assert!(
            stdout.contains(val),
            "output should contain '{val}', got:\n{stdout}"
        );
    }
}

#[test]
fn valid_mixed_output_contains_headers() {
    let out = run_with("valid_mixed.csv");
    let stdout = String::from_utf8_lossy(&out.stdout);
    for hdr in ["name", "age", "score"] {
        assert!(
            stdout.contains(hdr),
            "output should contain header '{hdr}', got:\n{stdout}"
        );
    }
}

#[test]
fn single_row_succeeds() {
    let out = run_with("single_row.csv");
    assert!(out.status.success(), "expected success for single_row.csv");
    assert!(!out.stdout.is_empty());
}

#[test]
fn header_only_succeeds() {
    let out = run_with("header_only.csv");
    assert!(out.status.success(), "expected success for header_only.csv");
}

#[test]
fn too_many_fields_fails() {
    let out = run_with("too_many_fields.csv");
    assert!(
        !out.status.success(),
        "expected failure for too_many_fields.csv"
    );
    assert!(!out.stderr.is_empty(), "expected error message on stderr");
}

#[test]
fn too_few_fields_fails() {
    let out = run_with("too_few_fields.csv");
    assert!(
        !out.status.success(),
        "expected failure for too_few_fields.csv"
    );
    assert!(!out.stderr.is_empty(), "expected error message on stderr");
}

#[test]
fn type_mismatch_fails() {
    let out = run_with("type_mismatch.csv");
    assert!(
        !out.status.success(),
        "expected failure for type_mismatch.csv: column 'age' has Integer in row 1 but Text in row 2"
    );
    assert!(!out.stderr.is_empty(), "expected error message on stderr");
}

#[test]
fn file_not_found_fails() {
    let out = Command::new(env!("CARGO_BIN_EXE_lab2"))
        .arg("nonexistent_file_that_does_not_exist.csv")
        .output()
        .expect("failed to execute binary");
    assert!(
        !out.status.success(),
        "expected failure for nonexistent file"
    );
    assert!(!out.stderr.is_empty(), "expected error message on stderr");
}

#[test]
fn empty_file_does_not_panic() {
    let out = run_with("empty.csv");
    // Whether empty is valid or an error is up to the student,
    // but it must not panic (exit code should be 0 or a clean non-zero)
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !stderr.contains("panicked"),
        "program panicked on empty file: {stderr}"
    );
}

#[test]
fn no_arguments_fails() {
    let out = Command::new(env!("CARGO_BIN_EXE_lab2"))
        .output()
        .expect("failed to execute binary");
    assert!(
        !out.status.success(),
        "expected failure when no arguments provided"
    );
}

#[test]
fn float_is_printed_with_decimal_point() {
    let out = run_with("valid_mixed.csv");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("9.0"),
        "output should contain '9.0', got:\n{stdout}"
    );
}

#[test]
fn multiple_errors_are_reported() {
    let out = run_with("multiple_errors.csv");
    assert!(
        !out.status.success(),
        "expected failure for multiple_errors.csv"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("2"),
        "expected line number 2 reported in error message, got:\n{stderr}"
    );
    assert!(
        stderr.contains("5"),
        "expected line number 5 reported in error message, got:\n{stderr}"
    );
}