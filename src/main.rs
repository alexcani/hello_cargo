fn main() {
    let mut x = String::from("hey");
    x.push_str("aaaa");
    println!("{}", x);

    let y = 5;
    let x = y;

    println!("{}", x);
    println!("{}", y);

    let x = String::from("hello");
    let y = x.clone();

    println!("{}", x);
    println!("{}", y);

    // Copy trait for stack-only objects
    // Can't annotate with copy if any component of type implements Drop

    // Same with functions
    some_function(x);  // moves x
    //println!("x: {}", x);  // not valid
    let y = 5;
    other_function(y);  // copies because i32 has Copy trait
    println!("y: {}", y);


    // Ownership from function return values
    let mut x = gives_ownership();
    x = takes_then_gives(x);
    println!("x: {}", x);

    // Instead of giving ownership and taking back,
    // we can just borrow ownership using references

}

fn some_function(value: String) {
    println!("String in function: {}", value);
}

fn other_function(value: u32) {
    println!("int in function: {}", value);
}

fn gives_ownership() -> String {
    let x = String::from("hello from function");
    x
}

fn takes_then_gives(value: String) -> String {
    value
}
