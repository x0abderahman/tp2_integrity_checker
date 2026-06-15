use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IocEntry {
    pub hash: String,
    pub label: String,
}

pub fn is_valid_sha256(value: &str) -> bool {
    if value.len() != 64 {
        return false;
    }
    value.chars().all(|c| c.is_ascii_hexdigit())
}

pub fn load_iocs(path: &Path) -> Result<(Vec<IocEntry>, usize), std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut entries = Vec::new();
    let mut invalid_count = 0usize;

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = trimmed.splitn(2, ',').collect();
        if parts.len() < 2 {
            invalid_count += 1;
            continue;
        }
        let hash = parts[0].trim();
        let label = parts[1].trim();
        if !is_valid_sha256(hash) {
            invalid_count += 1;
            continue;
        }
        entries.push(IocEntry {
            hash: hash.to_lowercase(),
            label: label.to_string(),
        });
    }
    Ok((entries, invalid_count))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_sha256_valid() {
        assert!(is_valid_sha256(
            "44ea92bec1f9e8aa690d8aceddf1294e9fb4a71d39769d6f383e3915ac76bb3b"
        ));
    }

    #[test]
    fn test_is_valid_sha256_invalid_short() {
        assert!(!is_valid_sha256("abc123"));
    }

    #[test]
    fn test_is_valid_sha256_invalid_chars() {
        assert!(!is_valid_sha256(
            "44ea92bec1f9e8aa690d8aceddf1294e9fb4a71d39769d6f383e3915ac76bb3z"
        ));
    }

    #[test]
    fn test_load_iocs_with_comments_and_invalid() {
        use std::io::Write;
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        writeln!(tmp, "# hash,label").unwrap();
        writeln!(
            tmp,
            "44ea92bec1f9e8aa690d8aceddf1294e9fb4a71d39769d6f383e3915ac76bb3b,Demo suspicious test sample"
        )
        .unwrap();
        writeln!(
            tmp,
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,Fake known malware hash"
        )
        .unwrap();
        writeln!(tmp, "INVALID_HASH_LINE_SHOULD_BE_IGNORED").unwrap();
        writeln!(tmp, "").unwrap();

        let (entries, invalid) = load_iocs(tmp.path()).unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(invalid, 1);
    }
}
