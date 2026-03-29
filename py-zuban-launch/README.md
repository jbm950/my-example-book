As simple an example as I could get to initialize the Zuban language server.
Some key takeaways:
* The top level message format is the JSONRPC specification.
* The LSP specification defines the 'params' field of the JSONRPC message
* The output pipe stays open so you have to read specific bytes rather than
  just full `stdout.read()` calls.


JSONRPC: https://www.jsonrpc.org/specification
LSP Spec: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/
