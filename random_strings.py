import string
import random
import sys

def random_strings(n, lon, alf):
    s = {}
    while len(s) < n:
        string = []
        for i in range(lon):
            string.append(random.choice(alf))
        s["".join(string)] = "".join(string)
    return s

strs = random_strings(int(sys.argv[1]), int(sys.argv[2]), sys.argv[3])
for s in strs:
    print(s)
