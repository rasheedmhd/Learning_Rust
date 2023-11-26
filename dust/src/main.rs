use std::{
    fs,
    thread,
    time::Duration,
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
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
    //     ("HTTP/1.1 200 OK", "./index.html")
    // } else {
    //     ("HTTP/1.1 404 NOT FOUND", "./404.html")
    // };

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "./index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));   
            ("HTTP/1.1 200 OK", "./index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "./404.html"),
    };             

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    // if request_line == "GET / HTTP/1.1" {
    //     let status_line = "HTTP/1.1 200 OK";
    //     let contents = fs::read_to_string("./index.html").unwrap();
    //     let length = contents.len();
    //     // println!("Request: {:#?}", http_request);
    //     let response =
    //         format!("{status_line}\r\nContent-Type: text/html\r\nContent-Length: {length}\r\n\r\n{contents}");
    //     stream.write_all(response.as_bytes()).unwrap();
    // } else {
    //     let status_line = "HTTP/1.1 404 NOT FOUND";
    //     let contents = fs::read_to_string("./404.html").unwrap();
    //     let length = contents.len();
    //     let response = 
    //         format!("{status_line}\r\nContent-Type: text/html\r\nContent-Length: {length}\r\n\r\n{contents}");
    //     stream.write_all(response.as_bytes()).unwrap();
    // }

}

