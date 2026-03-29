import json
from subprocess import PIPE, Popen
import threading


def read_stdout(proc):
    """Continuously read responses from the language server."""
    # Function taken from ChatGPT

    # Just doing proc.stdout.read() would freeze because it waits until the
    # pipe is closed and the pipe doesn't close. Instead you have to grab the
    # exact length of character of each message (get to the '\r\n\r\n' at the
    # end of the header, decode the 'content-length' and then grab that many
    # bytes).
    while True:
        # Read headers
        headers = b''
        while b'\r\n\r\n' not in headers:
            chunk = proc.stdout.read(1)
            if not chunk:
                return
            headers += chunk

        header_text = headers.decode()
        content_length = 0

        for line in header_text.split('\r\n'):
            if line.lower().startswith('content-length:'):
                content_length = int(line.split(':')[1].strip())

        # Read body
        body = proc.stdout.read(content_length)
        print('<<', body.decode())


def main():
    process = Popen(['uv', 'run', 'zuban', 'server'], stdin=PIPE, stdout=PIPE, stderr=PIPE)

    # https://www.jsonrpc.org/specification
    # https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#initialize
    initialize_request = {'jsonrpc': '2.0',  # Top level is JSONRPC specification
                          'id': 1,
                          'method': 'initialize',
                          'params': {  # params are InitializeParams from LSP specification
                              'processId': None,
                              'capabilities': {},
                              'workspaceFolders': None}}

    threading.Thread(target=read_stdout, args=(process,), daemon=True).start()

    body = json.dumps(initialize_request)
    header = f'Content-Length: {len(body)}\r\n\r\n'
    message = f'{header}{body}'.encode()
    process.stdin.write(message)
    process.stdin.flush()

    input('Press enter when done!\n')


if __name__ == "__main__":
    main()
