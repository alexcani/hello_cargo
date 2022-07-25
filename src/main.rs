use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let config = Config::new(&arguments).unwrap_or_else(|err| {
        eprintln!("Problems parsing argument: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
