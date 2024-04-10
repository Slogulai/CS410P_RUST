use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::io::{BufRead, Write, BufReader};

fn discard_request(tcp_stream: &mut TcpStream) {
    let mut reader = BufReader::new(tcp_stream);
    let mut line = String::new();
    //let x: &str = &line; //In this example, line is immutable while the &str borrows it
    //let x: &mut str = &mut line; //This is the same as the above example, except x is now mutable and can change line
    loop {
        reader.read_line(&mut line).unwrap();
        let trimmed = line.trim_end();
        if trimmed.is_empty() {
            return;
        }
        eprintln!("{}", trimmed);
        line.clear();
    }
}

fn main() {
    let localhost = Ipv4Addr::new(131, 252, 208, 23);
    let socket_addr = SocketAddrV4::new(localhost, 6102);
    let tcp_listener = TcpListener::bind(socket_addr).unwrap();
    loop {
        let (mut tcp_stream, addr) = tcp_listener.accept().unwrap();
        eprintln!("Connection from {}", addr);
        discard_request(&mut tcp_stream);
        write!(tcp_stream, "HTTP/1.1 200 OK\r\n\r\n").unwrap();
        let body: &str = "<html><body><h1>Hello, world!</h1></body></html>";
        write!(tcp_stream, "{}", body).unwrap();
        tcp_stream.flush().unwrap();
    }
}