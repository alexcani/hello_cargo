use core::fmt;
use std::ops::{Add, Deref};

fn main() {
    // Placeholder types
    pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
    }
    struct Counter;

    impl Iterator for Counter {
        type Item = u32; // difference for generics is that this is the ONLY implementation
        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    // Default generic type parameters and operator overload
    #[derive(PartialEq, Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    // the trait add contains pub trait Add<Rhs = Self> { so we don't need to specify generic
    impl Add for Point {
        // from std::ops
        type Output = Point;

        fn add(self, rhs: Self) -> Self::Output {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    assert_eq!(
        Point { x: 1, y: 3 } + Point { x: 4, y: -2 },
        Point { x: 5, y: 1 }
    );

    // We can use different parameter
    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, rhs: Meters) -> Self::Output {
            // mm + m = mm
            Millimeters(self.0 + (rhs.0 * 1000))
        }
    }

    // Calling methods with the same name: fully qualified syntax
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;
    impl Human {
        fn fly(&self) {
            println!("Human is flying!");
        }
    }

    impl Pilot for Human {
        fn fly(&self) {
            println!("Human piloting plane");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Magically flying!");
        }
    }

    // Calling each method:
    let bob = Human;
    bob.fly(); // base impl
    Human::fly(&bob); // also works
    Wizard::fly(&bob);
    Pilot::fly(&bob);

    // Methods without self
    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    let _doggo = Dog;
    // Can't call  Animal::baby_name(&doggo) because it doesn't has self
    println!("Baby dogs are called {}", <Dog as Animal>::baby_name());  // right syntax


    // Supertraits
    // Traits that depend on traits
    // "if a type implements a trait, then it must also implement the other trait"

    trait OutlinePrint: fmt::Display {  // supertrait, slightly different from trait bounds
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len+4));
            println!("*{}*", " ".repeat(len+2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len+2));
            println!("{}", "*".repeat(len+4));
        }
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    impl OutlinePrint for Point {}
    Point{x:3, y:4}.outline_print();


    // Newtype pattern to implement external traits on external types

    // Create a thing wrapper around the external type and implement the trait
    struct Wrapper(Vec<String>);
    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    // Can now use the wrapper type
    let w = Wrapper(vec!["hello".to_owned(), "world".to_owned()]);
    println!("w = {}", w);

    // Can implement the Deref trait in the wrapper to use the methods of Vec transparently
    impl Deref for Wrapper {
        type Target = Vec<String>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    println!("{}", w.join(" and "));
}
