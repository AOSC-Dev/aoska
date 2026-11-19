"""Serve a built ASMR v1 directory to a local Aoska desktop or browser preview."""

import argparse
from functools import partial
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path


class CatalogHandler(SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_header("Access-Control-Allow-Origin", "*")
        super().end_headers()


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("directory", type=Path)
    parser.add_argument("--port", type=int, default=8765)
    args = parser.parse_args()
    if not (args.directory / "index.json").is_file():
        parser.error("expected an ASMR dist/v1 directory containing index.json")
    handler = partial(CatalogHandler, directory=str(args.directory.resolve()))
    with ThreadingHTTPServer(("127.0.0.1", args.port), handler) as server:
        print(f"Serving {args.directory} at http://127.0.0.1:{server.server_port}/", flush=True)
        server.serve_forever()
