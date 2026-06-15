#!/usr/bin/env python3
"""Capture real terminal output from Docker and render as PNG screenshots."""
import subprocess
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont

BASE = Path("/home/shadowbytex0ff/compose-recipes/rust-venv/workspace/tp2_integrity_checker")
SCREENSHOTS = BASE / "screenshots"
SCREENSHOTS.mkdir(parents=True, exist_ok=True)

FONT_PATH = "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf"

def docker_exec(cmd):
    full = f"""docker exec rust-labs bash -lc 'export PATH=/opt/rust/cargo/bin:$PATH && cd /workspace/tp2_integrity_checker && {cmd}'"""
    r = subprocess.run(full, shell=True, capture_output=True, text=True, timeout=120)
    return r.stdout + r.stderr

def make_terminal_png(lines, output_path, title="", width=920):
    try:
        font = ImageFont.truetype(FONT_PATH, 13)
    except:
        font = ImageFont.load_default()
    
    line_h = 20
    pad = 24
    title_bar_h = 32
    content_h = len(lines) * line_h + pad
    height = title_bar_h + content_h + pad

    img = Image.new("RGB", (width, height), (30, 30, 30))
    draw = ImageDraw.Draw(img)

    # Title bar
    draw.rectangle([0, 0, width, title_bar_h], fill=(45, 45, 45))
    for i, (c, x) in enumerate([((255,95,87),10), ((255,189,46),30), ((39,201,63),50)]):
        draw.ellipse([x, 10, x+14, 24], fill=c)
    if title:
        draw.text((70, 7), title, fill=(180,180,180), font=ImageFont.truetype(FONT_PATH, 11) if FONT_PATH else font)

    # Terminal content
    y = title_bar_h + 12
    for line in lines:
        draw.text((pad, y), line, fill=(210,210,210), font=font)
        y += line_h

    img.save(output_path)
    return output_path

# Capture environment
env = docker_exec("rustc --version && cargo --version && cargo fmt --version && cargo clippy --version").strip().split("\n")
make_terminal_png(env, SCREENSHOTS / "01_environment.png", "Rust Toolchain - docker exec rust-labs")

# Capture sha256sum
sha = docker_exec("sha256sum samples/files/*").strip().split("\n")
make_terminal_png(sha, SCREENSHOTS / "02_sha256sum.png", "sha256sum samples/files/*")

# Capture cargo run
run = docker_exec("cargo run -- --target samples/files --ioc samples/iocs.txt --report reports/scan_report.csv").strip().split("\n")
make_terminal_png(run, SCREENSHOTS / "03_scan_result.png", "cargo run -- scan result", width=960)

# Capture cargo test
test = docker_exec("cargo test 2>&1").strip().split("\n")
make_terminal_png(test, SCREENSHOTS / "04_cargo_test.png", "cargo test -- 8 tests", width=920)

# Capture cargo clippy
clippy = docker_exec("cargo clippy -- -D warnings 2>&1").strip().split("\n")
make_terminal_png(clippy, SCREENSHOTS / "05_cargo_clippy.png", "cargo clippy -- -D warnings", width=720)

print("Done! Real terminal screenshots saved to screenshots/")
