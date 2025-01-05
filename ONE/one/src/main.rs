use one::ThreadPool;
use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn handle_conection(mut stream: &TcpStream) {
    let buf_reader = BufReader::new(stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, file_name) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "example.html"),
        "GET /sleep HTTP/1.1" => {
            std::thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "example.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    let contents =
        fs::read_to_string(file_name).unwrap_or_else(|_| fs::read_to_string("500.html").unwrap());

    let content_length = contents.len();

    stream
        .write_all(
            format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{contents}")
                .as_bytes(),
        )
        .unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let buffer = stream.unwrap();

        pool.execute(move || {
            handle_conection(&buffer);
        });
    }
}
