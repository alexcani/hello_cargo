use std::io;
use std::io::prelude::*;

fn main() {
    let guess: u32 = "42".parse().expect("Input a number!");
    println!("{}", guess);

    let guess: u8 = 0b1101_1000;
    println!("{}", guess);

    let guess = b'A';
    println!("{}", guess);

    let a = 5;
    let b = 3;
    let c = a/b;  // integer division
    println!("C is: {}", c);

    let zero = 2/3;
    println!("Zero: {}", zero);

    let c = '😻';
    println!("Emoji: {}", c);

    let b = true;  // bool
    println!("B is {}", b);
    println!("!B is {}", !b);


    // Tuples and arrays
    let tup = (32, -3, "hey");  // different types
    let (x, y, z) = tup;  // pattern matching
    println!("x: {}, y: {}, z: {}", x, y, z);
    println!("x: {}, y: {}, z: {}", tup.0, tup.1, tup.2);

    // Mutable tuples?
    let mut tup = (1, 2, 3);
    tup.1 = 5;
    println!("tup: {} {} {}", tup.0, tup.1, tup.2);

    // Arrays are fixed length, same type containers
    let arr = [1, 2, 3, 4, 5];  // allocated on stack instead of heap
    println!("{}", arr[0]);

    let arr: [u32; 5] = [0, 2, 4, 6, 8];  // type is annotated that way
    println!("{}", arr[0]);

    // Initialize to same value
    let arr = [5; 10];  // 10 elements with value of 5, type is inferred
    println!("{}", arr[0]);
    let arr : [u32; 10] = [5; 10];  // with explicit type annotation
    println!("{}", arr[0]);

    print!("Insert index: ");
    io::stdout().flush().expect("Error printing");

    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Insert index");
    let index : usize = index.trim().parse().expect("Insert number");
    println!("arr[{}] = {}", index, arr[index]);
}
