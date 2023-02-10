fn main() {
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    let text = "inside closure".to_owned();
    let closure = move |x: i32| {
        println!("{}", text); // if closure didn't
        add_one(x)
    };

    let answer = do_twice(add_one, 5);
    println!("{answer}");

    //let answer = do_twice(closure, 5);  // does not work because closure captures value
    //println!("{answer}");

    //closures can only be coerced to `fn` types if they do not capture any variables

    // enum variants become initializer functions that can also be used
    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("{:?}", list_of_statuses);

    // Returning closures, way 1
    fn returns_closure(adder: bool) -> Box<dyn Fn(i32) -> i32> {
        if adder {
            Box::new(|x| x + 1)
        } else {
            Box::new(|x| x - 1)
        }
    }

    let x = returns_closure(true);
    let y = returns_closure(false);
    println!("x: {} | y: {}", x(1), y(1));

    // Returning closures, way 2
    fn returns_closure_2(adder: bool) -> impl Fn(i32) -> i32 {
        if adder {
            |x| x + 1
        } else {
            |x| x - 1
        }
    }

    let x = returns_closure_2(true);
    let y = returns_closure_2(false);
    println!("x: {} | y: {}", x(1), y(1));
}
