use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from thread", i);
            thread::sleep(Duration::from_millis(50));
        }
    });

    for i in 1..5 {
        println!("Hi number {} from main thread", i);
        thread::sleep(Duration::from_millis(50));
    }

    handle.join().unwrap();

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {  // move makes the closure take ownership of used values
        println!("The vector: {:?}", v);  // v is owned by this thread now
    });

    handle.join().unwrap();
}
