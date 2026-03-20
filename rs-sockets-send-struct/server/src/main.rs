use common::Position;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    println!("Detected an incoming stream");

    let pos = Position {
        x: 3.2,
        y: 1.2,
        z: 2.5,
    };
    println!("{:?}", pos);
    let _ = stream.write(&pos.serialize());

    let pos = Position {
        x: 15.2,
        y: 8.3,
        z: 5.4,
    };
    println!("{:?}", pos);
    let _ = stream.write(&pos.serialize());

    let pos = Position {
        x: 11.1,
        y: 14.4,
        z: 22.2,
    };
    println!("{:?}", pos);
    let _ = stream.write(&pos.serialize());
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4000")?;

    let (stream, _) = listener.accept()?;
    handle_client(stream);

    Ok(())
}
