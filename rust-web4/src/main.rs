use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::io::{BufRead, Write, BufReader};
use std::path::Path;

#[derive(Debug, Clone)]
struct Request {
    method: String,
    path: String,
    _headers: Vec<(String, String)>,
}

fn get_request(tcp_stream: &mut TcpStream) -> Request {
    let mut reader = BufReader::new(tcp_stream);
    let mut line = String::new();

    reader.read_line(&mut line).unwrap();
    let fields: Vec<&str> = line.split_whitespace().collect();
    let method = fields[0].into();
    let path = fields[1].into();

    let mut headers = Vec::new();
    loop {
        line.clear();
        reader.read_line(&mut line).unwrap();
        let line = line.trim_end();
        if line.is_empty() {
            return Request { method, path, _headers: headers };
        }
        // eprintln!("header: {:?}", line.trim_end());
        let (name, value) = line.split_once(": ").unwrap();
        headers.push((name.into(), value.into()));
    }

}

fn send_content<P: AsRef<Path>>(tcp_stream: &mut TcpStream, path: P) {
    use std::io::Read;

    let mut content = Vec::new();
    std::fs::File::open(path)
        .unwrap()
        .read_to_end(&mut content)
        .unwrap();
    tcp_stream.write_all(&content).unwrap();
}

fn send_page(tcp_stream: &mut TcpStream) {
    write!(tcp_stream, "HTTP/1.0 200 OK\r\n").unwrap();
    write!(tcp_stream, "Content-Type: text/html; charset=utf-8\r\n").unwrap();
    write!(tcp_stream, "\r\n").unwrap();
    send_content(tcp_stream, "assets/index.html");
}

fn send_favicon(tcp_stream: &mut TcpStream) {
    write!(tcp_stream, "HTTP/1.0 200 OK\r\n").unwrap();
    write!(tcp_stream, "Content-Type: image/vnd.microsoft.icon\r\n").unwrap();
    write!(tcp_stream, "\r\n").unwrap();
    send_content(tcp_stream, "assets/favicon.ico");
}

fn main() {
    let localhost = Ipv4Addr::new(127, 0, 0, 1);
    let socket_addr = SocketAddrV4::new(localhost, 3000);
    eprintln!("server starts: {}", socket_addr);
    let tcp_listener = TcpListener::bind(socket_addr).unwrap();
    loop {
        let (mut tcp_stream, addr) = tcp_listener.accept().unwrap();
        let request = get_request(&mut tcp_stream);
        eprintln!("{}: {} {}", addr, request.method, request.path);
        if request.method != "GET" {
            write!(tcp_stream, "405 Method Not Allowed\r\n").unwrap();
        } else {
            match request.path.as_ref() {
                "/" => send_page(&mut tcp_stream),
                "/favicon.ico" => send_favicon(&mut tcp_stream),
                _ => write!(tcp_stream, "404 Not Found\r\n").unwrap(),
            }
        }
        tcp_stream.flush().unwrap();
    }
}