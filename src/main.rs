use std::{thread, time::Duration, sync::mpsc};

fn main() {
    //basic_stuff();
    //message_passing_concurrency();
    shared_state_concurrency();
}

fn basic_stuff() {
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
    let handle = thread::spawn(move || {
        // move makes the closure take ownership of used values
        println!("The vector: {:?}", v); // v is owned by this thread now
    });

    handle.join().unwrap();
}

fn message_passing_concurrency() {
    // Create a channel
    // mpsc: multiple producer, single consumer
    let (tx, rc) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    // This blocks, there are non-blocking variantes
    let msg = rc.recv().unwrap();
    println!("Message is: {}", msg);

    // Multiple messages
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let values = vec![
            String::from("hello"),
            String::from("many"),
            String::from("messages"),
            String::from("from this thread"),
        ];
        for i in values {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for msg in rx {
        println!("Got: {}", msg);
    }

    println!("Multiple producers:");

    // Multiple senders by cloning the transmitter
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    // Thread 1
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Thread 2
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // One receiver
    for received in rx {
        println!("Got: {}", received);
    }
}

fn shared_state_concurrency() {
    use std::sync::Mutex;
    use std::sync::Arc;

    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("{:?}", m);

    // Increment wiht multiple threads
    // Rc is no thread-safe, so we usa Arc
    let counter = Arc::new(Mutex::new(0));
    let mut t_handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut val = counter.lock().unwrap();
            *val += 1;
        });
        t_handles.push(handle);
    }

    for i in t_handles {
        i.join().unwrap();
    }

    println!("Result: {:?}", counter);
}
