mod hashing;
mod ioc;
mod manifest;
mod report;
mod scanner;

use scanner::{ScanResult, ScanStatus};
use std::path::Path;

struct Args {
    target: String,
    ioc: String,
    report: String,
    json: Option<String>,
    only_matches: bool,
    good_manifest: Option<String>,
}

fn parse_args() -> Result<Args, String> {
    let args: Vec<String> = std::env::args().collect();
    let mut target = None;
    let mut ioc = None;
    let mut report = None;
    let mut json = None;
    let mut only_matches = false;
    let mut good_manifest = None;

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
            "--json" => {
                i += 1;
                if i < args.len() {
                    json = Some(args[i].clone());
                } else {
                    return Err("Missing value for --json".to_string());
                }
            }
            "--only-matches" => {
                only_matches = true;
            }
            "--good-manifest" => {
                i += 1;
                if i < args.len() {
                    good_manifest = Some(args[i].clone());
                } else {
                    return Err("Missing value for --good-manifest".to_string());
                }
            }
            _ => {
                return Err(format!("Unknown argument: {}", args[i]));
            }
        }
        i += 1;
    }

    match (target, ioc, report) {
        (Some(t), Some(i), Some(r)) => Ok(Args {
            target: t,
            ioc: i,
            report: r,
            json,
            only_matches,
            good_manifest,
        }),
        _ => Err("Missing required arguments".to_string()),
    }
}

fn print_usage() {
    eprintln!("Usage:");
    eprintln!("  tp2_integrity_checker --target <FILE_OR_DIRECTORY> --ioc <IOC_FILE> --report <REPORT_FILE>");
    eprintln!("  tp2_integrity_checker --target <FILE_OR_DIR> --ioc <IOC_FILE> --report <CSV> [--json <JSON>] [--only-matches] [--good-manifest <MANIFEST>]");
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
    let known_good = results
        .iter()
        .filter(|r| matches!(r.status, ScanStatus::KnownGood(_)))
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
    println!("* Known-good files: {}", known_good);
    println!("* Matches found: {}", matches);
    println!("* Errors: {}", errors);
    println!();

    if known_good > 0 {
        println!("Known-good files:");
        for result in results {
            if let ScanStatus::KnownGood(label) = &result.status {
                println!("[OK] {} ({})", result.path, label);
            }
        }
        println!();
    }

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
    let args = parse_args()?;

    let target = Path::new(&args.target);
    if !target.exists() {
        return Err(format!("Error: target '{}' does not exist", args.target));
    }

    let ioc_file = Path::new(&args.ioc);
    if !ioc_file.exists() {
        return Err(format!("Error: IOC file '{}' does not exist", args.ioc));
    }

    let (iocs, invalid_iocs) =
        ioc::load_iocs(ioc_file).map_err(|e| format!("Error reading IOC file: {}", e))?;

    let good_manifest = if let Some(ref path) = args.good_manifest {
        let p = Path::new(path);
        if p.exists() {
            manifest::load_manifest(p).map_err(|e| format!("Error reading manifest: {}", e))?
        } else {
            return Err(format!("Error: manifest file '{}' does not exist", path));
        }
    } else {
        Vec::new()
    };

    let mut results = scanner::scan_target(target, &iocs, &good_manifest);

    results.sort_by(|a, b| a.path.cmp(&b.path));

    let output_results: Vec<ScanResult> = if args.only_matches {
        results
            .into_iter()
            .filter(|r| matches!(r.status, ScanStatus::Match(_)))
            .collect()
    } else {
        results
    };

    let report_file = Path::new(&args.report);
    report::write_csv_report(&output_results, report_file)
        .map_err(|e| format!("Error writing report: {}", e))?;

    if let Some(ref json_path) = args.json {
        let p = Path::new(json_path);
        report::write_json_report(&output_results, p)
            .map_err(|e| format!("Error writing JSON report: {}", e))?;
        println!("JSON report written to {}", json_path);
    }

    print_summary(
        &output_results,
        iocs.len(),
        invalid_iocs,
        &args.target,
        &args.ioc,
        &args.report,
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
