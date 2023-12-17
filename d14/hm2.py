from itertools import cycle
from tqdm.auto import trange
with open("cycle.txt", "r") as f:
    lines = f.readlines()

# for (i, v) in zip(trange(10000, 1000000000+1), cycle(lines)):
#     pass

# print(i, v)

print(lines[(1000000000 - 10000) % len(lines)])
