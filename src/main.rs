fn main() {
    println!("Hello, world!");
    another_function();
    another_function_2(2);
    let x = function_with_return(3);
    println!("Returned value {}", x);
    println!("function_with_early_return(-1): {}", function_with_early_return(-1));
    println!("function_with_early_return(2): {}", function_with_early_return(2));
}

fn another_function() {
    println!("Hey");
}

// Parameters
fn another_function_2(x: i32) {  // annotated types
    println!("x is {}", x);

    // Inner scopes are expressions
    let x = {
        let y = 6;
        y  // expressions have no semicolon
    };

    println!("other x is {}", x);
}

// Return values are simply expressions at the end of a function
fn function_with_return(x: i32) -> i32 {  // return value type is annotated
    // x + 1;   with ; it's a statement
    x+1 // now it's an expression and the return value
}

fn function_with_early_return(x: i32) -> i32 {
    if x < 0 {
        return 0;  // can return earlier with the return keyword (statement)
    }

    x + 1  // or implicitly with final expression
    // could also have been return x+1
}
