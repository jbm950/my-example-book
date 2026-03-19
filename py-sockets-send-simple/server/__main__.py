import struct
from socket import socket

LITTLE_ENDIAN = '<'
CHAR = 'c'  # 1 byte
UNSIGNED_INT = 'I'  # 4 bytes


def main():
    sock = socket()
    sock.bind(('127.0.0.1', 12345))
    sock.listen()

    conn, addr = sock.accept()
    print(f'Detected an incoming stream from {addr}!')

    conn.send(struct.pack(LITTLE_ENDIAN + CHAR * 2 + UNSIGNED_INT * 4,
                          b'h', b's', 1, 2, 3, 4))
    conn.send(struct.pack(LITTLE_ENDIAN + CHAR * 2 + UNSIGNED_INT * 4,
                          b'h', b's', 5, 6, 7, 8))
    conn.send(struct.pack(LITTLE_ENDIAN + CHAR * 2 + UNSIGNED_INT * 4,
                          b'h', b's', 9, 10, 11, 12))

    sock.close()


if __name__ == "__main__":
    main()
