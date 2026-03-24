from http.server import SimpleHTTPRequestHandler
import socketserver

PORT = 8000


def main():
    handler = SimpleHTTPRequestHandler

    with socketserver.TCPServer(('', PORT), handler) as httpd:
        print(f'Serving on port {PORT}')
        httpd.serve_forever()


if __name__ == "__main__":
    main()
