use std::borrow::Borrow;
use std::cell::{RefCell, Ref};
use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::vec;

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
    let con_list = Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Cons(4, Rc::new(Nil))))))));
    println!("{:?}", con_list);

    deref_operator_tests();
    drop_trait_tests();
    rc_pointer_tests();
    refcell_tests();
    weak_ptr_tests();
}

// Cons list example using Box
// It's a recursive type
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),  // contains the value and the next element. List needs to be a Box so it has fixed size
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

fn rc_pointer_tests() {
    use crate::List::{Cons, Nil};
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    dbg!(Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));  // could have called a.clone()
    dbg!(Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));  // but Rust convention is Rc::clone
        println!("{:?}\n{:?}", b, c);
        dbg!(Rc::strong_count(&a));
    }
    dbg!(Rc::strong_count(&a));
}

#[derive(Debug)]
enum ListRC {
    Cons(Rc<RefCell<i32>>, Rc<ListRC>),
    Nil
}

fn refcell_tests() {
    // RefCell is used to enforce borrowing rules at runtime instead of compile time
    // Interior Mutability: A Mutable Borrow to an Immutable Value
    let _x = 5;
    // let y = &mut _x; fails

    // Make a conlist with refcell
    use ListRC::*;
    let a = Rc::new(Cons(Rc::new(RefCell::new(5)), Rc::new(Nil)));
    println!("{:?}", a);

    let b = Rc::new(Cons(Rc::new(RefCell::new(3)), Rc::clone(&a)));
    let c = Rc::new(Cons(Rc::new(RefCell::new(4)), Rc::clone(&a)));
    println!("{:?}\n{:?}", b, c);

    // Change via 'a'
    if let Cons(x, _) = &*a {
        *x.borrow_mut() = 6;
    }

    println!("{:?}\n{:?}\n{:?}", a, b, c);

    // Change via 'b'
    if let Cons(_, x) = &*b {
        if let Cons(x, y) = &**x {
            *x.borrow_mut() = 7;
        }
    }

    println!("{:?}\n{:?}\n{:?}", a, b, c);
}

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

fn weak_ptr_tests() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
{
    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new()),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!(
        "branch strong = {}, weak = {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch),
    );

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
