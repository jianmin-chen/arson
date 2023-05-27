from aast import AST_TYPE
from pprint import PrettyPrinter
from random import randint

pprinter = PrettyPrinter()


kind = lambda value, kind: value["type"] == AST_TYPE[kind]


class Array:
    def __init__(self, items):
        self.items = items

    def _getattr(self, token):
        if kind(token, "Number"):
            return self.items[int(token["value"])]
        elif kind(token, "Var"):
            return getattr(self, token["name"])
        raise Exception(
            "Expected Number or Word for Attribute but got " + token["type"]
        )

    def length(self):
        return len(self.items)

    def push(self, new):
        self.items.append(new)

    def update(self, index, value):
        self.items[int(index)] = value

    def __repr__(self):
        return pprinter.pformat(self.items)


def fire(*args):
    for arg in args:
        print(arg, end=" ")
    print()


def load(ask):
    # Wrapper for input
    return input(ask)


def random(min, max):
    return randint(min, max)


builtins = {"fire": fire, "load": load, "random": random}
