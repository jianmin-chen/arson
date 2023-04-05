from pprint import PrettyPrinter

pprinter = PrettyPrinter()


class Array:
    def __init__(self, items):
        self.items = items

    def push(self, new):
        self.items.append(new)

    def __repr__(self):
        repr = []
        for item in self.items:
            repr.append(item["value"])
        return pprinter.pformat(repr)


def fire(*args):
    for arg in args:
        print(arg)
