use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let needle = args[1].clone();
        let haystack = args[2].clone();

        Ok(Config {
            query: needle,
            filename:haystack
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Search for {} in {}", config.query, config.filename);

    let file_content = fs::read_to_string(config.filename)?;
    println!("File contents: {}", file_content);

    Ok(())
}
