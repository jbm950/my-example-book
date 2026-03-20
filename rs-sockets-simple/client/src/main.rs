use std::net::{TcpStream};


fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:4000");

    Ok(())
}
