import struct
from socket import socket

LITTLE_ENDIAN = '<'
CHAR = 'c'  # 1 byte
UNSIGNED_INT = 'I'  # 4 bytes


def main():
    sock = socket()
    sock.connect(('127.0.0.1', 12345))
    print(struct.unpack(LITTLE_ENDIAN + CHAR * 2 + UNSIGNED_INT * 4, sock.recv(18)))
    print(struct.unpack(LITTLE_ENDIAN + CHAR * 2 + UNSIGNED_INT * 4, sock.recv(18)))
    print(struct.unpack(LITTLE_ENDIAN + CHAR * 2 + UNSIGNED_INT * 4, sock.recv(18)))

    sock.close()


if __name__ == "__main__":
    main()
