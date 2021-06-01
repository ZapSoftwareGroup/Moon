import argparse
from add_repository import Repository
import moon

parser = argparse.ArgumentParser(
    prog="moon Package Manager", description="A Hackable Package Manager in Rust and Python")

parser.add_argument("-a", "--ask", help="Display confirmation for command",
                    action="store_true", default=False)
parser.add_argument("-r", "--remove", help="Delete selected package",
                    action="store_true", default=False)
parser.add_argument("-s", "--source", help="Delete original source code. Ex: moon --ask -s urxvt",
                    action="store_true", default=False)
parser.add_argument(
    "-d", "--deep", help="Select all dependencies. Ex: moon --ask --system -d or moon --ask -d urxvt", action="store_true")
parser.add_argument(
    "--system", help="Select entire system for updating.", action="store_true")
parser.add_argument(
    "--sync", help="Sync all downloaded repos", action="store_true")
parser.add_argument(
    "-c", "--category", action="store", help="Speed up file indexing by adding category of package")
parser.add_argument(
    "-v", "--version", action="store", help="Install a specific version of a package")
parser.add_argument(
    "package", help="Select the package you wish to install/remove. e.g: urxvt")

args = parser.parse_args()


def main():
    if not args.version:
        args.version = "latest"
    bool_args_dict = {"ask": args.ask, "remove": args.remove, "source": args.source,
                 "deep": args.deep, "all": args.system, "sync": args.sync}
    string_args_dict = {"version": args.version}
    return moon.parse_args(bool_args_dict, string_args_dict, args.package)


if __name__ == "__main__":
    print(main())
