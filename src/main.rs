fn main() {
    // Literals
    let x = 1;
    match x {
        1 => println!("One"),
        2 => println!("Two"),
        _ => println!("other"),
    }

    // Named variables
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Match y = {y}"),
        _ => println!("Default, x = {:?}", x),
    }

    println!("at the end x = {:?}, y = {y}", x);

    // Multiple patterns
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("other"),
    }

    // Ranges
    let x = 5;
    match x {
        1..=5 => println!("one through five"), // ..= inclusive range
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("Early ASCII"),
        'k'..='z' => println!("Late ASCII"),
        _ => println!("something else"),
    }

    // Destructuring
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    // 'let' statements also take patterns
    let Point {x: a, y: b} = p;
    println!("a: {a}, b: {b}");

    // Can use the same variable name
    let Point {x: x, y: y} = p;
    println!("x: {x}, y: {y}");

    // To avoid duplication there is a shorthand, just use the same names as the fields
    let Point {x, y} = p;
    println!("x: {x}, y: {y}");

    // Can mix literals with named to test struct for particular values
    match p {
        Point{x, y:0} => println!("On the x axis at {x}"),
        Point{x: 0, y} => println!("On the y axis at {y}"),
        Point{x, y} => println!("Not on axis: ({x}, {y})"),
    }

    // Destructuring enums
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(125, 152, 100);
    match msg {
        Message::Quit => println!("Quit, no data"),
        Message::Move { x, y } => println!("Move has x: {x} and y: {y}"),
        Message::Write(text) => println!("Message has an unnamed string: {text}"),
        Message::ChangeColor(a, b, c) => println!("ChangeColor has 3 ints ({a}, {b}, {c})"),
    }

    // Can Match nested structs/tuples/enums
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum NewMessage {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = NewMessage::ChangeColor(Color::Rgb(0, 160, 255));
    match msg {
        NewMessage::ChangeColor(Color::Rgb(r, g, b)) => println!("rgb message ({r}, {g}, {b})"),
        NewMessage::ChangeColor(Color::Hsv(_, _, _)) => println!("hsv message"),
        _ => println!("other messages"),
    }

    // Destructuring structs and tuples
    // Can mix and match anything
    let ((feet, inches), Point{x, y}) = ((3, 10), p);
    println!("(({feet}, {inches}), x:{x}, y:{y})");


    // Ways to ignore values
    // With _
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }

    foo(3, 4);

    // Ignoring parts of value with nested _
    let mut setting_value = None;//Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => println!("Can't overwrite an existing value"),
        _ => setting_value = new_setting_value,
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 5, 6, 7);
    match numbers {
        (first, _, third, _, fifth) => println!("Some numbers: {first}, {third}, {fifth}"),
    }


    // Ignore unused variable by starting name with _
    let _x = 5;  // no warning, but difference is that _x is still valid variable which a bound value

    // Use .. to ignore the remaining parts of a value
    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = Point3D { x: 0, y: 0, z: 0};
    match origin {
        Point3D {x, ..} => println!("x is {x}"),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => println!("first: {first}, last: {last}"),
    }


    // Match guards
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("Even number {x}"),  // logic with the pattern variables
        Some(x) => println!("Odd number"),
        _ => ()
    }

    // Match guards apply to all patterns in 'or'
    let x = 4;
    let y = true;
    match x{
        4 | 5 | 6 if y => println!("yes"),  // aplies to all
        _ => println!("no"),
    }


    // @ bindings
    enum Msg {
        Hello {id: i32},
    }
    let msg = Msg::Hello { id: 5 };

    match msg {
        // Match ids between 3 and 7 incl
        // bind and test
        Msg::Hello { id: id @ 3..=7 } => println!("found id in range: {id}"),
        Msg::Hello { id: 10..=12 } => println!("found id in another range"),
        Msg::Hello { id } => println!("found id out of all ranges"),
    }

}
