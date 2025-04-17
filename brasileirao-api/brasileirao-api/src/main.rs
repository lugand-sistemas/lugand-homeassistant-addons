use std::net::TcpListener;
use std::io::Write;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Servidor rodando na porta 8080");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nOlá, Home Assistant!";
        stream.write_all(response.as_bytes()).unwrap();
    }
}
