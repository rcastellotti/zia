from jinja2 import jinja2_template
from datetime import datetime
from pathlib import Path
from zia import Zia
import argparse
import os
import http.server
import socketserver


def run_http_server(directory="reports", port=8000):
    """
    Starts a simple HTTP server to serve files from the given directory.

    :param directory: Directory to serve (default is 'reports')
    :param port: Port to listen on (default is 8000)
    """
    os.chdir(directory)  # Change the current working directory to the desired directory

    with socketserver.TCPServer(
        ("", port), http.server.SimpleHTTPRequestHandler
    ) as httpd:
        print(f"Serving {directory} at http://localhost:{port}...")
        httpd.serve_forever()


def main():
    parser = argparse.ArgumentParser(description="zia")
    subparsers = parser.add_subparsers(dest="command", help="Available commands")

    rp = subparsers.add_parser("run", help="Run zia")
    rp.add_argument("--html", action="store_true", help="Save html report")
    rp.add_argument("--json", action="store_true", help="Save json report")

    sp = subparsers.add_parser("serve", help="Serve results from results folder")
    sp.add_argument("--dir", action="store_true", help="set reports directory")

    args = parser.parse_args()

    if args.command == "run":
        repdir = Path(os.getcwd()) / "reports"
        timestamp = datetime.now().strftime("%Y-%m-%d-%H:%M:%S")

        z = Zia()
        z.load_config("config.toml")
        results = z.results()

        if args.html:
            fi = repdir / f"zia-report-{timestamp}.html"
            print(f"Saving HTML report to {fi}")

            data = {
                "slugify": lambda text: "-".join(text.split()),
                "col": lambda text: "success" if text == 0 else "failure",
                "timestamp": timestamp,
                "results": results,
            }
            report = jinja2_template("index.html.jinja2", **data)
            os.makedirs(os.path.dirname(fi), exist_ok=True)
            with fi.open("w+") as f:
                f.write(report)

        elif args.json:
            print(f"Saving JSON report to {repdir}")
            with Path(repdir) / f"zia-report-{timestamp}.json".open("w") as f:
                f.write(results)

    elif args.command == "serve":
        run_http_server("reports", 9172)

    else:
        parser.print_help()


if __name__ == "__main__":
    main()
