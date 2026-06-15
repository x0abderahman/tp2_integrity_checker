# TP2 – File Integrity Checker and IOC Matcher in Rust

A Rust command-line tool that scans files/directories, calculates SHA-256 hashes, compares them against a list of IOCs (Indicators of Compromise), and generates a CSV report.

## Usage

```bash
cargo run -- --target <FILE_OR_DIRECTORY> --ioc <IOC_FILE> --report <REPORT_FILE>
```

## Features

- SHA-256 hashing of files
- IOC file parsing with comment/invalid line handling
- Directory scanning
- CSV report generation
- Comprehensive error handling
