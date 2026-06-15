use crate::hashing::hash_file_sha256;
use crate::ioc::IocEntry;
use crate::manifest::ManifestEntry;
use rayon::prelude::*;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScanStatus {
    Clean,
    KnownGood(String),
    Match(String),
    Error(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanResult {
    pub path: String,
    pub sha256: Option<String>,
    pub status: ScanStatus,
}

pub fn scan_target(
    target: &Path,
    iocs: &[IocEntry],
    good_manifest: &[ManifestEntry],
) -> Vec<ScanResult> {
    let mut files = Vec::new();

    if target.is_file() {
        files.push(target.to_path_buf());
    } else if target.is_dir() {
        for entry in WalkDir::new(target).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                files.push(entry.path().to_path_buf());
            }
        }
    }

    files
        .par_iter()
        .map(|path| scan_single_file(path, iocs, good_manifest))
        .collect()
}

fn scan_single_file(path: &Path, iocs: &[IocEntry], good_manifest: &[ManifestEntry]) -> ScanResult {
    let path_str = path.to_string_lossy().to_string();

    match hash_file_sha256(path) {
        Ok(hash) => {
            if let Some(good) = good_manifest.iter().find(|e| e.hash == hash) {
                return ScanResult {
                    path: path_str,
                    sha256: Some(hash),
                    status: ScanStatus::KnownGood(good.label.clone()),
                };
            }
            if let Some(ioc) = iocs.iter().find(|entry| entry.hash == hash) {
                ScanResult {
                    path: path_str,
                    sha256: Some(hash),
                    status: ScanStatus::Match(ioc.label.clone()),
                }
            } else {
                ScanResult {
                    path: path_str,
                    sha256: Some(hash),
                    status: ScanStatus::Clean,
                }
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

        let iocs = vec![];
        let manifest = vec![];
        let results = scan_target(tmp.path(), &iocs, &manifest);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].status, ScanStatus::Clean);
    }

    #[test]
    fn test_scan_known_good() {
        use std::io::Write;
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        writeln!(tmp, "known good content").unwrap();

        let path = tmp.path().to_path_buf();
        let hash = hash_file_sha256(&path).unwrap();

        let manifest = vec![ManifestEntry {
            hash: hash.clone(),
            label: "known_good.txt".to_string(),
        }];
        let results = scan_target(&path, &[], &manifest);
        assert_eq!(results.len(), 1);
        assert_eq!(
            results[0].status,
            ScanStatus::KnownGood("known_good.txt".to_string())
        );
    }

    #[test]
    fn test_scan_match() {
        use std::io::Write;
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        writeln!(tmp, "malicious content").unwrap();

        let path = tmp.path().to_path_buf();
        let hash = hash_file_sha256(&path).unwrap();
        let label = "malware".to_string();

        let iocs = vec![IocEntry {
            hash: hash.clone(),
            label: label.clone(),
        }];
        let results = scan_target(&path, &iocs, &[]);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].status, ScanStatus::Match(label));
    }

    #[test]
    fn test_recursive_directory_scan() {
        let dir = tempfile::tempdir().unwrap();
        let sub = dir.path().join("sub");
        std::fs::create_dir(&sub).unwrap();
        std::fs::write(dir.path().join("a.txt"), "a").unwrap();
        std::fs::write(sub.join("b.txt"), "b").unwrap();

        let results = scan_target(dir.path(), &[], &[]);
        assert_eq!(results.len(), 2);
    }
}
