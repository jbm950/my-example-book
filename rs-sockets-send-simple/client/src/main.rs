use std::io::Read;
use std::net::TcpStream;


fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:4000").unwrap();

    for _ in 0..3 {
        let mut buffer = [0; 6];
        let _ = stream.read(&mut buffer);
        println!("{:?}", buffer)
    }

    Ok(())
}
