use std::env;
use std::fs;

//TODO: resolve issues
//  1. main has too many resposibilities (well 2 accept args and read file)
//  2. group configuration variables into 1 structure to make their purpose clearer
//  3. properly handle file errors instead of just using 'expect'
//  4. consolidate all error handling code into 1 place
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

fn get_args() {

}