use std::ops::Deref;

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
    println!("{:?}", con_list);

    deref_operator_tests();
    drop_trait_tests();
}

// Cons list example using Box
// It's a recursive type
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),  // contains the value and the next element. List needs to be a Box so it has fixed size
    Nil               // or contains Nil
}

fn deref_operator_tests() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    // assert_eq!(5, y); fails
    assert_eq!(5, *y);

    println!("{} {} {}", x, y, *y);  // coersion happens here

    // Using Box as reference
    let z = Box::new(x);
    assert_eq!(5, *z);

    // Using custom smart pointer
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Deref coercion to &str
    let x = MyBox::new(String::from("hey"));
    hello(&x);  // &MyBox gets coersed into &String then &str because it implements Deref trait
}

// Our custom smart pointer
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {  // deref trait
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(x: &str) {
    println!("Hello, {x}");
}



// Drop trait
struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}", self.data);
    }
}

fn drop_trait_tests() {
    let a = CustomSmartPointer {
        data: String::from("stuff")
    };
    let b = CustomSmartPointer {
        data: String::from("other stuff")
    };
    println!("Created smart pointers");
    drop(b);

    println!("Called drop")
}
