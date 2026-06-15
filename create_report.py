#!/usr/bin/env python3
from pathlib import Path
from fpdf import FPDF

BASE = Path("/home/shadowbytex0ff/compose-recipes/rust-venv/workspace/tp2_integrity_checker")
SCREENSHOTS = BASE / "screenshots"
REPORT_DIR = BASE / "report"
REPORT_DIR.mkdir(parents=True, exist_ok=True)


class PDF(FPDF):
    def header(self):
        if self.page_no() > 1:
            self.set_font("Helvetica", "I", 8)
            self.cell(0, 10, "TP2 - File Integrity Checker and IOC Matcher", align="C", new_x="LMARGIN", new_y="NEXT")
            self.line(10, 15, 200, 15)
            self.ln(5)

    def footer(self):
        self.set_y(-15)
        self.set_font("Helvetica", "I", 8)
        self.cell(0, 10, f"Page {self.page_no()}/{{nb}}", align="C")

    def section_title(self, title):
        self.set_font("Helvetica", "B", 14)
        self.set_text_color(0, 51, 102)
        self.cell(0, 10, title, new_x="LMARGIN", new_y="NEXT")
        self.set_draw_color(0, 51, 102)
        self.line(10, self.get_y(), 200, self.get_y())
        self.ln(4)

    def sub_title(self, title):
        self.set_font("Helvetica", "B", 11)
        self.set_text_color(51, 51, 51)
        self.cell(0, 8, title, new_x="LMARGIN", new_y="NEXT")
        self.ln(2)

    def body_text(self, text):
        self.set_font("Helvetica", "", 10)
        self.set_text_color(0, 0, 0)
        self.multi_cell(0, 5, text)
        self.ln(2)

    def code_block(self, text):
        self.set_font("Courier", "", 8)
        self.set_fill_color(240, 240, 240)
        self.set_text_color(30, 30, 30)
        self.multi_cell(0, 4, text, fill=True)
        self.ln(3)

    def add_screenshot(self, img_path, caption, w=160):
        if img_path.exists():
            self.image(str(img_path), x=25, w=w)
            self.set_font("Helvetica", "I", 9)
            self.cell(0, 6, caption, align="C", new_x="LMARGIN", new_y="NEXT")
            self.ln(3)


pdf = PDF()
pdf.alias_nb_pages()
pdf.set_auto_page_break(auto=True, margin=20)

# ---- Title Page ----
pdf.add_page()
pdf.ln(40)
pdf.set_font("Helvetica", "B", 24)
pdf.set_text_color(0, 51, 102)
pdf.cell(0, 15, "TP2 - File Integrity Checker", align="C", new_x="LMARGIN", new_y="NEXT")
pdf.cell(0, 15, "and IOC Matcher in Rust", align="C", new_x="LMARGIN", new_y="NEXT")
pdf.ln(10)
pdf.set_font("Helvetica", "", 14)
pdf.set_text_color(80, 80, 80)
pdf.cell(0, 8, "Module 7.1 - Programming with Rust", align="C", new_x="LMARGIN", new_y="NEXT")
pdf.ln(20)

pdf.set_text_color(0, 0, 0)
pdf.set_font("Helvetica", "", 12)
pdf.cell(0, 8, "Student: Abderahman Mohamed Lemin", align="C", new_x="LMARGIN", new_y="NEXT")
pdf.cell(0, 8, "Student ID: 25235", align="C", new_x="LMARGIN", new_y="NEXT")
pdf.cell(0, 8, "Date: June 15, 2026", align="C", new_x="LMARGIN", new_y="NEXT")
pdf.cell(0, 8, "GitHub: https://github.com/x0abderahman/tp2_integrity_checker", align="C", new_x="LMARGIN", new_y="NEXT")
pdf.ln(30)
pdf.set_font("Helvetica", "I", 10)
pdf.cell(0, 8, "Defensive Security - File Integrity Verification Tool", align="C", new_x="LMARGIN", new_y="NEXT")

# ---- Objective ----
pdf.add_page()
pdf.section_title("1. Objective")
pdf.body_text(
    "This project implements a command-line File Integrity Checker and IOC (Indicator of "
    "Compromise) Matcher written in Rust. The tool scans a file or directory, calculates "
    "SHA-256 cryptographic hashes for each regular file, compares them against a provided "
    "list of known malicious hashes (IOCs), and produces a structured CSV report."
)
pdf.body_text(
    "File integrity checking is a fundamental defensive security technique. It allows "
    "security analysts to verify that files have not been tampered with by comparing their "
    "current cryptographic hash against a known good or known bad baseline. An IOC "
    "(Indicator of Compromise) is a piece of forensic evidence - such as a file hash, IP "
    "address, or domain name - that indicates a potential security breach."
)
pdf.body_text(
    "The tool is designed with secure programming practices: it avoids unsafe code, handles "
    "errors gracefully without panicking, and splits functionality into well-defined modules."
)

# ---- Environment ----
pdf.add_page()
pdf.section_title("2. Environment")
pdf.body_text("The development environment uses a Docker Compose setup with Debian Bookworm:")
pdf.code_block(
    "mkdir -p workspace\n"
    "docker compose up -d --build\n"
    "docker compose exec rustlab bash\n"
    "cd /workspace/tp2_integrity_checker"
)
pdf.body_text("Rust toolchain versions:")
pdf.code_block(
    "rustc 1.95.0 (59807616e 2026-04-14)\n"
    "cargo 1.95.0 (f2d3ce0bd 2026-03-21)\n"
    "rustfmt 1.9.0-stable\n"
    "clippy 0.1.95"
)
pdf.body_text("Project path inside the container: /workspace/tp2_integrity_checker")
pdf.body_text("Student ID: 25235")

# ---- Implementation ----
pdf.add_page()
pdf.section_title("3. Implementation")

pdf.sub_title("3.1 Hashing Module (hashing.rs)")
pdf.body_text(
    "The hashing module implements the SHA-256 hash calculation using the sha2 crate. "
    "Files are read in 8KB buffers to handle large files efficiently without loading the "
    "entire content into memory. The hash is returned as a lowercase hexadecimal string."
)
pdf.code_block(
    "pub fn hash_file_sha256(path: &Path) -> Result<String, io::Error> {\n"
    '    let mut file = File::open(path)?;\n'
    "    let mut hasher = Sha256::new();\n"
    "    let mut buffer = [0u8; 8192];\n"
    "    loop {\n"
    "        let bytes_read = file.read(&mut buffer)?;\n"
    "        if bytes_read == 0 { break; }\n"
    "        hasher.update(&buffer[..bytes_read]);\n"
    "    }\n"
    '    Ok(format!("{:x}", hasher.finalize()))\n'
    "}"
)

pdf.sub_title("3.2 IOC Parsing Module (ioc.rs)")
pdf.body_text(
    "The IOC module parses a CSV-like file containing hashes and labels. It handles: "
    "empty lines (ignored), comment lines starting with # (ignored), invalid SHA-256 "
    "strings (counted but skipped), and valid entries (parsed into IocEntry structs). "
    "The is_valid_sha256 function validates that strings are exactly 64 hexadecimal characters."
)

pdf.sub_title("3.3 Scanner Module (scanner.rs)")
pdf.body_text(
    "The scanner module walks through a target path using the walkdir crate for recursive "
    "directory traversal. It scans all regular files in nested directories. Hashing is "
    "parallelized using the rayon crate (par_iter). Each file's hash is compared against "
    "the IOC list and optionally against a known-good manifest."
)

pdf.sub_title("3.4 Report Module (report.rs)")
pdf.body_text(
    "The report module generates both CSV and JSON output. The CSV uses the csv crate "
    "with columns: path, sha256, status, label. The JSON output includes a complete "
    "scan report with timestamp, summary statistics, and per-file details. Results are "
    "sorted by path for reproducible output."
)

pdf.sub_title("3.5 Manifest Module (manifest.rs)")
pdf.body_text(
    "A new module for parsing known-good manifest files. This allows the tool to identify "
    "and label trusted files, distinguishing them from unknown or suspicious files. "
    "The format is identical to the IOC file format."
)

pdf.sub_title("3.6 CLI Handling (main.rs)")
pdf.body_text(
    "Command-line arguments support --target, --ioc, --report (required), plus optional "
    "--json <PATH>, --only-matches, and --good-manifest <PATH>. Missing arguments produce "
    "a clear usage message. The main function delegates to run() which returns Result, "
    "avoiding unwrap() in the execution path."
)
pdf.code_block(
    "Usage:\n"
    "  tp2_integrity_checker --target <FILE_OR_DIR> --ioc <IOC_FILE> --report <REPORT>\n"
    "  [--json <JSON>] [--only-matches] [--good-manifest <MANIFEST>]"
)

# ---- Execution Evidence ----
pdf.add_page()
pdf.section_title("4. Execution Evidence")
pdf.body_text("The following screenshots demonstrate successful execution of the tool:")

pdf.add_screenshot(SCREENSHOTS / "02_cargo_run.png", "Figure 2: cargo run base scan output", w=160)
pdf.add_screenshot(SCREENSHOTS / "03_cargo_scan.png", "Figure 3: Scan result with match detection", w=160)
pdf.add_screenshot(SCREENSHOTS / "04_cargo_test.png", "Figure 4: cargo test - 12 unit tests + 4 integration tests", w=160)
pdf.add_screenshot(SCREENSHOTS / "05_cargo_clippy.png", "Figure 5: cargo clippy - zero warnings", w=160)
pdf.add_screenshot(SCREENSHOTS / "06_cargo_audit.png", "Figure 6: cargo audit - dependency vulnerabilities check", w=140)
pdf.add_screenshot(SCREENSHOTS / "07_json.png", "Figure 7: JSON output format", w=160)
pdf.add_screenshot(SCREENSHOTS / "08_good_manifest.png", "Figure 8: Known-good manifest mode", w=160)

pdf.body_text("Generated CSV report content:")
csv_content = (
    "path,sha256,status,label\n"
    "samples/files/clean_readme.txt,"
    "70bbeaa0f2d408a45827aa9d5bd58209564a5bd7b61d6c069267c9e9e35f97cd,CLEAN,\n"
    "samples/files/suspicious_dropper.txt,"
    "44ea92bec1f9e8aa690d8aceddf1294e9fb4a71d39769d6f383e3915ac76bb3b,MATCH,"
    "Demo suspicious test sample\n"
    "samples/files/notes.txt,"
    "1888673b4c962129e57b54b81fda2f967c52f87c28e9d7908e5fba0dfed097e3,CLEAN,"
)
pdf.code_block(csv_content)

pdf.body_text("Test results: 12 unit tests + 4 integration tests = 16 total, all passing.")
pdf.body_text("cargo clippy: zero warnings with -D warnings.")

# ---- Discussion ----
pdf.add_page()
pdf.section_title("5. Discussion")

pdf.sub_title("5.1 Invalid IOC Handling")
pdf.body_text(
    "The IOC file parser gracefully handles invalid lines: lines that are not 64-character "
    "hexadecimal strings are counted as invalid and skipped. The program continues execution "
    "and reports the number of invalid lines in the summary."
)

pdf.sub_title("5.2 Missing File Handling")
pdf.body_text(
    "If the target file or directory does not exist, the program prints a clear error "
    "message and exits with a non-zero status code."
)

pdf.sub_title("5.3 Why SHA-256 Over MD5 or SHA-1")
pdf.body_text(
    "SHA-256 is preferred over MD5 and SHA-1 because both are cryptographically broken. "
    "SHA-256 remains collision-resistant and is the NIST standard."
)

pdf.sub_title("5.4 Bonus Features Implemented")
pdf.body_text(
    "The following bonus features have been implemented:\n\n"
    "- Recursive directory scanning using the walkdir crate\n"
    "- JSON output option (--json) with structured scan reports\n"
    "- --only-matches flag to filter output to suspicious files only\n"
    "- Known-good manifest mode (--good-manifest) using a separate hash allowlist\n"
    "- Parallel scanning using rayon threads for multi-core performance\n"
    "- Integration tests in tests/integration_test.rs (4 test cases)"
)

# ---- Conclusion ----
pdf.add_page()
pdf.section_title("6. Conclusion")
pdf.body_text(
    "This TP successfully implemented a File Integrity Checker and IOC Matcher in Rust "
    "with all mandatory requirements and multiple bonus features."
)
pdf.body_text(
    "Rust concepts practiced:\n"
    "- Multi-module project structure (6 modules)\n"
    "- External crate dependencies (sha2, csv, walkdir, serde, serde_json, rayon)\n"
    "- Result and Option for error handling without panics\n"
    "- Struct definitions, enum types, trait derivations\n"
    "- File I/O, directory traversal, cryptographic hashing\n"
    "- Unit testing (12 tests) and integration testing (4 tests)\n"
    "- Parallel processing with rayon\n"
    "- Serializable data structures with serde\n"
    "- Code formatting (cargo fmt) and linting (cargo clippy -D warnings)"
)

# Save with correct filename
output_path = REPORT_DIR / "TP2_Rust_Integrity_Checker_Abderahman_Mohamed_Lemin.pdf"
pdf.output(str(output_path))
print(f"PDF report created: {output_path}")
