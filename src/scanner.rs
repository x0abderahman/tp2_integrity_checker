use crate::hashing::hash_file_sha256;
use crate::ioc::IocEntry;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScanStatus {
    Clean,
    Match(String),
    Error(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanResult {
    pub path: String,
    pub sha256: Option<String>,
    pub status: ScanStatus,
}

pub fn scan_target(target: &Path, iocs: &[IocEntry]) -> Vec<ScanResult> {
    let mut results = Vec::new();

    if target.is_file() {
        results.push(scan_single_file(target, iocs));
    } else if target.is_dir() {
        if let Ok(entries) = std::fs::read_dir(target) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    results.push(scan_single_file(&path, iocs));
                }
            }
        }
    }

    results
}

fn scan_single_file(path: &Path, iocs: &[IocEntry]) -> ScanResult {
    let path_str = path.to_string_lossy().to_string();

    match hash_file_sha256(path) {
        Ok(hash) => {
            let matching = iocs.iter().find(|entry| entry.hash == hash);
            match matching {
                Some(ioc) => ScanResult {
                    path: path_str,
                    sha256: Some(hash),
                    status: ScanStatus::Match(ioc.label.clone()),
                },
                None => ScanResult {
                    path: path_str,
                    sha256: Some(hash),
                    status: ScanStatus::Clean,
                },
            }
        }
        Err(e) => ScanResult {
            path: path_str,
            sha256: None,
            status: ScanStatus::Error(e.to_string()),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_single_clean_file() {
        use std::io::Write;
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        writeln!(tmp, "clean content").unwrap();

        let iocs = vec![IocEntry {
            hash: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            label: "test".to_string(),
        }];

        let results = scan_target(tmp.path(), &iocs);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].status, ScanStatus::Clean);
    }
}
