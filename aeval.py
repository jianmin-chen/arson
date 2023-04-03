from aast import AST_TYPE
from sys import exit
from abuiltins import fire


initial = {"fire": fire}


class ReturnException(Exception):
    def __init__(self, message, errors, value):
        super().__init__(self, message, errors)
        self.value = value


def execute(ast, scope=initial):
    kind = ast["type"]
    if kind == AST_TYPE["Var"]:
        scope[ast["name"]] = evaluate(ast["value"], scope)
    elif kind == AST_TYPE["Func"]:
        # This needs to return a function
        def func(*args):
            local_scope = scope
            for i in range(0, len(ast["args"])):
                local_scope[ast["args"][i]] = args[i]
            try:
                for command in ast.body:
                    execute(command, local_scope)
            except ReturnException as e:
                return e["value"]
            except Exception as e:
                exit(1)
            return None

        scope[ast["name"]] = func
    elif kind == AST_TYPE["Return"]:
        raise ReturnException(evaluate(ast["value"], scope))
    elif kind == AST_TYPE["While"]:
        pass
    elif kind == AST_TYPE["For"]:
        pass
    else:
        evaluate(ast, scope)


def evaluate(ast, scope=initial):
    kind = ast["type"]
    if (
        kind == AST_TYPE["Number"]
        or kind == AST_TYPE["String"]
        or kind == AST_TYPE["Bool"]
    ):
        return ast["value"]
    elif kind == AST_TYPE["Var"]:
        if ast["name"] in scope.keys():
            return scope[ast["name"]]
        print("Variable " + ast["name"] + " does not exist")
    elif kind == AST_TYPE["BinOp"]:
        op = ast["op"]
        if op == "+":
            return evaluate(ast["left"], scope) + evaluate(ast["right"], scope)
        elif op == "-":
            return evaluate(ast["left"], scope) - evaluate(ast["right"], scope)
        elif op == "*":
            return evaluate(ast["left"], scope) * evaluate(ast["right"], scope)
        elif op == "/":
            return evaluate(ast["left"], scope) / evaluate(ast["right"], scope)
        elif op == "==":
            return evaluate(ast["left"], scope) == evaluate(ast["right"], scope)
        elif op == "<":
            return evaluate(ast["left"], scope) < evaluate(ast["right"], scope)
        elif op == "and":
            return evaluate(ast["left"], scope) and evaluate(ast["right"], scope)
        elif op == "or":
            return evaluate(ast["left"], scope) or evaluate(ast["right"], scope)
    elif kind == AST_TYPE["Call"]:
        args = []
        for arg in ast["args"]:
            args.append(evaluate(arg, scope))
        return evaluate(ast["func"], scope)(*args)
    elif kind == AST_TYPE["Lambda"]:
        pass
