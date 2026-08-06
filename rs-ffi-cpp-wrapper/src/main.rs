mod ffi {
    unsafe extern "C" {
        pub fn proj1_hello();
    }
}

/// Prints a greeting from the C++ side.
///
/// # Safety-notes
/// `hello()` is a trivial call with no preconditions, so this wrapper is safe.
pub fn say_hello() {
    unsafe { ffi::proj1_hello() }
}

fn main() {
    say_hello();
}
