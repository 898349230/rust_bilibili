use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs;
use std::time::Duration;
use std::thread;

use web_demo::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7879").unwrap();
    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();
    //     handle_connection(stream);
    // }

        let pool: ThreadPool = ThreadPool::new(4);
        for stream in listener.incoming().take(2) {
            let stream = stream.unwrap();
            pool.execute (|| {
                handle_connection(stream);
            });
        }
        println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    // 响应  b 将文本转换为字节字符串
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    }else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}
