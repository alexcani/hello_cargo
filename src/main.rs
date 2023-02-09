fn main() {
    // Type aliases
    type Kilometers = i32;

    // However it's just a synonym, not a different type
    let x: i32 = 5;
    let y: Kilometers = 6;
    println!("{x} + {y} = {}", x+y);

    // Main use case is to reduce lengthy types
    type _Thunk = Box<dyn Fn() + Send + 'static>;

    // ! type

    // Dynamically Sized Types, Sized Trait
    // Rust implicitly adds Sized to all generic functions so they only work with types
    // that have known sizes at compile time
    fn generic<T>(t: T) {

    }

    // Is actually
    fn _generic<T: Sized>(t: T) {

    }

    // But we can relax this restriction to allow non Sized types
    fn __generic<T: ?Sized>(t: &T) {  // also use &T instead of T because t may not be Sized

    }
}
