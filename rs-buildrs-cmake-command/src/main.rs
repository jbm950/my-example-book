unsafe extern "C" {
    fn hello_from_c();
    fn add(a: i32, b: i32) -> i32;
}

fn main() {
    unsafe {
        hello_from_c();
        println!("2 + 3 = {}", add(2, 3));
    }
}
