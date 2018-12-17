import os
import re


def parse_claim(claim):
    g = re.match(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)", claim)

    return (
        int(g.group(1)),
        int(g.group(2)),
        int(g.group(3)),
        int(g.group(4)),
        int(g.group(5)),
    )


def fill_matrix(matrix, id, x, y, w, h):
    for yy in range(h):
        for xx in range(w):
            matrix[x + xx][y + yy] = ((matrix[x + xx][y + yy])[0] + 1, id)


def check_claim(matrix, id, x, y, w, h):
    success = True

    for yy in range(h):
        for xx in range(w):
            success &= (matrix[x + xx][y + yy])[1] == id
            success &= (matrix[x + xx][y + yy])[0] == 1

    return success


def print_matrix(matrix):
    for x in matrix:
        print("".join([str(e[0]) if e[0] != 0 else "." for e in x]))


def run(input, size=1000):
    claims = [e.strip("\n") for e in input.readlines()]
    matrix = [[(0, None) for ee in range(size)] for e in range(size)]

    for c in claims:
        a = parse_claim(c)
        fill_matrix(matrix, *a)

    s = 0
    for y in range(size):
        for x in range(size):
            if (matrix[x][y])[0] > 1:
                s += 1

    print("= results =")

    for c in claims:
        a = parse_claim(c)
        if check_claim(matrix, *a):
            print(f"# {a}")

    print(s)


if __name__ == "__main__":
    run(open(os.path.join(os.path.dirname(__file__), "input2")), size=10)
    # run(open(os.path.join(os.path.dirname(__file__), "input")))
