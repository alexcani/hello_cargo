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

// Public
pub fn eat_at_restaurant() {
    // absolute, from the crate root 'crate'
    crate::front_of_house::hosting::add_to_waitlist();

    // relative
    front_of_house::hosting::add_to_waitlist();
}
