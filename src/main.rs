use ctrlc;
use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    let should_shutdown = Arc::new(Mutex::new(false));

    {
        let should_shutdown = Arc::clone(&should_shutdown);
        ctrlc::set_handler(move || {
            println!("Got CTRL+C, will shut down");
            *should_shutdown.lock().unwrap() = true;
        })
        .unwrap();
    }

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.run(move || {
            handle_connection(stream);
        });

        if *should_shutdown.lock().unwrap() {
            break;
        }
    }

    println!("Shutting down");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, file) = match &http_request[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(10));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        "GET /sleep2 HTTP/1.1" => {
            thread::sleep(Duration::from_secs(10));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        "GET /sleep3 HTTP/1.1" => {
            thread::sleep(Duration::from_secs(10));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        "GET /sleep4 HTTP/1.1" => {
            thread::sleep(Duration::from_secs(10));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let file_contents = fs::read_to_string(file).unwrap();
    let length = file_contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{file_contents}");
    stream.write_all(response.as_bytes()).unwrap();

    println!("[{:#?}] Request: {:#?}", chrono::Local::now(), http_request);
}
