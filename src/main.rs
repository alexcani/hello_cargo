use std::env;
use std::fs;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    println!("Arguments: {:#?}", arguments);

    let needle = &arguments.get(1).expect("Please provide 2 arguments");
    let haystack = &arguments.get(2).expect("Please provide 2 arguments");

    println!("Search for {} in {}", needle, haystack);

    let file_content = fs::read_to_string(haystack).expect("Something went wrong with reading the file");
    println!("File contents: {}", file_content);
}
