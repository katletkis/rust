use std::error::Error;
use std::env;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() != 3 {
            return Err("not enough arguments");
        }
        // let name = args[0].clone();
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_incensitive(&config.query, &contents)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line.trim());
        }
    }
    result
}

pub fn search_case_incensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut result: Vec<&str> = vec![];
    for line in contents.lines() {
        // NOTE: calling to_lowercase() creates new data rather than using a reference
        if line.to_lowercase().contains(&query.to_lowercase()) {
            result.push(line.trim());
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Duct tape";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three. 
        Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_incensitive(&query, &contents));
    }

    #[test]
    fn args() {
        let args: Vec<String> = vec!["my_prog".to_string(), "foo".to_string(), "bar".to_string()];
        let config = Config::new(&args).unwrap();
        assert_eq!(config.query, "foo");
        assert_eq!(config.filename, "bar");
    }

    #[test]
    #[should_panic(expected = "No such file")]
    fn run_bad_config() {
        let args: Vec<String> = vec!["my_prog".to_string(), "foo".to_string(), "bar".to_string()];
        let config = Config::new(&args).unwrap();
        if let Err(e) = run(config) {
            panic!("{}", e);
        }
    }

    #[test]
    fn run_good_config() {
        let args: Vec<String> = vec!["my_prog".to_string(), "foo".to_string(), "poem.txt".to_string()];
        let config = Config::new(&args).unwrap();
        if let Err(e) = run(config) {
            panic!("{}", e);
        }
    }
}
