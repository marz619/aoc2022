#!/usr/bin/env python

import sys


def part_one(data: str) -> int:
    print(data)
    raise NotImplementedError("part_one")


def part_two(data: str) -> int:
    print(data)
    raise NotImplementedError("part_two")


def read_input() -> str:
    return sys.stdin.read().strip("\n")


def main(args) -> None:
    data = read_input()

    match a := args[0]:
        case "1":
            print(f"part_one: {part_one(data)}")
        case "2":
            print(f"part_two: {part_two(data)}")
        case _:
            raise ValueError(f"invalid argument '{a}'") 


if __name__ == "__main__":
    main(sys.argv[1:])

