use std::net::TcpListener;
const PORT: i32 = 7171;
const ADDRESS: &str = "127.0.0.1";


fn main() {
    println!("Dust Web Server!");

    let listener = TcpListener::bind("127.0.0.1:7171").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("[Dust Web Server]: Open here: {}:{}", ADDRESS, PORT);
        println!("[Dust Web Server]: Connection established!");
    }
}
