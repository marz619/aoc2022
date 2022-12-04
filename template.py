#!/usr/bin/env python

import sys


def part_one() -> None:
    data = read_input()
    print(f"{type(data)}: {data}")
    raise NotImplementedError("part_one")


def part_two() -> None:
    data = read_input()
    print(f"{type(data)}: {data}")
    raise NotImplementedError("part_two")


def read_input() -> str:
    return sys.stdin.read()


def main(args):
    match a := args[0]:
        case "1":
            return part_one() is None
        case "2":
            return part_two() is None
        case _:
            raise ValueError(f"invalid argument {c}") 


if __name__ == "__main__":
    main(sys.argv[1:])

