use std::{
    fs,
    io::{
        prelude::*,
        BufReader
    },
    net::{
        TcpListener,
        TcpStream
    },
};

const PORT: i32 = 7171;
const ADDRESS: &str = "127.0.0.1";


fn main() {
    println!("Dust Web Server!");

    let listener = TcpListener::bind("127.0.0.1:7171").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("[Dust Web Server]: Open here: {}:{}", ADDRESS, PORT);
        // println!("[Dust Web Server]: Connection established!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("./index.html").unwrap();
    let length = contents.len();
    // println!("Request: {:#?}", http_request);
    let response =
        format!("{status_line}\r\nContent-Type: text/html\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

