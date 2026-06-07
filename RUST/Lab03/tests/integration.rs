use std::path::Path;
use std::process::Command;

// ── Helpers ──────────────────────────────────────────────────────────────────

fn fixtures_dir() -> &'static Path {
    Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures"))
}

/// Run the binary with a fixture file as first arg, followed by extra args.
fn run_with(fixture: &str, args: &[&str]) -> std::process::Output {
    let path = fixtures_dir().join(fixture);
    let mut cmd_args: Vec<&str> = vec![path.to_str().unwrap()];
    cmd_args.extend_from_slice(args);
    Command::new(env!("CARGO_BIN_EXE_lab3"))
        .args(&cmd_args)
        .output()
        .expect("failed to execute binary")
}

/// Run the binary with raw args (no fixture prefix).
fn run_raw(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_lab3"))
        .args(args)
        .output()
        .expect("failed to execute binary")
}

/// Check that stdout contains an exact line (after trimming each line).
fn has_line(output: &std::process::Output, expected: &str) -> bool {
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.lines().any(|line| line.trim() == expected)
}

fn stdout(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn stderr(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stderr).to_string()
}

// ── Aggregation: count ──────────────────────────────────────────────────────

// valid_mixed.csv: Alice/30/9.5, Bob/20/6.5, Charlie/40/3.5, Diana/10/11.5

#[test]
fn count_succeeds() {
    let out = run_with("valid_mixed.csv", &["--mode", "count"]);
    assert!(out.status.success(), "expected success, stderr: {}", stderr(&out));
}

#[test]
fn count_result_is_correct() {
    let out = run_with("valid_mixed.csv", &["--mode", "count"]);
    assert!(
        has_line(&out, "result: 4"),
        "expected 'result: 4', got:\n{}", stdout(&out)
    );
}

#[test]
fn count_rows_analyzed_is_correct() {
    let out = run_with("valid_mixed.csv", &["--mode", "count"]);
    assert!(
        has_line(&out, "rows_analyzed: 4"),
        "expected 'rows_analyzed: 4', got:\n{}", stdout(&out)
    );
}

#[test]
fn count_output_contains_mode() {
    let out = run_with("valid_mixed.csv", &["--mode", "count"]);
    assert!(
        has_line(&out, "mode: count"),
        "expected 'mode: count', got:\n{}", stdout(&out)
    );
}

#[test]
fn sum_output_contains_mode() {
    let out = run_with("valid_mixed.csv", &["--mode", "sum", "--column", "age"]);
    assert!(
        has_line(&out, "mode: sum"),
        "expected 'mode: sum', got:\n{}", stdout(&out)
    );
}

#[test]
fn avg_output_contains_mode() {
    let out = run_with("valid_mixed.csv", &["--mode", "avg", "--column", "age"]);
    assert!(
        has_line(&out, "mode: avg"),
        "expected 'mode: avg', got:\n{}", stdout(&out)
    );
}

#[test]
fn min_output_contains_mode() {
    let out = run_with("valid_mixed.csv", &["--mode", "min", "--column", "age"]);
    assert!(
        has_line(&out, "mode: min"),
        "expected 'mode: min', got:\n{}", stdout(&out)
    );
}

#[test]
fn max_output_contains_mode() {
    let out = run_with("valid_mixed.csv", &["--mode", "max", "--column", "age"]);
    assert!(
        has_line(&out, "mode: max"),
        "expected 'mode: max', got:\n{}", stdout(&out)
    );
}

#[test]
fn count_output_has_no_column_line() {
    let out = run_with("valid_mixed.csv", &["--mode", "count"]);
    let s = stdout(&out);
    assert!(
        !s.lines().any(|l| l.trim().starts_with("column:")),
        "count without --column should not print a 'column:' line, got:\n{s}"
    );
}

// ── Aggregation: sum ────────────────────────────────────────────────────────

#[test]
fn sum_integer_column() {
    // age: 30 + 20 + 40 + 10 = 100
    let out = run_with("valid_mixed.csv", &["--mode", "sum", "--column", "age"]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(
        has_line(&out, "result: 100.0"),
        "expected 'result: 100.0', got:\n{}", stdout(&out)
    );
}

#[test]
fn sum_float_column() {
    // score: 9.5 + 6.5 + 3.5 + 11.5 = 31.0
    let out = run_with("valid_mixed.csv", &["--mode", "sum", "--column", "score"]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    let s = stdout(&out);
    // Result must be decimal with a point (the column is Float)
    assert!(
        has_line(&out, "result: 31.0"),
        "expected 'result: 31.0', got:\n{s}"
    );
}

#[test]
fn sum_output_contains_column() {
    let out = run_with("valid_mixed.csv", &["--mode", "sum", "--column", "age"]);
    assert!(
        has_line(&out, "column: age"),
        "expected 'column: age', got:\n{}", stdout(&out)
    );
}

// ── Aggregation: avg ────────────────────────────────────────────────────────

#[test]
fn avg_integer_column() {
    // age: 100 / 4 = 25.0  (avg is always decimal)
    let out = run_with("valid_mixed.csv", &["--mode", "avg", "--column", "age"]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(
        has_line(&out, "result: 25.0"),
        "expected 'result: 25.0', got:\n{}", stdout(&out)
    );
}

#[test]
fn avg_float_column() {
    // score: 31.0 / 4 = 7.75
    let out = run_with("valid_mixed.csv", &["--mode", "avg", "--column", "score"]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(
        has_line(&out, "result: 7.75"),
        "expected 'result: 7.75', got:\n{}", stdout(&out)
    );
}

// ── Aggregation: min / max ──────────────────────────────────────────────────

#[test]
fn min_integer_column() {
    // min age = 10
    let out = run_with("valid_mixed.csv", &["--mode", "min", "--column", "age"]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(
        has_line(&out, "result: 10.0"),
        "expected 'result: 10.0', got:\n{}", stdout(&out)
    );
}

#[test]
fn max_integer_column() {
    // max age = 40
    let out = run_with("valid_mixed.csv", &["--mode", "max", "--column", "age"]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(
        has_line(&out, "result: 40.0"),
        "expected 'result: 40.0', got:\n{}", stdout(&out)
    );
}

#[test]
fn min_float_column() {
    // min score = 3.5
    let out = run_with("valid_mixed.csv", &["--mode", "min", "--column", "score"]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(
        has_line(&out, "result: 3.5"),
        "expected 'result: 3.5', got:\n{}", stdout(&out)
    );
}

#[test]
fn max_float_column() {
    // max score = 11.5
    let out = run_with("valid_mixed.csv", &["--mode", "max", "--column", "score"]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(
        has_line(&out, "result: 11.5"),
        "expected 'result: 11.5', got:\n{}", stdout(&out)
    );
}

// ── Aggregation on integer-only CSV ─────────────────────────────────────────

// valid_integers.csv: x(10,20,30) y(100,200,300) z(1,2,3)

#[test]
fn integers_only_sum() {
    let out = run_with("valid_integers.csv", &["--mode", "sum", "--column", "x"]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(
        has_line(&out, "result: 60.0"),
        "expected 'result: 60.0', got:\n{}", stdout(&out)
    );
}

#[test]
fn integers_only_avg() {
    // avg y = 600 / 3 = 200.0 (avg is always decimal)
    let out = run_with("valid_integers.csv", &["--mode", "avg", "--column", "y"]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(
        has_line(&out, "result: 200.0"),
        "expected 'result: 200.0', got:\n{}", stdout(&out)
    );
}

// ── Filtering ───────────────────────────────────────────────────────────────

#[test]
fn count_with_filter_gt() {
    // age > 25: Alice(30) and Charlie(40) => 2
    let out = run_with("valid_mixed.csv", &["--mode", "count", "--filter", "age>25"]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(
        has_line(&out, "result: 2"),
        "expected 'result: 2', got:\n{}", stdout(&out)
    );
    assert!(
        has_line(&out, "rows_analyzed: 2"),
        "expected 'rows_analyzed: 2', got:\n{}", stdout(&out)
    );
}

#[test]
fn count_with_filter_lt() {
    // age < 15: Diana(10) => 1
    let out = run_with("valid_mixed.csv", &["--mode", "count", "--filter", "age<15"]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(
        has_line(&out, "result: 1"),
        "expected 'result: 1', got:\n{}", stdout(&out)
    );
}

#[test]
fn count_with_filter_eq_text() {
    // name = Alice => 1
    let out = run_with("valid_mixed.csv", &["--mode", "count", "--filter", "name=Alice"]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(
        has_line(&out, "result: 1"),
        "expected 'result: 1', got:\n{}", stdout(&out)
    );
}

#[test]
fn avg_with_filter() {
    // age > 25 => Alice(score=9.5), Charlie(score=3.5)
    // avg = 13.0 / 2 = 6.5
    let out = run_with(
        "valid_mixed.csv",
        &["--mode", "avg", "--column", "score", "--filter", "age>25"],
    );
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(
        has_line(&out, "result: 6.5"),
        "expected 'result: 6.5', got:\n{}", stdout(&out)
    );
    assert!(
        has_line(&out, "rows_analyzed: 2"),
        "expected 'rows_analyzed: 2', got:\n{}", stdout(&out)
    );
}

#[test]
fn sum_with_filter() {
    // age > 25 => Alice(age=30), Charlie(age=40)
    // sum = 70
    let out = run_with(
        "valid_mixed.csv",
        &["--mode", "sum", "--column", "age", "--filter", "age>25"],
    );
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(
        has_line(&out, "result: 70.0"),
        "expected 'result: 70.0', got:\n{}", stdout(&out)
    );
}

#[test]
fn filter_output_contains_filter_line() {
    let out = run_with("valid_mixed.csv", &["--mode", "count", "--filter", "age>25"]);
    assert!(
        has_line(&out, "filter: age>25"),
        "expected 'filter: age>25', got:\n{}", stdout(&out)
    );
}

#[test]
fn no_filter_output_has_no_filter_line() {
    let out = run_with("valid_mixed.csv", &["--mode", "count"]);
    let s = stdout(&out);
    assert!(
        !s.lines().any(|l| l.trim().starts_with("filter:")),
        "output without --filter should not print a 'filter:' line, got:\n{s}"
    );
}

// ── Edge cases: header only ─────────────────────────────────────────────────

#[test]
fn header_only_count() {
    let out = run_with("header_only.csv", &["--mode", "count"]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(
        has_line(&out, "result: 0"),
        "expected 'result: 0', got:\n{}", stdout(&out)
    );
    assert!(
        has_line(&out, "rows_analyzed: 0"),
        "expected 'rows_analyzed: 0', got:\n{}", stdout(&out)
    );
}

#[test]
fn header_only_avg_is_nan() {
    let out = run_with("header_only.csv", &["--mode", "avg", "--column", "score"]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(
        has_line(&out, "result: NaN"),
        "expected 'result: NaN' for avg with no rows, got:\n{}", stdout(&out)
    );
}

#[test]
fn header_only_min_is_nan() {
    let out = run_with("header_only.csv", &["--mode", "min", "--column", "age"]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(
        has_line(&out, "result: NaN"),
        "expected 'result: NaN' for min with no rows, got:\n{}", stdout(&out)
    );
}

#[test]
fn header_only_max_is_nan() {
    let out = run_with("header_only.csv", &["--mode", "max", "--column", "age"]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(
        has_line(&out, "result: NaN"),
        "expected 'result: NaN' for max with no rows, got:\n{}", stdout(&out)
    );
}

#[test]
fn header_only_sum_is_zero() {
    let out = run_with("header_only.csv", &["--mode", "sum", "--column", "age"]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(
        has_line(&out, "result: 0.0"),
        "expected 'result: 0.0' for sum with no rows, got:\n{}", stdout(&out)
    );
}

// ── Edge cases: single row ──────────────────────────────────────────────────

// single_row.csv: Alice, 25, 8.5

#[test]
fn single_row_count() {
    let out = run_with("single_row.csv", &["--mode", "count"]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(
        has_line(&out, "result: 1"),
        "expected 'result: 1', got:\n{}", stdout(&out)
    );
}

#[test]
fn single_row_sum() {
    let out = run_with("single_row.csv", &["--mode", "sum", "--column", "age"]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(
        has_line(&out, "result: 25.0"),
        "expected 'result: 25.0', got:\n{}", stdout(&out)
    );
}

#[test]
fn single_row_avg() {
    let out = run_with("single_row.csv", &["--mode", "avg", "--column", "score"]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(
        has_line(&out, "result: 8.5"),
        "expected 'result: 8.5', got:\n{}", stdout(&out)
    );
}

#[test]
fn single_row_min_equals_max() {
    let out_min = run_with("single_row.csv", &["--mode", "min", "--column", "age"]);
    let out_max = run_with("single_row.csv", &["--mode", "max", "--column", "age"]);
    assert!(out_min.status.success());
    assert!(out_max.status.success());
    assert!(
        has_line(&out_min, "result: 25.0"),
        "expected min 'result: 25.0', got:\n{}", stdout(&out_min)
    );
    assert!(
        has_line(&out_max, "result: 25.0"),
        "expected max 'result: 25.0', got:\n{}", stdout(&out_max)
    );
}

// ── Edge cases: filter matches nothing ──────────────────────────────────────

#[test]
fn filter_no_matches_count() {
    let out = run_with("valid_mixed.csv", &["--mode", "count", "--filter", "age>100"]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(
        has_line(&out, "result: 0"),
        "expected 'result: 0', got:\n{}", stdout(&out)
    );
    assert!(
        has_line(&out, "rows_analyzed: 0"),
        "expected 'rows_analyzed: 0', got:\n{}", stdout(&out)
    );
}

#[test]
fn filter_no_matches_avg_is_nan() {
    let out = run_with(
        "valid_mixed.csv",
        &["--mode", "avg", "--column", "score", "--filter", "age>100"],
    );
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(
        has_line(&out, "result: NaN"),
        "expected 'result: NaN', got:\n{}", stdout(&out)
    );
}

#[test]
fn filter_no_matches_sum_is_zero() {
    let out = run_with(
        "valid_mixed.csv",
        &["--mode", "sum", "--column", "age", "--filter", "age>100"],
    );
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(
        has_line(&out, "result: 0.0"),
        "expected 'result: 0.0', got:\n{}", stdout(&out)
    );
}

#[test]
fn filter_no_matches_min_is_nan() {
    let out = run_with(
        "valid_mixed.csv",
        &["--mode", "min", "--column", "age", "--filter", "age>100"],
    );
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(
        has_line(&out, "result: NaN"),
        "expected 'result: NaN', got:\n{}", stdout(&out)
    );
}

#[test]
fn filter_no_matches_max_is_nan() {
    let out = run_with(
        "valid_mixed.csv",
        &["--mode", "max", "--column", "age", "--filter", "age>100"],
    );
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(
        has_line(&out, "result: NaN"),
        "expected 'result: NaN', got:\n{}", stdout(&out)
    );
}

#[test]
fn count_with_filter_eq_integer() {
    // age = 30: Alice => 1
    let out = run_with("valid_mixed.csv", &["--mode", "count", "--filter", "age=30"]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(
        has_line(&out, "result: 1"),
        "expected 'result: 1', got:\n{}", stdout(&out)
    );
    assert!(
        has_line(&out, "rows_analyzed: 1"),
        "expected 'rows_analyzed: 1', got:\n{}", stdout(&out)
    );
}

// ── Error cases: missing / invalid arguments ────────────────────────────────

#[test]
fn no_arguments_fails() {
    let out = run_raw(&[]);
    assert!(
        !out.status.success(),
        "expected failure when no arguments provided"
    );
}

#[test]
fn file_not_found_fails() {
    let out = run_raw(&["nonexistent_file.csv", "--mode", "count"]);
    assert!(!out.status.success(), "expected failure for nonexistent file");
    assert!(!out.stderr.is_empty(), "expected error message on stderr");
}

#[test]
fn missing_mode_fails() {
    let out = run_with("valid_mixed.csv", &[]);
    assert!(
        !out.status.success(),
        "expected failure when --mode is missing"
    );
}

#[test]
fn invalid_mode_fails() {
    let out = run_with("valid_mixed.csv", &["--mode", "invalid"]);
    assert!(!out.status.success(), "expected failure for invalid mode");
    assert!(
        !out.stderr.is_empty(),
        "expected error message on stderr for invalid mode"
    );
}

// ── Error cases: missing --column ───────────────────────────────────────────

#[test]
fn sum_without_column_fails() {
    let out = run_with("valid_mixed.csv", &["--mode", "sum"]);
    assert!(
        !out.status.success(),
        "expected failure for sum without --column"
    );
}

#[test]
fn avg_without_column_fails() {
    let out = run_with("valid_mixed.csv", &["--mode", "avg"]);
    assert!(
        !out.status.success(),
        "expected failure for avg without --column"
    );
}

#[test]
fn min_without_column_fails() {
    let out = run_with("valid_mixed.csv", &["--mode", "min"]);
    assert!(
        !out.status.success(),
        "expected failure for min without --column"
    );
}

#[test]
fn max_without_column_fails() {
    let out = run_with("valid_mixed.csv", &["--mode", "max"]);
    assert!(
        !out.status.success(),
        "expected failure for max without --column"
    );
}

// ── Error cases: bad column ─────────────────────────────────────────────────

#[test]
fn column_not_found_fails() {
    let out = run_with(
        "valid_mixed.csv",
        &["--mode", "sum", "--column", "nonexistent"],
    );
    assert!(!out.status.success(), "expected failure for nonexistent column");
    assert!(!out.stderr.is_empty(), "expected error message on stderr");
}

#[test]
fn sum_on_text_column_uses_length() {
    let out = run_with("text_only.csv", &["--mode", "sum", "--column", "first"]);
    assert!(
        out.status.success(),
        "sum on text column should succeed (treating text length as value), got stderr:\n{}", stderr(&out)
    );
    assert!(
        has_line(&out, "result: 8.0"),
        "expected 'result: 8.0' for sum on text column (sum of lengths of 'Alice', 'Bob'), got:\n{}", stdout(&out)
    );
}

#[test]
fn avg_on_text_column_uses_length() {
    let out = run_with("text_only.csv", &["--mode", "avg", "--column", "city"]);
    assert!(
        out.status.success(),
        "avg on text column should succeed (treating text length as value), got stderr:\n{}", stderr(&out)
    );
    assert!(
        has_line(&out, "result: 5.0"),
        "expected 'result: 5.0' for avg on text column (average length of 'Turin', 'Milan'), got:\n{}", stdout(&out)
    );
}

#[test]
fn min_on_text_column_uses_length() {
    let out = run_with("text_only.csv", &["--mode", "min", "--column", "first"]);
    assert!(
        out.status.success(),
        "min on text column should succeed (treating text length as value), got stderr:\n{}", stderr(&out)
    );
    assert!(
        has_line(&out, "result: 3.0"),
        "expected 'result: 3.0' for min on text column (length of shortest name 'Bob'), got:\n{}", stdout(&out)
    );
}

#[test]
fn max_on_text_column_uses_length() {
    let out = run_with("text_only.csv", &["--mode", "max", "--column", "first"]);
    assert!(
        out.status.success(),
        "max on text column should succeed (treating text length as value), got stderr:\n{}", stderr(&out)
    );
    assert!(
        has_line(&out, "result: 5.0"),
        "expected 'result: 5.0' for max on text column (length of longest name 'Alice'), got:\n{}", stdout(&out)
    );
}

// ── Error cases: bad filter ─────────────────────────────────────────────────

#[test]
fn malformed_filter_fails() {
    let out = run_with(
        "valid_mixed.csv",
        &["--mode", "count", "--filter", "invalid"],
    );
    assert!(
        !out.status.success(),
        "expected failure for malformed filter expression"
    );
    assert!(!out.stderr.is_empty(), "expected error message on stderr");
}

#[test]
fn filter_column_not_found_fails() {
    let out = run_with(
        "valid_mixed.csv",
        &["--mode", "count", "--filter", "nonexistent>5"],
    );
    assert!(
        !out.status.success(),
        "expected failure when filter references nonexistent column"
    );
    assert!(!out.stderr.is_empty(), "expected error message on stderr");
}

// ── Error cases: empty file ─────────────────────────────────────────────────

#[test]
fn empty_file_does_not_panic() {
    let out = run_with("empty.csv", &["--mode", "count"]);
    let err = stderr(&out);
    assert!(
        !err.contains("panicked"),
        "program panicked on empty file: {err}"
    );
}
