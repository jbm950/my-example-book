from http.server import BaseHTTPRequestHandler, HTTPServer
import urllib.parse

PORT = 8888


class GetHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        print(f'Path {self.path}')

        parsed = urllib.parse.urlparse(self.path)
        print(f'Parsed {parsed}')
        print(f'Parsed path {parsed.path}')

        params = urllib.parse.parse_qs(parsed.query)
        print(f'Params {params}')

        self.server.code = params['code'][0]

        self.send_response(200)
        self.end_headers()
        self.wfile.write(b'Recieved GET Request')


def main():
    server = HTTPServer(('localhost', PORT), GetHandler)
    server.handle_request()

    print(f'Outside GET handler! {server.code}')


if __name__ == "__main__":
    main()

    # Paste in browser
    # http://localhost:8888/callback?code=AQA-rkQA7x-nrvnLKzy6
