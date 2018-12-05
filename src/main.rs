extern crate kitten;

use std::env;
use std::process;

use kitten::kinds::Config;
use kitten::exec;

fn main() {
    let cfg = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = exec::run(cfg) {
        eprintln!("Couldn't cat files: {}", e);
        process::exit(1);
    }
}
