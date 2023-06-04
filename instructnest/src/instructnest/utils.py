import operator
from functools import reduce
from itertools import chain
from itertools import combinations

def compute_bitmask(bits, bigendian=True, capacity=None):
    if not bits:
        return "0x0"
    capacity = capacity or (max(bits) // 8 + 1) * 8 - 1
    return hex(sum(map(lambda x: 2 ** (capacity - x), bits)))


def dict_find(data, element):
    if not element:
        return data
    return reduce(operator.getitem, element.split('.'), data)


def dict_insert(data, key: str, value: str):
    *paths, key = key.split('.')
    for path in paths:
        if path not in data:
            data[path] = {}
        data = data[path]
    data[key] = value


def all_subsets(dataset):
    return chain(*map(lambda x: combinations(dataset, x), range(1, len(dataset)+1)))


def all_d1cluters(dataset):
    cluster = []
    for item in sorted(dataset):
        if not cluster or item - 1 in cluster or item + 1 in cluster:
            cluster.append(item)
            continue
        yield cluster
        cluster = [item]
    yield cluster
