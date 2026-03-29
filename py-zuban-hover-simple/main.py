import json
from pathlib import Path
from subprocess import PIPE, Popen

print('hello!')  # Code returns the "hover" result for the print on this line


def read_stdout(proc):
    """Continuously read responses from the language server."""
    # Function taken from ChatGPT

    # Just doing proc.stdout.read() would freeze because it waits until the
    # pipe is closed and the pipe doesn't close. Instead you have to grab the
    # exact length of character of each message (get to the '\r\n\r\n' at the
    # end of the header, decode the 'content-length' and then grab that many
    # bytes).

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
    return proc.stdout.read(content_length).decode()


def main():
    process = Popen(['uv', 'run', 'zuban', 'server'], stdin=PIPE, stdout=PIPE, stderr=PIPE)

    initialize_request = {'jsonrpc': '2.0',
                          'id': 1,
                          'method': 'initialize',
                          'params': {
                              'processId': None,
                              'capabilities': {},
                              'workspaceFolders': None}}

    body = json.dumps(initialize_request)
    header = f'Content-Length: {len(body)}\r\n\r\n'
    message = f'{header}{body}'.encode()
    process.stdin.write(message)
    process.stdin.flush()

    read_stdout(process)  # Not printing the initialize response for this example

    initialized_notification = {
            "jsonrpc": "2.0",
            "method": "initialized",
            "params": {},
        }
    body = json.dumps(initialized_notification)
    header = f'Content-Length: {len(body)}\r\n\r\n'
    message = f'{header}{body}'.encode()
    process.stdin.write(message)
    process.stdin.flush()

    hover_request = {
        'jsonrpc': '2.0',
        'id': 2,
        'method': 'textDocument/hover',
        'params': {
            'textDocument': {'uri': Path(__file__).as_uri()},
            'position': {
                'line': 4,
                'character': 0
            }
        }
    }

    body = json.dumps(hover_request)
    header = f'Content-Length: {len(body)}\r\n\r\n'
    message = f'{header}{body}'.encode()
    process.stdin.write(message)
    process.stdin.flush()

    print('\nRaw Response')
    response = read_stdout(process)
    print(response)
    print('\nresult/content/value')
    print(json.loads(response)['result']['contents']['value'])


if __name__ == "__main__":
    main()
