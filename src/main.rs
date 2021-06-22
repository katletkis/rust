use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let query = &args[1];
    let filename = &args[2];

    if args.len() != 3 {
        println!("usage: minigrep {{searchstring}} {{filename}}");
    } else {
        println!("Searching for {} in file {}", query, filename);

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        println!("File Contents\n{}", contents);
    }
}
