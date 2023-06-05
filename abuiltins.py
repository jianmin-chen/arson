from aast import AST_TYPE
from pprint import pformat
from random import randint


kind = lambda value, kind: value["type"] == AST_TYPE[kind]


class Array:
    def __init__(self, items):
        self.items = items

    def _getattr(self, token):
        if kind(token, "Number"):
            return self.items[int(token["value"])]
        elif kind(token, "Var"):
            return getattr(self, token["name"])
        raise Exception("Expected Number or Var for Attribute but got " + token["type"])

    def length(self):
        return len(self.items)

    def push(self, new):
        self.items.append(new)

    def update(self, index, value):
        self.items[int(index)] = value

    def __eq__(self, other):
        return self.items == other.items

    def __repr__(self):
        return pformat(self.items)


class Dict:
    def __init__(self, obj):
        self.obj = obj

    def _getattr(self, token):
        if kind(token, "String"):
            return self.obj[token["value"]]
        elif kind(token, "Var"):
            return getattr(self, token["name"])
        raise Exception("Expected Number or Var for Attribute but got " + token["type"])

    def update(self, key, value):
        self.obj[key] = value

    def __repr__(self):
        return pformat(self.obj)


def fire(*args):
    for arg in args:
        print(arg, end="")
    print()


def load(ask):
    # Wrapper for input
    return input(ask)


def random(min, max):
    return randint(min, max)


builtins = {
    "fire": fire,
    "load": load,
    "random": random,
    "int": int,
    "float": float,
    "str": str,
    "bool": bool,
}
