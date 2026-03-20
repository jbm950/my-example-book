use common::Position;
use std::io::Read;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:4000")?;

    for _ in 0..3 {
        let mut buffer = [0u8; 14];
        let _ = stream.read(&mut buffer);
        println!("{:?}", Position::deserialize(buffer));
    }

    Ok(())
}
