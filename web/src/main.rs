// 获取读写流所需的特定 trait
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use std::thread;
use threadpool::ThreadPool;

fn main() {
    // 绑定服务，可能失败，暂用unwrap
    let listenser = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listenser.incoming() {
        let stream = stream.unwrap();

        // println!("connection established!");
        // thread::spawn(|| {
        //     handle_connection(stream);
        // });
        pool.execute(|| {
            handle_connection(stream);
        })
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "response.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}{}",
        status_line,
        contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}