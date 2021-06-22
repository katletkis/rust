use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() != 3 {
            return Err("not enough arguments");
        }
        // let name = args[0].clone();
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("File Contents\n{}", contents);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
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
