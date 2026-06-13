use pyo3::prelude::*;

#[pymodule]
mod rust_ext {
    use pyo3::prelude::*;

    #[pyfunction]
    fn rs_fib(n: usize) -> PyResult<usize> {
        if n <= 1 {
            return Ok(n);
        }
        return Ok(rs_fib(n - 1).unwrap() + rs_fib(n - 2).unwrap());
    }
}
