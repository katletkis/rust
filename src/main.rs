use std::env;
use std::fs;
use std::process;

//TODO: resolve issues
//  1. main has too many resposibilities (well 2 accept args and read file)
//  2. group configuration variables into 1 structure to make their purpose clearer
//  3. properly handle file errors instead of just using 'expect'
//  4. consolidate all error handling code into 1 place
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else( |_err| {
        // println!("Problem parsing arguments: {}", err);
        println!("usage: minigrep {{searchstring}} {{filename}}");
        process::exit(1);
    });

    run(config);
}

fn run(config: Config) {
    println!("Searching for {} in file {}", config.query, config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("File Contents\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() != 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}