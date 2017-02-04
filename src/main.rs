extern crate serde_json;
extern crate clap;

use serde_json::Value;
use serde_json::Deserializer;
use std::io;
use clap::{App, Arg};
use std::fs;

fn main() {
    
    let matches = App::new("jp").version("0.1")
        .about("Extract JSON elements from a stream")
        .author("Brian McCallister <brianm@skife.org>")
        .arg(Arg::with_name("INPUT").index(1).help("input file to use if not receiving on stdin"))
        .get_matches();

    let input = matches.value_of("INPUT").unwrap_or("-");    
    let rdr: io::Read = match input {
        "-" => io::stdin(),
        _   => fs::File::open(input).unwrap()
    };

    let iter = Deserializer::from_reader(rdr).into_iter::<Value>();
    for rv in iter {
        let v: Value = rv.unwrap();
        println!("{}\t{}",
                 v.pointer("/name").unwrap(),
                 v.pointer("/age").unwrap());
    }
}
