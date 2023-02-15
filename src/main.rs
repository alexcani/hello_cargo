use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, file) = if http_request == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let file_contents = fs::read_to_string(file).unwrap();
    let length = file_contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{file_contents}");
    stream.write_all(response.as_bytes()).unwrap();

    println!("Request: {:#?}", http_request);
}
