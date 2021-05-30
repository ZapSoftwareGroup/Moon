import argparse
import moon

parser = argparse.ArgumentParser()

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
    "package", help="Select the package you wish to install/remove. e.g: urxvt")

args = parser.parse_args()


def main():
    args_dict = {"ask": args.ask, "remove": args.remove, "source": args.source,
                 "deep": args.deep, "all": args.system}
    print(moon.parse_args(args_dict, args.package))

if __name__=="__main__":
    main()
