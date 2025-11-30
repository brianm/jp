use std::process::Command;

fn main() {
    // Generate man page if pandoc is available (optional)
    let _ = Command::new("pandoc")
        .args(["jp.1.md", "-s", "-t", "man", "-o", "target/jp.1"])
        .status();
}
