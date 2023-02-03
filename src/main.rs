fn main() {
    // Inside unsafe blocks one can do 5 things:
    //  Dereference a raw pointer
    //  Call an unsafe function or method
    //  Access or modify a mutable static variable
    //  Implement an unsafe trait
    //  Access fields of unions

    // Dereference raw pointer
    // 2 new types: raw pointers
    //  *const T -> immutable pointer
    //  *mut T   -> mutable pointer
    let mut num = 5;

    // Raw pointers can be created outside safe blocks normally
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;  // can have multiple mut and const pointers to same location

    let address = 0x012345usize;
    let r = address as *const i32;

    // But dereferencing needs to be inside unsafe
    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }

    // Caling unsafe functions or methods
    unsafe fn dangerous() {}  // function body is implicitly "unsafe", no need to add another block

    // dangerous();   can't call normally
    unsafe {
        dangerous();
    }


    // Using extern functions to call external code
    // Functions declared in extern blocks are always unsafe to call
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("abs(-3) from C library: {}", abs(-3));
    }

    // We can also use extern to create interfaces that allow other languages to call Rust code
    #[no_mangle]
    pub extern "C" fn callable_from_c() {
        println!("This has been written in rust");
    }
    // If someone were to compile this as a .so and link it against a C program it would work


    // Access or modify mutable static variable
    println!("name is {}", HELLO_WORLD);  // access immutable is safe

    // Accessing mutable is not safe
    //  println!("counter is {COUNTER}"); doesn't work
    unsafe {
        println!("counter is {COUNTER}");
    }

    // Safe function encapsulating unsafe code
    add_to_count(3);

    unsafe {
        println!("counter is {COUNTER}");
    }



    // Implementing an unsafe trait
    unsafe trait Foo {   // mark trait as unsafe
    }

    unsafe impl Foo for i32 {  // mark implementation as unsafe as well
    }

    // Accessing union fields is also unsafe (usually to interface with C code)
}

// Global variable
static HELLO_WORLD: &str = "Hello, world!";
// Mutabel global variable
static mut COUNTER: i32 = 0;

fn add_to_count(inc: i32) {
    unsafe {  // mutating mut statics is also unsafe
        COUNTER += inc;
    }
}
