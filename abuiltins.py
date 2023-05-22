from aast import AST_TYPE
from pprint import PrettyPrinter

pprinter = PrettyPrinter()


def kind(value, kind):
    if value["type"] == AST_TYPE[kind]:
        return True
    return False


class Class:
    def __init__(self):
        pass


class Dict:
    def __init__(self, obj):
        self.obj = obj

    def attribute(self, token, args):
        if kind(token, "String"):
            # Check if key is actually in self.obj
            if token["value"] in self.obj:
                return self.obj[token["value"]]
            raise Exception(token["value"] + " is invalid key")
        elif kind(token, "Word"):
            ref = getattr(self, token["value"])
            if callable(ref):
                return ref(*args)
            return ref
        raise Exception("Expected String or Word but got " + token["type"])

    def __repr__(self):
        return pprinter.pformat(self.obj)


class Array:
    def __init__(self, items):
        self.items = items

    def attribute(self, token, args):
        # Okay, so every default class has a attribute method that can be used to get a property, method, whatever
        if kind(token, "Number"):
            return self.items[int(token["value"])]
        elif kind(token, "Word"):
            ref = getattr(self, token["value"])
            if callable(ref):
                return ref(*args)
            return ref
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


def load(kind, store):
    # Wrapper for input
    pass
