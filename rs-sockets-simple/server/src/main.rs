use std::net::{TcpListener, TcpStream};

fn handle_client(stream: TcpStream) {
    println!("Detected an incoming stream");
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4000")?;

    let (stream, _) = listener.accept()?;
    handle_client(stream);

    Ok(())
}
