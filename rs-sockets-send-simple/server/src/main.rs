use std::io::Write;
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    println!("Detected an incoming stream");
    stream.write_all(&[b'h', b's', 1, 2, 3, 4]).unwrap();
    stream.write_all(&[b'h', b's', 5, 6, 7, 8]).unwrap();
    stream.write_all(&[b'h', b's', 9, 10, 11, 12]).unwrap();
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4000")?;

    let (stream, _) = listener.accept()?;
    handle_client(stream);

    Ok(())
}
