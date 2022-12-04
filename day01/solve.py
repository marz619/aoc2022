#!/usr/bin/env python

import sys


def parse_inventories(inventories: str) -> list[list[int]]:
    """
    returns a list of list of ints

    e.g. input:  '1\n\n1\n2\n\n3\n\n4\n5\n6'
         output: [[1], [1, 2], [3], [4, 5, 6]]

    >>> parse_inventories("1\\n\\n1\\n2\\n\\n3\\n\\n4\\n5\\n6")
    [[1], [1, 2], [3], [4, 5, 6]]
    """
    return list(
        list(map(int, inventory.split("\n")))       # map to int
        for inventory in inventories.split("\n\n")  # split on double lines
    )


def part_one(data: str) -> int:
    """
    return the largest sum 
    """
    return max(sum(inventory) for inventory in parse_inventories(data))


def part_two(data: str) -> int:
    """
    return the sum of the 3 largest sums
    """
    return sum(sorted((sum(inventory) for inventory in parse_inventories(data)), reverse=True)[:3])


def read_input() -> str:
    """
    reads and returns the input from stdin
    """
    return sys.stdin.read().strip("\n")


def main(args):
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

