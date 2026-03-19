from socket import socket


def main():
    sock = socket()
    sock.connect(('127.0.0.1', 12345))


if __name__ == "__main__":
    main()
