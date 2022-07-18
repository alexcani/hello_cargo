use std::env;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    println!("Arguments: {:#?}", arguments);

    let needle = &arguments.get(1).expect("Please provide 2 arguments");
    let haystack = &arguments.get(2).expect("Please provide 2 arguments");

    println!("Search for {} in {}", needle, haystack);
}
