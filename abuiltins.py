from aast import AST_TYPE
from pprint import PrettyPrinter

pprinter = PrettyPrinter()


def kind(value, kind):
    if value["type"] == AST_TYPE[kind]:
        return True
    return False


class Array:
    def __init__(self, items):
        self.items = items

    def attribute(self, token, args):
        # Okay, so every default class has a attribute method that can be used to get a property, method, whatever
        if kind(token, "Number"):
            return self.items[int(token["value"])]
        elif kind(token, "String"):
            ref = getattr(self, token["value"])
            if callable(ref):
                return ref(*args)
            else:
                return ref

    def length(self):
        return len(self.items)

    def push(self, new):
        self.items.append(new)

    def __repr__(self):
        return pprinter.pformat(self.items)


def fire(*args):
    for arg in args:
        print(arg, end=" ")
    print()
