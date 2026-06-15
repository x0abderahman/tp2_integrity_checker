use crate::scanner::{ScanResult, ScanStatus};
use serde::Serialize;
use std::path::Path;

pub fn write_csv_report(results: &[ScanResult], report_path: &Path) -> Result<(), std::io::Error> {
    if let Some(parent) = report_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut wtr = csv::Writer::from_path(report_path)?;
    wtr.write_record(["path", "sha256", "status", "label"])?;

    for result in results {
        let sha256 = result.sha256.as_deref().unwrap_or("");
        let (status, label) = status_parts(&result.status);
        wtr.write_record([&result.path, sha256, status, &label])?;
    }

    wtr.flush()?;
    Ok(())
}

#[derive(Serialize)]
struct JsonReport {
    scan_timestamp: String,
    summary: JsonSummary,
    files: Vec<JsonFile>,
}

#[derive(Serialize)]
struct JsonSummary {
    total: usize,
    clean: usize,
    known_good: usize,
    matches: usize,
    errors: usize,
}

#[derive(Serialize)]
struct JsonFile {
    path: String,
    sha256: Option<String>,
    status: String,
    label: String,
}

pub fn write_json_report(results: &[ScanResult], report_path: &Path) -> Result<(), std::io::Error> {
    if let Some(parent) = report_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let (total, clean, known_good, matches, errors) = counts(results);

    let files: Vec<JsonFile> = results
        .iter()
        .map(|r| {
            let (status, label) = status_parts(&r.status);
            JsonFile {
                path: r.path.clone(),
                sha256: r.sha256.clone(),
                status: status.to_string(),
                label,
            }
        })
        .collect();

    let report = JsonReport {
        scan_timestamp: chrono_now(),
        summary: JsonSummary {
            total,
            clean,
            known_good,
            matches,
            errors,
        },
        files,
    };

    let file = std::fs::File::create(report_path)?;
    let writer = std::io::BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &report)?;
    Ok(())
}

fn chrono_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let dur = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}", dur.as_secs())
}

fn status_parts(status: &ScanStatus) -> (&str, String) {
    match status {
        ScanStatus::Clean => ("CLEAN", String::new()),
        ScanStatus::KnownGood(lbl) => ("KNOWN_GOOD", lbl.clone()),
        ScanStatus::Match(lbl) => ("MATCH", lbl.clone()),
        ScanStatus::Error(e) => ("ERROR", e.clone()),
    }
}

fn counts(results: &[ScanResult]) -> (usize, usize, usize, usize, usize) {
    let total = results.len();
    let mut clean = 0;
    let mut known_good = 0;
    let mut matches = 0;
    let mut errors = 0;
    for r in results {
        match r.status {
            ScanStatus::Clean => clean += 1,
            ScanStatus::KnownGood(_) => known_good += 1,
            ScanStatus::Match(_) => matches += 1,
            ScanStatus::Error(_) => errors += 1,
        }
    }
    (total, clean, known_good, matches, errors)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_report_generation() {
        use std::io::Read;
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().to_path_buf();

        let results = vec![
            ScanResult {
                path: "clean.txt".to_string(),
                sha256: Some(
                    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
                ),
                status: ScanStatus::Clean,
            },
            ScanResult {
                path: "malicious.txt".to_string(),
                sha256: Some(
                    "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".to_string(),
                ),
                status: ScanStatus::Match("malware".to_string()),
            },
        ];

        write_csv_report(&results, &path).unwrap();

        let mut content = String::new();
        std::fs::File::open(&path)
            .unwrap()
            .read_to_string(&mut content)
            .unwrap();

        assert!(content.contains("path,sha256,status,label"));
        assert!(content.contains("CLEAN"));
        assert!(content.contains("MATCH"));
    }

    #[test]
    fn test_json_report_generation() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("report.json");

        let results = vec![ScanResult {
            path: "test.txt".to_string(),
            sha256: Some(
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
            ),
            status: ScanStatus::Clean,
        }];

        write_json_report(&results, &path).unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("test.txt"));
        assert!(content.contains("CLEAN"));
    }
}
