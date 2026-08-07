mod ffi {
    unsafe extern "C" {
        pub fn counter_new(initial_value: i32) -> *mut Counter;
        pub fn counter_increment(counter: *mut Counter);
        pub fn counter_increment_by(counter: *mut Counter, amount: i32);
        pub fn counter_value(counter: *const Counter) -> i32;
        pub fn counter_delete(counter: *mut Counter);
    }

    #[repr(C)]
    pub struct Counter {
        _private: [u8; 8],
    }
}

struct Counter {
    inner: *mut ffi::Counter,
}

impl Counter {
    pub fn new(initial_value: i32) -> Self {
        // SAFETY: counter_new is a well-defined FFI call with no preconditions
        // on its arguments (initial_value is a plain i32). The returned pointer
        // is checked for null below before being stored.
        let inner = unsafe {
            ffi::counter_new(initial_value)
        };
        assert!(!inner.is_null(), "counter_new returned null");

        Self { inner }
    }

    pub fn increment(&mut self) {
        // SAFETY: self.inner is non-null (checked in `new`, never reassigned)
        // and points to a live Counter owned exclusively by this struct.
        // &mut self ensures no other Rust reference is concurrently accessing it.
        unsafe {
            ffi::counter_increment(self.inner);
        }
    }

    pub fn increment_by(&mut self, amount: i32) {
        // SAFETY: same invariants as `increment` — self.inner is non-null and
        // uniquely owned; amount is a plain i32 with no validity constraints.
        unsafe {
            ffi::counter_increment_by(self.inner, amount);
        }
    }

    pub fn value(&self) -> i32 {
        // SAFETY: self.inner is non-null and points to a live Counter.
        // counter_value only reads state, so this is safe to call through &self.
        unsafe {
            ffi::counter_value(self.inner)
        }
    }
}

impl Drop for Counter {
    fn drop(&mut self) {
        // SAFETY: self.inner is non-null and was allocated by counter_new.
        // This is the sole owner of that allocation (Counter is not Clone),
        // and drop runs at most once, so this delete cannot double-free or
        // outlive other references to the same pointer.
        unsafe {
            ffi::counter_delete(self.inner);
        }
    }
}

fn main() {
    let mut counter = Counter::new(10);

    counter.increment();
    counter.increment_by(5);

    println!("Counter: {}", counter.value());
}
