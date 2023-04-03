from aast import AST_TYPE
from sys import exit

initial = {"fire": lambda x: print(x)}


class ReturnException:
    def __init__(self, value):
        self.value = value


def evaluate():
    pass


def execute(ast, scope=initial):
    kind = ast["type"]
    if kind == AST_TYPE["Var"]:
        if scope.get(kind, None) is not None:
            print("Variable " + ast["name"] + " already exists")
        scope[ast["name"]] = evaluate(ast["value"], scope)
    elif kind == AST_TYPE["Func"]:
        scope[ast["name"]] = function
    elif kind == AST_TYPE["Return"]:
        raise ReturnException(evaluate(ast["value"], scope))
