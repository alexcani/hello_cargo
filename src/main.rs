use std::env;
use std::fs;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let config = Config::new(&arguments);

    println!("Arguments: {:#?}", arguments);
    println!("Search for {} in {}", config.query, config.filename);

    let file_content = fs::read_to_string(config.filename).expect("Something went wrong with reading the file");
    println!("File contents: {}", file_content);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let needle = args.get(1).expect("Please provide 2 arguments").clone();
        let haystack = args.get(2).expect("Please provide 2 arguments").clone();

        Config {
            query: needle,
            filename:haystack
        }
    }
}
