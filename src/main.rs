use minigrep::{run, Config};
use std::{env, process};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {}", error);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("in file {}", config.filename);

    if let Err(error) = run(config) {
        eprintln!("Application error: {}", error);
        process::exit(1);
    }
}
