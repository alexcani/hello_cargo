fn main() {
    // Smart pointers
    // Box simply points to heap memory, no overhead
    //   When you want to pass ownership of large amounts of data without making copies
    //   Trait objects

    // Smart pointers implement special traits
    // Deref -> so they can be dereferenced (i.e. have value semantics with respect to the memory they point to)
    // Drop  -> what to run when they're deleted

    let x = Box::new(5);  // 5 is on the heap (this allocates memory)
    println!("value of x: {}", x);  // x is dereferenced

    // Enabling recursive types with Box
    // For when you can't know the size of a type at compile time
    use List::{Cons, Nil};
    let con_list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Cons(4, Box::new(Nil))))))));
    dbg!(&con_list);
    println!("{:?}", con_list);
}

// Cons list example using Box
// It's a recursive type
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),  // contains the value and the next element. List needs to be a Box so it has fixed size
    Nil               // or contains Nil
}
