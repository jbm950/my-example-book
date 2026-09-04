fn main() {
    let mut port = serialport::new("/dev/ttyAMA0", 115_200)
        .open()
        .expect("Failed to open port");

    let output = "OMG what a test. Crazy testing!\r\n".as_bytes();
    port.write(output).expect("Write failed!");
}
