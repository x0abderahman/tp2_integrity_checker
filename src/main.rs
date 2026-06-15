mod hashing;
mod ioc;
mod report;
mod scanner;

use scanner::{ScanResult, ScanStatus};
use std::path::Path;

fn parse_args() -> Result<(String, String, String), String> {
    let args: Vec<String> = std::env::args().collect();
    let mut target = None;
    let mut ioc = None;
    let mut report = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--target" => {
                i += 1;
                if i < args.len() {
                    target = Some(args[i].clone());
                } else {
                    return Err("Missing value for --target".to_string());
                }
            }
            "--ioc" => {
                i += 1;
                if i < args.len() {
                    ioc = Some(args[i].clone());
                } else {
                    return Err("Missing value for --ioc".to_string());
                }
            }
            "--report" => {
                i += 1;
                if i < args.len() {
                    report = Some(args[i].clone());
                } else {
                    return Err("Missing value for --report".to_string());
                }
            }
            _ => {
                return Err(format!("Unknown argument: {}", args[i]));
            }
        }
        i += 1;
    }

    match (target, ioc, report) {
        (Some(t), Some(i), Some(r)) => Ok((t, i, r)),
        _ => Err("Missing required arguments".to_string()),
    }
}

fn print_usage() {
    eprintln!("Usage:");
    eprintln!(
        "tp2_integrity_checker --target <FILE_OR_DIRECTORY> --ioc <IOC_FILE> --report <REPORT_FILE>"
    );
}

fn print_summary(
    results: &[ScanResult],
    iocs_count: usize,
    invalid_iocs: usize,
    target: &str,
    ioc_path: &str,
    report_path: &str,
) {
    let files_scanned = results.len();
    let matches = results
        .iter()
        .filter(|r| matches!(r.status, ScanStatus::Match(_)))
        .count();
    let errors = results
        .iter()
        .filter(|r| matches!(r.status, ScanStatus::Error(_)))
        .count();

    println!("TP2 File Integrity Checker and IOC Matcher");
    println!("Target: {}", target);
    println!("IOC file: {}", ioc_path);
    println!("Report: {}", report_path);
    println!();
    println!("Summary:");
    println!();
    println!("* Files scanned: {}", files_scanned);
    println!("* IOC entries loaded: {}", iocs_count);
    println!("* Invalid IOC lines: {}", invalid_iocs);
    println!("* Matches found: {}", matches);
    println!("* Errors: {}", errors);
    println!();

    if matches > 0 {
        println!("Matches:");
        for result in results {
            if let ScanStatus::Match(label) = &result.status {
                println!("[ALERT] {}", result.path);
                if let Some(hash) = &result.sha256 {
                    println!("SHA-256: {}", hash);
                }
                println!("IOC label: {}", label);
                println!();
            }
        }
    }

    println!("CSV report written to {}", report_path);
}

fn run() -> Result<(), String> {
    let (target_path, ioc_path, report_path) = parse_args()?;

    let target = Path::new(&target_path);
    if !target.exists() {
        return Err(format!("Error: target '{}' does not exist", target_path));
    }

    let ioc_file = Path::new(&ioc_path);
    if !ioc_file.exists() {
        return Err(format!("Error: IOC file '{}' does not exist", ioc_path));
    }

    let (iocs, invalid_iocs) =
        ioc::load_iocs(ioc_file).map_err(|e| format!("Error reading IOC file: {}", e))?;

    let results = scanner::scan_target(target, &iocs);

    let report_file = Path::new(&report_path);
    report::write_csv_report(&results, report_file)
        .map_err(|e| format!("Error writing report: {}", e))?;

    print_summary(
        &results,
        iocs.len(),
        invalid_iocs,
        &target_path,
        &ioc_path,
        &report_path,
    );

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        print_usage();
        std::process::exit(1);
    }
}
