use std::fs;
use std::path::PathBuf;
use std::process::Command;

// ─── Helpers ───────────────────────────────────────────────────────────────────

fn get_bin_path() -> PathBuf {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.pop();
    path.push("lab4.exe");
    if !path.exists() {
        path.set_extension("");
    }
    path
}

fn run_with(fixture: &str, args: &[&str]) -> std::process::Output {
    let bin = get_bin_path();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("fixtures");
    path.push(fixture);

    let mut cmd_args: Vec<&str> = vec![path.to_str().unwrap()];
    cmd_args.extend_from_slice(args);
    Command::new(bin)
        .args(&cmd_args)
        .output()
        .expect("failed to execute binary")
}

fn stderr(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stderr).to_string()
}

fn has_line(output: &std::process::Output, expected: &str) -> bool {
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.lines().any(|line| line.trim() == expected)
}

// ─── Basic Count ────────────────────────────────────────────────────────────────

#[test]
fn count_succeeds() {
    let out = run_with("valid_mixed.csv", &["--mode", "count"]);
    assert!(
        out.status.success(),
        "expected success, stderr: {}",
        stderr(&out)
    );
}

#[test]
fn count_result_is_correct() {
    let out = run_with("valid_mixed.csv", &["--mode", "count"]);
    assert!(has_line(&out, "result: 4"));
}

#[test]
fn count_rows_analyzed_is_correct() {
    let out = run_with("valid_mixed.csv", &["--mode", "count"]);
    assert!(has_line(&out, "rows_analyzed: 4"));
}

#[test]
fn count_output_contains_mode() {
    let out = run_with("valid_mixed.csv", &["--mode", "count"]);
    assert!(has_line(&out, "mode: count"));
}

// ─── Basic Sum ──────────────────────────────────────────────────────────────────

#[test]
fn sum_succeeds() {
    let out = run_with("valid_mixed.csv", &["--mode", "sum", "--column", "age"]);
    assert!(out.status.success());
}

#[test]
fn sum_result_correct_int() {
    let out = run_with("valid_mixed.csv", &["--mode", "sum", "--column", "age"]);
    assert!(has_line(&out, "result: 100.0") || has_line(&out, "result: 100"));
}

#[test]
fn sum_result_correct_float() {
    let out = run_with("valid_mixed.csv", &["--mode", "sum", "--column", "score"]);
    assert!(has_line(&out, "result: 31.0") || has_line(&out, "result: 31"));
}

#[test]
fn sum_output_contains_mode() {
    let out = run_with("valid_mixed.csv", &["--mode", "sum", "--column", "age"]);
    assert!(has_line(&out, "mode: sum"));
}

#[test]
fn sum_output_contains_column() {
    let out = run_with("valid_mixed.csv", &["--mode", "sum", "--column", "age"]);
    assert!(has_line(&out, "column: age"));
}

// ─── Basic Avg ──────────────────────────────────────────────────────────────────

#[test]
fn avg_succeeds() {
    let out = run_with("valid_mixed.csv", &["--mode", "avg", "--column", "age"]);
    assert!(out.status.success());
}

#[test]
fn avg_result_correct() {
    let out = run_with("valid_mixed.csv", &["--mode", "avg", "--column", "age"]);
    assert!(has_line(&out, "result: 25.0") || has_line(&out, "result: 25"));
}

#[test]
fn avg_result_correct_float() {
    let out = run_with("valid_mixed.csv", &["--mode", "avg", "--column", "score"]);
    assert!(has_line(&out, "result: 7.75"));
}

// ─── Basic Min/Max ──────────────────────────────────────────────────────────────

#[test]
fn min_succeeds() {
    let out = run_with("valid_mixed.csv", &["--mode", "min", "--column", "age"]);
    assert!(out.status.success());
}

#[test]
fn min_result_correct() {
    let out = run_with("valid_mixed.csv", &["--mode", "min", "--column", "age"]);
    assert!(has_line(&out, "result: 10.0") || has_line(&out, "result: 10"));
}

#[test]
fn max_succeeds() {
    let out = run_with("valid_mixed.csv", &["--mode", "max", "--column", "age"]);
    assert!(out.status.success());
}

#[test]
fn max_result_correct() {
    let out = run_with("valid_mixed.csv", &["--mode", "max", "--column", "score"]);
    assert!(has_line(&out, "result: 11.5"));
}

// ─── Filter ─────────────────────────────────────────────────────────────────────

#[test]
fn filter_equals_works() {
    let out = run_with(
        "valid_mixed.csv",
        &["--mode", "count", "--filter", "name=Bob"],
    );
    assert!(has_line(&out, "result: 1"));
    assert!(has_line(&out, "rows_analyzed: 1"));
}

#[test]
fn filter_greater_works() {
    let out = run_with(
        "valid_mixed.csv",
        &["--mode", "count", "--filter", "age>25"],
    );
    assert!(has_line(&out, "result: 2")); // Alice and Charlie
}

#[test]
fn filter_less_works() {
    let out = run_with(
        "valid_mixed.csv",
        &["--mode", "avg", "--column", "score", "--filter", "age<30"],
    );
    // Bob (20, 6.5), Diana (10, 11.5) => Avg = 9.0
    assert!(has_line(&out, "result: 9.0") || has_line(&out, "result: 9"));
}

#[test]
fn filter_missing_column() {
    let out = run_with(
        "valid_mixed.csv",
        &["--mode", "count", "--filter", "missing=10"],
    );
    assert!(!out.status.success());
    assert!(stderr(&out).contains("not found"));
}

#[test]
fn filter_invalid_format() {
    let out = run_with(
        "valid_mixed.csv",
        &["--mode", "count", "--filter", "name~Bob"],
    );
    assert!(!out.status.success());
}

// ─── Empty & Header-only ────────────────────────────────────────────────────────

#[test]
fn empty_file_fails() {
    let out = run_with("empty.csv", &["--mode", "count"]);
    assert!(!out.status.success());
}

#[test]
fn header_only_count() {
    let out = run_with("header_only.csv", &["--mode", "count"]);
    assert!(out.status.success());
    assert!(has_line(&out, "result: 0"));
}

#[test]
fn header_only_avg_is_nan() {
    let out = run_with("header_only.csv", &["--mode", "avg", "--column", "age"]);
    assert!(out.status.success());
    assert!(has_line(&out, "result: NaN"));
}

#[test]
fn filter_all_out_avg_is_nan() {
    let out = run_with(
        "valid_mixed.csv",
        &["--mode", "avg", "--column", "age", "--filter", "age>100"],
    );
    assert!(out.status.success());
    assert!(has_line(&out, "result: NaN"));
}

// ─── Missing Columns ────────────────────────────────────────────────────────────

#[test]
fn sum_requires_column() {
    let bin = get_bin_path();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("fixtures");
    path.push("valid_mixed.csv");
    let out = Command::new(bin)
        .args(&[path.to_str().unwrap(), "--mode", "sum"])
        .output()
        .expect("failed to execute binary");
    assert!(!out.status.success());
}

#[test]
fn missing_column_errors() {
    let out = run_with(
        "valid_mixed.csv",
        &["--mode", "sum", "--column", "invalid_col"],
    );
    assert!(!out.status.success());
    assert!(stderr(&out).contains("Column not found"));
}

// ─── Group-By ───────────────────────────────────────────────────────────────────

#[test]
fn group_by_works_with_avg() {
    let out = run_with(
        "studenti.csv",
        &["--mode", "avg", "--column", "voto", "--group-by", "corso"],
    );
    assert!(out.status.success());
    assert!(has_line(&out, "analisi: 24.0") || has_line(&out, "analisi: 24"));
    assert!(has_line(&out, "pds: 28.0") || has_line(&out, "pds: 28"));
    assert!(has_line(&out, "PDS: 25.0") || has_line(&out, "PDS: 25"));
}

#[test]
fn group_by_works_with_count() {
    let out = run_with("studenti.csv", &["--mode", "count", "--group-by", "corso"]);
    assert!(out.status.success());
    assert!(has_line(&out, "analisi: 1"));
    assert!(has_line(&out, "pds: 1"));
    assert!(has_line(&out, "PDS: 1"));
}

#[test]
fn group_by_missing_column() {
    let out = run_with(
        "studenti.csv",
        &["--mode", "count", "--group-by", "notexist"],
    );
    assert!(!out.status.success());
    assert!(stderr(&out).contains("not found"));
}

// ─── Transform ──────────────────────────────────────────────────────────────────

#[test]
fn transform_uppercase_works() {
    let out = run_with(
        "studenti.csv",
        &[
            "--transform",
            "corso=uppercase",
            "--group-by",
            "corso",
            "--mode",
            "avg",
            "--column",
            "voto",
        ],
    );
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(has_line(&out, "transform: corso=uppercase"));
    assert!(has_line(&out, "ANALISI: 24.0") || has_line(&out, "ANALISI: 24"));
    assert!(has_line(&out, "PDS: 26.5"));
}

#[test]
fn transform_lowercase_works() {
    let out = run_with(
        "studenti.csv",
        &[
            "--transform",
            "corso=lowercase",
            "--group-by",
            "corso",
            "--mode",
            "avg",
            "--column",
            "voto",
        ],
    );
    assert!(out.status.success());
    assert!(has_line(&out, "analisi: 24.0") || has_line(&out, "analisi: 24"));
    assert!(has_line(&out, "pds: 26.5"));
}

#[test]
fn transform_invalid_op() {
    let out = run_with(
        "studenti.csv",
        &["--transform", "corso=reverse", "--mode", "count"],
    );
    assert!(!out.status.success());
    assert!(stderr(&out).contains("Unknown transform operation"));
}

#[test]
fn transform_invalid_format() {
    let out = run_with(
        "studenti.csv",
        &["--transform", "corsouppercase", "--mode", "count"],
    );
    assert!(!out.status.success());
}

#[test]
fn transform_on_numeric_column_fails() {
    let out = run_with(
        "studenti.csv",
        &["--transform", "voto=uppercase", "--mode", "count"],
    );
    assert!(!out.status.success());
    assert!(stderr(&out).contains("only be applied to Text columns"));
}

// ─── Export / Import ────────────────────────────────────────────────────────────

#[test]
fn export_mutually_exclusive_with_mode() {
    let out = run_with(
        "studenti.csv",
        &["--export", "dummy.json", "--mode", "count"],
    );
    assert!(!out.status.success());
}

#[test]
fn export_mutually_exclusive_with_groupby() {
    let out = run_with(
        "studenti.csv",
        &["--export", "dummy.json", "--group-by", "corso"],
    );
    assert!(!out.status.success());
}

#[test]
fn export_and_import_roundtrip() {
    let mut export_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    export_path.push("tests");
    export_path.push("fixtures");
    export_path.push("test_roundtrip.json");
    let export_str = export_path.to_str().unwrap();

    let _ = fs::remove_file(&export_path);

    let out1 = run_with("valid_mixed.csv", &["--export", export_str]);
    assert!(out1.status.success(), "stderr: {}", stderr(&out1));
    assert!(export_path.exists());

    // Run import using the generated json
    let bin = get_bin_path();
    let out2 = Command::new(bin)
        .args(&[export_str, "--mode", "avg", "--column", "score"])
        .output()
        .expect("failed to execute binary");

    assert!(out2.status.success());
    assert!(has_line(&out2, "result: 7.75"));

    // Cleanup
    let _ = fs::remove_file(&export_path);
}

#[test]
fn export_with_transform() {
    let mut export_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    export_path.push("tests");
    export_path.push("fixtures");
    export_path.push("test_transform.json");
    let export_str = export_path.to_str().unwrap();

    let _ = fs::remove_file(&export_path);

    let out1 = run_with(
        "studenti.csv",
        &["--transform", "corso=uppercase", "--export", export_str],
    );
    assert!(out1.status.success());
    assert!(export_path.exists());

    // The JSON should contain PDS instead of pds
    let content = fs::read_to_string(&export_path).unwrap();
    assert!(content.contains("\"PDS\""));
    assert!(!content.contains("\"pds\""));

    let _ = fs::remove_file(&export_path);
}

#[test]
fn invalid_operand_column_combination() {
    let out = run_with(
        "valid_mixed.csv",
        &["--mode", "avg", "--column", "score", "--filter", "name>4"],
    );
    assert!(!out.status.success());
    assert!(stderr(&out).contains("incompatible types"));
}
