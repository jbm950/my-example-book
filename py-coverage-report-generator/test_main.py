from main import add, divide, fibonacci

def test_add():
    assert add(1, 3) == 4


def test_divide():
    assert divide(3, 4) == 3 / 4


def test_fibonacci():
    assert fibonacci(0) == 0
