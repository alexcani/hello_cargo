fn main() {
    /*let r;
    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);  // r outlives the value it is referencing!*/

    let string1 = String::from("abcd");
    let string2 = "xyzaa";
    let result = longest(string1.as_str(), string2);
    let string1 = 5;
    let string2 = 10;
    println!("The longest slice is: {}", result);
    println!("{} {}", string1, string2);
    println!("The longest slice is: {}", result);


    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let excertp = ImportantExcertp {part: first_sentence};
}

fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

// Lifetime annotations in struct definitions
struct ImportantExcertp<'a> {
    part: &'a str,
}

// Lifetime elision rules
// - Every parameter that is a reference gets assigned a unique lifetime parameter
// - If there is only 1 input lifetime, it is assigned to the output lifetime
// - If there are multiple input lifetime parameters, but one of them is is &self or &mut self,
// this lifetime is assigned to all output lifetime parameters

// Lifetime parameters in method
impl<'a> ImportantExcertp<'a> {  // lifetime in the impl block because it's part of the type
    fn level(&self) -> i32 {  // no lifetime in the method
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {  // elision rules apply
        // result type has lifetime of &self
        println!("Attention please: {}", announcement);
        self.part
    }
}
