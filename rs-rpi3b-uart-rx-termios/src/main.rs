use std::{
    fs::OpenOptions,
    io::{self, Read},
    os::fd::AsRawFd,
};

use libc::{B115200, TCSANOW, cfmakeraw, cfsetispeed, cfsetospeed, tcgetattr, tcsetattr};

fn main() -> io::Result<()> {
    let mut serial = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/ttyAMA0")?;

    let fd = serial.as_raw_fd();

    let mut termios = unsafe {
        let mut termios = std::mem::zeroed();

        if tcgetattr(fd, &mut termios) != 0 {
            return Err(io::Error::last_os_error());
        }

        termios
    };

    unsafe {
        cfmakeraw(&mut termios);

        if cfsetispeed(&mut termios, B115200) != 0 {
            return Err(io::Error::last_os_error());
        }

        if cfsetospeed(&mut termios, B115200) != 0 {
            return Err(io::Error::last_os_error());
        }

        if tcsetattr(fd, TCSANOW, &termios) != 0 {
            return Err(io::Error::last_os_error());
        }
    }

    let mut buffer = [0u8; 128];

    loop {
        let bytes_read = serial.read(&mut buffer)?;
        println!("Received {}", String::from_utf8_lossy(&buffer[..bytes_read]));
    }
}
