from socket import socket

def main():
    sock = socket()
    sock.bind(('127.0.0.1', 12345))
    sock.listen()

    sock.accept()
    print('Detected an incoming stream!')


if __name__ == "__main__":
    main()
