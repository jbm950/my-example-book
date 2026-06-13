import time

import rust_ext

def py_fib(n):
    if n <= 1:
        return n

    return py_fib(n - 1) + py_fib(n - 2)


def main():
    n = 38
    start = time.time()
    py_val = py_fib(n)
    print(f'Took {time.time() - start} seconds')

    start = time.time()
    rs_val = rust_ext.rs_fib(n)
    print(f'Took {time.time() - start} seconds')

    print(f'Python calculated {py_val}')
    print(f'Rust calculated {rs_val}')


if __name__ == "__main__":
    main()
