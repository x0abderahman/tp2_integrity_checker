use crate::scanner::{ScanResult, ScanStatus};
use std::path::Path;

pub fn write_csv_report(results: &[ScanResult], report_path: &Path) -> Result<(), std::io::Error> {
    if let Some(parent) = report_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut wtr = csv::Writer::from_path(report_path)?;

    wtr.write_record(["path", "sha256", "status", "label"])?;

    for result in results {
        let sha256 = result.sha256.as_deref().unwrap_or("");
        let (status, label) = match &result.status {
            ScanStatus::Clean => ("CLEAN", String::new()),
            ScanStatus::Match(lbl) => ("MATCH", lbl.clone()),
            ScanStatus::Error(e) => ("ERROR", e.clone()),
        };
        wtr.write_record([&result.path, sha256, status, &label])?;
    }

    wtr.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::ScanStatus;

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
        assert!(content.contains("clean.txt"));
        assert!(content.contains("CLEAN"));
        assert!(content.contains("malicious.txt"));
        assert!(content.contains("MATCH"));
        assert!(content.contains("malware"));
    }
}
