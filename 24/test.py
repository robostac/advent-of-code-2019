import sys

lines = sys.stdin.readlines()


def to_str(d):
    return "".join([str(d[(x, y)]) for y in range(5) for x in range(5)])


def to_str2(d, levl):
    for y in range(5):
        print("".join([str(d.get((x, y, levl), '?'))for x in range(5)]))


def nextv(d):
    nd = {}
    for k in d:
        v = d[k]
        count = 0
        for dv in [(0, 1), (1, 0), (-1, 0), (0, -1)]:

            nk = (dv[0] + k[0], dv[1] + k[1])
            if d.get(nk, '.') == '#':

                count += 1
        # print(k, count)
        if v == '#' and count != 1:
            v = '.'
        elif v == '.' and (count == 1 or count == 2):
            v = '#'
        nd[k] = v
    return nd


data = {(x, y): v for y, yv in enumerate(lines)
        for x, v in enumerate(yv.strip())}
orig_data = data.copy()
turns = 0
seen = {}
while True:
    k = to_str(data)
    if k in seen:
        break
    seen[k] = turns
    turns += 1
    data = nextv(data)


def bio_div(data):
    data = to_str(data)
    v = 0
    for p, x in enumerate(data):
        if x == '#':
            v += 2**p
    return v


print(turns, seen[k], to_str(data))
print(bio_div(data))
data = {}
for v in range(-300, 300):
    for x in range(5):
        for y in range(5):
            if (x == 2 and y == 2):
                continue
            data[(x, y, v)] = '.'
for k in orig_data:
    data[(k[0], k[1], 0)] = orig_data.get(k)


del data[(2, 2, 0)]
# print(data)


def get_neighbours(dv, level):
    level += 1
    if dv == (1, 0):
        return [(0, 0, level), (0, 1, level), (0, 2, level), (0, 3, level), (0, 4, level)]
    if dv == (-1, 0):
        return [(4, 0, level), (4, 1, level), (4, 2, level), (4, 3, level), (4, 4, level)]
    if dv == (0, 1):
        return [(0, 0, level), (1, 0, level), (2, 0, level), (3, 0, level), (4, 0, level)]
    return [(0, 4, level), (1, 4, level), (2, 4, level), (3, 4, level), (4, 4, level)]


def nextv2(d):
    nd = {}
    for k in d:
        v = d[k]
        count = 0
        for dv in [(0, 1), (1, 0), (-1, 0), (0, -1)]:
            nk = (dv[0] + k[0], dv[1] + k[1], k[2])
            if nk[0] < 0:
                nk = (1, 2, nk[2] - 1)
            if nk[1] < 0:
                nk = (2, 1, nk[2] - 1)
            if nk[0] >= 5:
                nk = (3, 2, nk[2] - 1)
            if nk[1] >= 5:
                nk = (2, 3, nk[2] - 1)
            if nk[0] == 2 and nk[1] == 2:
                for xd in get_neighbours(dv, k[2]):
                    if d.get(xd, '.') == '#':
                        count += 1

            else:
                if d.get(nk, '.') == '#':
                    count += 1

        # print(k, count)
        if v == '#' and count != 1:
            v = '.'
        elif v == '.' and (count == 1 or count == 2):
            v = '#'
        nd[k] = v
    return nd


for i in range(200):
    data = (nextv2(data))

print(len([x for x in data.values() if x == '#']))
