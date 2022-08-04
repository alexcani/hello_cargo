#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue
}

struct Inventory {
    shirts: Vec<ShirtColor>
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Red => num_red += 1
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue]
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("The user with preference {:?} gets {:?}", user_pref1, giveaway1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("The user with preference {:?} gets {:?}", user_pref2, giveaway2);


    let mut list = vec![1, 2, 3];
    println!("Before closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);
    // println!("After defining, before calling: {:?}", list);  // won't compile
    borrows_mutably();
    println!("After calling closure: {:?}", list);

    // We can force the closure to take ownership by using the move keyword
    let _takes_ownership = move || list.push(8);  // doesn't need "mut" because it's not a mutable borrow
    //println!("Won't work because the closure owns the variable now: {:?}", list);
    // Even though we didn't call the closure yet

    // Closure traits:
    // FnOnce() -> can be called at least once. All closures implement this
    //             closures that move captured values out of the body only implement this
    // FnMut()  -> can be called more than once. Don't move values out of the body, but
    //             may mutate the captured values
    // Fn()     -> don't move captured values, don't mutate values, can be called more than once
    //             closures that don't capture anything also implement this

    // Functions can also implement the traits:
    // unwrap_or_else(Vec::new)  uses Vec::new instead of a closure
}
