import random
import sys

def distance(s1, s2):
    dist = 0
    for i in range(len(s1)):
        if s1[i] != s2[i]:
            dist += 1
    return dist

def max_dist(s, strings):
    max = 0
    for ss in strings:
        dist = distance(s,ss)
        max = dist if dist > max else max
    return max

def better_random_strings(n, lon, dist, alph):
    strings = {}
    center = random_string(lon, alph)
    strings[center] = center
    while len(strings) < n:
        new_string = mutate(center, dist, alph)
        if max_dist(new_string, strings) <= dist:
            strings[new_string] = new_string
    return strings

def repeat(n, f):
    l = []
    for i in range(n):
        l.append(f())
    return l

def mutate(s, chars, alph):
    indeces = repeat(chars, (lambda: random.choice(range(len(s)))))
    copy = list(s[:])
    for i in indeces:
        copy[i] = random.choice(alph)
    return "".join(copy)


def random_string(lon, alph):
    s = []
    for i in range(lon):
        s.append(random.choice(alph))
    return "".join(s)

strings = better_random_strings(int(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3]), sys.argv[4])
max_distance = 0
for s in strings:
    dist = max_dist(s, strings)
    max_distance = dist if dist > max_distance else max_distance

for k in strings:
    print(k)
print("La distancia maxima entre las cadenas es: {}".format(max_distance))
