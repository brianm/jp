use anyhow::Result;
use clap::Parser;
use human_panic::setup_panic;
use serde_json::Deserializer;
use serde_json::Value;
use std::fs;
use std::io;
use std::string::String;
use std::vec::Vec;

#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Extract JSON elements from a stream via JSON Pointers")]
struct Args {
    /// Input file to use if not receiving on stdin
    #[arg(short, long)]
    input: Option<String>,

    /// Delimiter between output values, default is tab
    #[arg(short, long, default_value = "\t")]
    delimiter: String,

    /// JSON Pointer expressions to match on input
    #[arg(required = true)]
    pointers: Vec<String>,
}

fn main() {
    setup_panic!();

    match run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    }
}

fn run() -> Result<()> {
    let args = Args::parse();

    let rdr: Box<dyn io::Read> = match &args.input {
        Some(path) => Box::new(fs::File::open(path)?),
        None => Box::new(io::stdin()),
    };

    let delim = &args.delimiter;

    let iter = Deserializer::from_reader(rdr).into_iter::<Value>();
    for rv in iter {
        let v = rv?;
        let mut line = String::new();
        for p in &args.pointers {
            line.push_str(&render(v.pointer(p).unwrap_or(&Value::Null)));
            line.push_str(delim);
        }
        line.pop(); // remove the final delimiter
        println!("{}", line);
    }
    Ok(())
}

fn render(v: &Value) -> String {
    match v {
        Value::Null => String::new(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.replace('\n', "\\n").replace('\t', "\\t"),
        Value::Array(_) | Value::Object(_) => v.to_string(),
    }
}
