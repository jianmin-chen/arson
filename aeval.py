from aast import AST_TYPE
from sys import exit
from abuiltins import fire, Array
import inspect

initial = {"fire": fire, "Array": Array}


class ReturnException(Exception):
    pass


def execute(ast, scope=initial):
    kind = ast["type"]
    if kind == AST_TYPE["Func"]:
        # This needs to return a function
        def func(*args):
            local_scope = scope
            for i in range(0, len(ast["args"])):
                local_scope[ast["args"][i]] = args[i]
            try:
                for command in ast["body"]:
                    execute(command, local_scope)
            except ReturnException as return_value:
                return return_value
            except Exception:
                exit(2)
            return None

        scope[ast["name"]] = func
    elif kind == AST_TYPE["Var"]:
        scope[ast["name"]] = evaluate(ast["value"], scope)
    elif kind == AST_TYPE["Return"]:
        raise ReturnException(evaluate(ast["value"], scope))
    elif kind == AST_TYPE["While"]:
        for command in ast["body"]:
            execute(command, scope)
    elif kind == AST_TYPE["For"]:
        for i in range(
            int(evaluate(ast["range"][0], scope)),
            int(evaluate(ast["range"][1], scope)),
        ):
            for command in ast["body"]:
                execute(command, scope)
    elif kind == AST_TYPE["If"]:
        if evaluate(ast["condition"]):
            for command in ast["body"]:
                execute(command, scope)
        else:
            for stmt in ast["otherwise"]:
                if stmt["type"] == AST_TYPE["Else"]:
                    for command in stmt["body"]:
                        execute(command, scope)
                    break
                if evaluate(stmt["condition"]):
                    for command in stmt["body"]:
                        execute(command, scope)
                    break
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
    elif kind == AST_TYPE["Call"]:
        args = []
        for arg in ast["args"]:
            args.append(evaluate(arg, scope))
        return scope[ast["func"]["name"]](*args)
    elif kind == AST_TYPE["Array"]:
        return Array(ast["value"])
    elif kind == AST_TYPE["Var"]:
        if ast["name"] in scope.keys():
            return scope[ast["name"]]
        print("Variable " + ast["name"] + " does not exist")
        exit(1)
    elif kind == AST_TYPE["BinOp"]:
        op = ast["op"]
        if op == "*":
            return evaluate(ast["left"], scope) * evaluate(ast["right"], scope)
        elif op == "/":
            return evaluate(ast["left"], scope) / evaluate(ast["right"], scope)
        elif op == "+":
            return evaluate(ast["left"], scope) + evaluate(ast["right"], scope)
        elif op == "-":
            return evaluate(ast["left"], scope) - evaluate(ast["right"], scope)
        elif op == "==":
            return evaluate(ast["left"], scope) == evaluate(ast["right"], scope)
        elif op == "<":
            return evaluate(ast["left"], scope) < evaluate(ast["right"], scope)
        elif op == "<=":
            return evaluate(ast["left"], scope) <= evaluate(ast["right"], scope)
        elif op == ">":
            return evaluate(ast["left"], scope) > evaluate(ast["right"], scope)
        elif op == ">=":
            return evaluate(ast["left"], scope) >= evaluate(ast["right"], scope)
        elif op == "and":
            return evaluate(ast["left"], scope) and evaluate(ast["right"], scope)
        elif op == "or":
            return evaluate(ast["left"], scope) or evaluate(ast["right"], scope)
