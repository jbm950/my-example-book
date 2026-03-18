def add(x, y):
    return x + y


def divide(x, y):
    if y == 0:
        return ZeroDivisionError
    return x / y


def fibonacci(n):
    a, b = 0, 1
    for _ in range(n):
        a, b = b, a + b
    return a
