use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestEntry {
    pub hash: String,
    pub label: String,
}

pub fn load_manifest(path: &Path) -> Result<Vec<ManifestEntry>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut entries = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = trimmed.splitn(2, ',').collect();
        if parts.len() < 2 {
            continue;
        }
        let hash = parts[0].trim();
        let label = parts[1].trim();
        if hash.len() == 64 && hash.chars().all(|c| c.is_ascii_hexdigit()) {
            entries.push(ManifestEntry {
                hash: hash.to_lowercase(),
                label: label.to_string(),
            });
        }
    }
    Ok(entries)
}
