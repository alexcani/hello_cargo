#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

pub fn add_two(x: i32) -> i32 {
    x+2
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]  // only compiles in test build
mod tests {  // placed along with the source code it tests
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    /*#[test]
    fn panics() {
        panic!("aaaa");
    }*/

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 10,
        };
        let smaller = Rectangle {
            width: 5,
            height: 9
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 10,
        };
        let smaller = Rectangle {
            width: 5,
            height: 9
        };

        assert!(!smaller.can_hold(&larger), "Can use this as error message!");
    }

    #[test]
    #[should_panic]
    fn large_guess() {
        Guess::new(200);
    }

    // Can use a subset of the panic message to make sure we're capturing the right panic
    #[test]
    #[should_panic(expected="Guess value must be between 1 and 100")]
    fn large_guess_expected() {
        Guess::new(200);
    }


    // Can also use Result in tests to indicate failure
    #[test]
    fn it_works_result() -> Result<(), String> {
        if 2+2 == 4 {
            Ok(())
        } else {
            Err(String::from("2+2 is not 4"))
        }
    }
}
