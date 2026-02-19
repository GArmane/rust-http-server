use std::io::{BufRead, BufReader, Result, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, thread};

use http_server::thread_pool::ThreadPool;

fn main() -> Result<()> {
    let addr = "127.0.0.1:7878";
    let thread_pool = ThreadPool::build(16).unwrap();

    TcpListener::bind(addr)
        .unwrap()
        .incoming()
        .map(|stream| stream.unwrap())
        .for_each(|stream| {
            thread_pool.execute(|| handle_connection(stream));
        });

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let request_line = BufReader::new(&stream).lines().next().unwrap().unwrap();
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "./public/index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "./public/index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "./public/404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write_all(response.as_bytes()).unwrap();
}
