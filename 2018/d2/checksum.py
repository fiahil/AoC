f = open("/Users/leffray quentin/AoC/d2/input")
x = f.readlines()

x = [e.strip("\n") for e in x]

import Levenshtein
from collections import defaultdict

distances = defaultdict(list)

for e in x:
    for ee in x:
        distances[Levenshtein.distance(e, ee)].append((e, ee))

print(distances[1])

p = distances[1][0]

