use std::process::Command;

const BINARY: &str = "target/debug/tp2_integrity_checker";

#[test]
fn test_full_scan_csv_report() {
    let output = Command::new(BINARY)
        .args([
            "--target",
            "samples/files",
            "--ioc",
            "samples/iocs.txt",
            "--report",
            "/tmp/test_scan_report.csv",
        ])
        .output()
        .expect("failed to run binary");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Files scanned: 3"));
    assert!(stdout.contains("IOC entries loaded: 2"));
    assert!(stdout.contains("Invalid IOC lines: 1"));
    assert!(stdout.contains("Matches found: 1"));

    let report = std::fs::read_to_string("/tmp/test_scan_report.csv").unwrap();
    assert!(report.contains("CLEAN"));
    assert!(report.contains("MATCH"));
}

#[test]
fn test_scan_missing_target() {
    let output = Command::new(BINARY)
        .args([
            "--target",
            "nonexistent",
            "--ioc",
            "samples/iocs.txt",
            "--report",
            "/tmp/test_missing.csv",
        ])
        .output()
        .expect("failed to run binary");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("does not exist"));
}

#[test]
fn test_scan_json_output() {
    let json_path = "/tmp/test_scan_report.json";
    let output = Command::new(BINARY)
        .args([
            "--target",
            "samples/files",
            "--ioc",
            "samples/iocs.txt",
            "--report",
            "/tmp/test_json_report.csv",
            "--json",
            json_path,
        ])
        .output()
        .expect("failed to run binary");

    assert!(output.status.success());
    let content = std::fs::read_to_string(json_path).unwrap();
    assert!(content.contains("samples/files"));
}

#[test]
fn test_scan_only_matches() {
    let output = Command::new(BINARY)
        .args([
            "--target",
            "samples/files",
            "--ioc",
            "samples/iocs.txt",
            "--report",
            "/tmp/test_only_matches.csv",
            "--only-matches",
        ])
        .output()
        .expect("failed to run binary");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Files scanned: 1"));
    assert!(stdout.contains("Matches found: 1"));
}
