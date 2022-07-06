// Library crate inside the package
// restaurant example

// Code inside module
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

use front_of_house::hosting;

fn test_fn() {
}

mod test_mod {
    // Modules are separate scopes altogether
    fn haha() {
        // neither of these calls will run
        //hosting::add_to_waitlist();   even with use keyword outside
        //test_fn();  even if it's declared above

        // Needs to use parent scope
        super::test_fn();
        super::hosting::add_to_waitlist();

        // The logic is that when a name is brought into scope what actually happens
        // is that the name is appended to the scope's path, so test_fn is
        // crate::test_fn. When calling test_fn from test_mod::haha, it would be like calling
        // crate::test_mod::test_fn, which doesn't exist.
        // Same thing for hosting and the use keyword, when use front_of_house::hosting; is
        // executed inside crate::, the alias crate::hosting is created, calling it from inside
        // test_mod would be like calling crate::test_mod::hosting::add_to_waitlist, which doesnt
        // exist

        // use
        use crate::front_of_house::hosting;
        hosting::add_to_waitlist();

        // use as
        use crate::front_of_house::hosting as host;
        host::add_to_waitlist();

        // When including enums/structs and items in general use the full path
        // When including functions use path up to the function's module
    }
}

// Public
pub fn eat_at_restaurant() {
    // absolute, from the crate root 'crate'
    crate::front_of_house::hosting::add_to_waitlist();

    // relative
    front_of_house::hosting::add_to_waitlist();




    hosting::add_to_waitlist();
}

// Re-exporting names as other name
pub use front_of_house::hosting as host_api;
// The exported name must already be public

// Nested paths in use
mod aa {
    // Includes both
    use crate::front_of_house::{hosting, hosting::add_to_waitlist};
    // or
    mod b {
        // Use self to include hosting
        use crate::front_of_house::hosting::{self, add_to_waitlist};
    }
}
