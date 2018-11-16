use std::process::Command;

fn main() {
    Command::new("pandoc")
        .args(&["jp.1.md", "-s", "-t", "man", "-o", "target/jp.1"])
        .status()
        .unwrap();
}
