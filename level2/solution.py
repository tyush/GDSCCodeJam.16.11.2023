#!/usr/bin/env python3

from sys import argv
import itertools

get = lambda arr, x, d: arr[x] if x < len(arr) else d()

#               \/ hacky, i want devspeed
arr = get(argv, 1, lambda: eval(input("numbers [\\d+,+] > ")))
arr = [int(x) for x in arr]
target = get(argv, 2, lambda: int(input("target sum > ")))

triplets = set([])
for x, y, z in itertools.combinations(arr, 3):
    if x + y + z == target:
        triplets.add((x, y, z))

if len(triplets) > 0:
    print(f"found triplets that sum to {target}: {triplets}")
else:
    print(f"found no triplets that sum to {target}")
