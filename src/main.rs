fn main() {
    // Breaking and continuing loops with loop labels
    let mut count = 0;
    'counting_up: loop {  // loop label with the 'name: format
        println!("Count: {}", count);
        let mut remaining = 10;

        loop {
            println!("Remaining: {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;  // break outer loop
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count: {}", count);


    // Returning values from loops?
    let mut counter = 0;
    let x = loop {  // loops are expressions
        counter += 1;
        if counter == 10 {
            break counter * 2;  // break loop returning expressions
        }
    };  // semicolon
    println!("Result is {}", x);

    // whiles
    let mut x = 0;
    while x < 5 {
        x += 1;
    }
    println!("x: {}", x);

    // for loop to iterate over collections
    let a = [1, 2, 3, 4, 5];
    for element in a {
        println!("element: {}", element);
    }


    // Use range
    for i in 1..3 {  // 1 to 2
        println!("Range i: {}", i);
    }
    for i in (1..3).rev() {  // rev to reverse
        println!("Range i: {}", i);
    }
    for i in (1..=3).rev() {  // right-including range, reversed (3 down to 1)
        println!("Range i: {}", i);
    }

    for i in 0..=5 {
        println!("fib({}) = {}", i, fib(i));
    }
}

fn fib(x: u32) -> u32 {
    if x == 0 {
        return 0
    }

    if x == 1 {
        return 1
    }

    fib(x-1) + fib(x-2)
}
