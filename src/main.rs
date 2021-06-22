use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else( |_err| {
        // println!("Problem parsing arguments: {}", err);
        println!("usage: minigrep {{searchstring}} {{filename}}");
        process::exit(1);
    });

    // println!("Searching for '{}' in file '{}'", config.query, config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
    // run(config);
}

