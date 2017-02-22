extern crate serde_json;
extern crate clap;

use serde_json::Value;
use serde_json::Deserializer;
use std::io::{self, Write};
use clap::{App, Arg};
use std::fs;
use std::result;
use std::error::Error;
use std::vec::Vec;
use std::string::String;

type Result<T> = result::Result<T, Box<Error>>;

fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => {
            writeln!(&mut io::stderr(), "error: {}", e).unwrap();
            std::process::exit(1);
        }
    }
}

fn run() -> Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("Extract JSON elements from a stream via JSON Pointers")
        .arg(Arg::with_name("INPUT")
            .short("i")
            .long("input")
            .takes_value(true)
            .help("input file to use if not receiving on stdin"))
        .arg(Arg::with_name("DELIMITER")
            .short("d")
            .long("delimiter")
            .takes_value(true)
            .help("delimiter between output values, default is tab"))
        .arg(Arg::with_name("POINTER")
            .required(true)
            .multiple(true)
            .help("JSON Pointer expressions to match on input"))
        .get_matches();

    let input = matches.value_of("INPUT").unwrap_or("-");
    let rdr: Box<io::Read> = match input {
        "-" => Box::new(io::stdin()),
        _ => Box::new(fs::File::open(input)?),
    };

    let delim = matches.value_of("DELIMITER").unwrap_or("\t");

    let mut pointers = Vec::new();
    // unwrap is safe as POINTER is required
    for p in matches.values_of("POINTER").unwrap() {
        pointers.push(p);
    }

    let iter = Deserializer::from_reader(rdr).into_iter::<Value>();
    for rv in iter {
        let v = rv?;
        let mut line = String::new();
        for p in &pointers {
            line.push_str(&render(v.pointer(p).unwrap_or(&Value::Null)));
            line.push_str(delim);
        }
        line.pop(); // remove the final tab
        println!("{}", line);
    }
    Ok(())
}

fn render(v: &Value) -> String {
    match v {
        &Value::Null => String::new(),
        &Value::Bool(ref b) => format!("{}", b),
        &Value::Number(ref b) => format!("{}", b),
        &Value::String(ref s) => {
            format!("{}",
                    s.replace("\n", "\\n")
                        .replace("\t", "\\t"))
        }
        &Value::Array(_) => format!("{}", v),
        &Value::Object(_) => format!("{}", v),
    }
}
