const HOURS_IN_SECONDS: u32 = 3600;

fn main() {
    let mut x = 5;
    println!("Value of x is {}", x);
    x = 6;
    println!("Value of x is {}", x);
    println!("Value of HOURS_IN_SECONDS is {}", HOURS_IN_SECONDS);

    // Shadowing
    let y = 5;
    println!("Y is: {}", y);

    let mut y = y + 6;
    println!("Y is: {}", y);
    {
        y = y+2;
        println!("Y is: {}", y);
        let y = 3;
        println!("Y is: {}", y);
    }
    println!("Y is: {}", y);

    let spaces = "    ";  // first is a string
    let spaces = spaces.len();  // shadowed variable is integer, but still immutable
    println!("Spaces: {}", spaces);
}
