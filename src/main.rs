use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

const PORT: u16 = 6247;
const HOST: &str = "127.0.0.1";

fn main() {
    let full_address = format!("{}:{}", HOST, PORT);
    let tcp_listener = TcpListener::bind(full_address);

    match tcp_listener {
        Ok(listener) => {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => handle_stream(stream),
                    Err(_e) => {}
                }
            }
        }
        Err(_e) => {}
    }
}

fn send_command(stream: &mut std::net::TcpStream, command: &str) {
    let command = format!("{}\n", command);
    stream
        .write(command.as_bytes())
        .expect("Failed to write to stream");
}

fn handle_stream(mut stream: TcpStream) {
    println!("request received");
    send_command(&mut stream, "ls -la /");
    let mut buffer = [0; 1024];
    stream
        .peek(&mut buffer)
        .expect("Failed to read from stream");
    let response = String::from_utf8_lossy(&buffer);
    println!("Response: {}", response);
}
