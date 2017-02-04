extern crate serde_json;
extern crate clap;

use serde_json::Value;
use serde_json::Deserializer;
use std::io;
use clap::{App, Arg};
use std::fs;
use std::result;
use std::error::{Error};
use std::vec::Vec;
use std::string::String;

type Result<T> = result::Result<T, Box<Error>>;

fn run() -> Result<()> {    
    let matches = App::new("jp").version("0.1")
        .about("Extract JSON elements from a stream")
        .author("Brian McCallister <brianm@skife.org>")
        .arg(Arg::with_name("INPUT")
                .short("i")
                .long("input")
                .takes_value(true)
                .help("input file to use if not receiving on stdin"))
        .arg(Arg::with_name("POINTER")
                .index(1)
                .required(true)
                .multiple(true)
                .help("JSON Pointer expressions to match on input"))
        .get_matches();

    let input = matches.value_of("INPUT").unwrap_or("-");    
    let rdr: Box<io::Read> = match input {
        "-" => Box::new(io::stdin()),
        _   => Box::new(try!(fs::File::open(input)))
    };

    let pointers_in = matches.values_of("POINTER").unwrap();
    let mut pointers = Vec::new();
    for p in pointers_in {
        pointers.push(p);
    }

    let iter = Deserializer::from_reader(rdr).into_iter::<Value>();
    for rv in iter {
        let mut line = String::new();
        let v: Value = rv.unwrap();
        for p in &pointers {
            line.push_str(&render(v.pointer(p).unwrap_or(&Value::Null)));
            line.push('\t');
        }
        line.pop();
        println!("{}", line);                 
    }
    Ok(())
}

fn main() {    
    match run() {
        Ok(_) => {}
        Err(e) => {
            // TODO print to STDERR instead of STDOUT
            println!("{}", e);
            std::process::exit(1);
        }
    }
}

fn render(v: &Value) -> String {
    match v {
        &Value::Null => String::new(),
        &Value::Bool(ref b) => format!("{}", b),
        &Value::Number(ref b) => format!("{}", b),
        &Value::String(ref s) => format!("{}", s),
        &Value::Array(_) =>  String::from("[...]"), //format!("{}", a),
        &Value::Object(_) => String::from("{...}")  //  format!("{}", o)
    }
}
