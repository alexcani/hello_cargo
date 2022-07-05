#[derive(Debug)]
enum IpAddrKind {
    V4{x: String},
    V6(String)
}

impl IpAddrKind {
    fn call(&self) {
        println!("Called self: {}", match self {
            IpAddrKind::V4{x} => x,
            IpAddrKind::V6(x) => x
        });
    }
}

#[derive(Debug)]
enum State {
    A,
    B
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State)
}

fn us_coin_to_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("quarter from {:?}", state);
            25
        }
    }
}
fn main() {

    let ipv4 = IpAddrKind::V4{x: String::from("127.0.0.1")};
    let ipv6 = IpAddrKind::V6(String::from("::::"));

    func(&ipv4);
    func(&ipv6);

    ipv4.call();
    ipv6.call();

    println!("Coins");
    println!("penny: {} nickel {}", us_coin_to_cents(Coin::Penny),
                                    us_coin_to_cents(Coin::Nickel));
    println!("dime {}", us_coin_to_cents(Coin::Dime));
    println!("quarter A: {} quarter B: {}", us_coin_to_cents(Coin::Quarter(State::A)),
                                            us_coin_to_cents(Coin::Quarter(State::B)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    dbg!(five);
    dbg!(six);
    dbg!(none);

    let int_value = 9;
    match int_value {
        1 => println!("one"),
        9 => println!("nine"),
        _ => println!("other!"),  // catch-all, doesn't bind
        // to return nothing use ()
        //_ => ()
    }

    // if let
    // bad way
    let max = Some(3u8);
    match max {
        Some(x) => println!("max is {}", x),
        _ => ()  // boilerplater
    }

    // good way
    // if let pattern = expression { code }
    if let Some(x) = max {
        println!("max is {}", x);
    }

    // Can include an else
    let max: Option<u8> = None;
    if let Some(x) = max {
        println!("max is {}", x);
    } else {
        // Equivalent to a catch-all _
        println!("none");
    }
}

fn plus_one(x: Option<u32>) -> Option<u32> {
    match x {
        None => None,
        Some(x) => Some(x+1)
    }
}

fn func(test: &IpAddrKind) {
    println!("You printed {:#?}", test);
    match test {
        IpAddrKind::V4{x} => println!("It's ipv4 {}", x),
        IpAddrKind::V6(x) => println!("It's ipv6 {}", x)
    }
}
