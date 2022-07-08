use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("teste.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("teste.txt") {
                Ok(file) => file,
                Err(error) => panic!("Couldn't create file {:?}", error)
            }
            other_errors => panic!("Other problems creating file: {:?}", other_errors)
        }
    };

    // oR unwrap or expect
    let f = File::open("teste.txt").unwrap();
    let f = File::open("teste.txt").expect("error message");
}


use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Result::Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Result::Ok(s),
        Err(e) => Result::Err(e),
    }
}

fn read_username_from_file_using_operator() -> Result<String, io::Error> {
    // ? is a unary operator that is called on the Result object
    // it is kind of a match operator that returns the Err to the calling code, or returns an
    // expression containing the value
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_chained() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

use std::fs;

fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")  // since this is a common operation
}
