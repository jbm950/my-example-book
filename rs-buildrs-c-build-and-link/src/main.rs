unsafe extern "C" {
    fn hello_from_c();
}

fn main() {
    unsafe {
        hello_from_c();
    }
}
