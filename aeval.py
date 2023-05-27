from aast import AST_TYPE
from sys import exit
from abuiltins import builtins, Array, Dict
from pprint import pformat
import inspect

initial = builtins


class ReturnException(Exception):
    def __repr__(self):
        return pformat(self.args[0])


def execute(ast, scope=initial):
    kind = ast["type"]
    if kind == AST_TYPE["Func"]:

        def func(*args):
            local_scope = scope
            for i in range(0, len(ast["args"])):
                local_scope[ast["args"][i]] = args[i]
            try:
                for command in ast["body"]:
                    execute(command, local_scope)
            except ReturnException as return_value:
                return return_value.args[0]
            except Exception:
                exit(2)
            return None

        scope[ast["name"]] = func
    elif kind == AST_TYPE["Var"]:
        scope[ast["name"]] = evaluate(ast["value"], scope)
    elif kind == AST_TYPE["Return"]:
        raise ReturnException(evaluate(ast["value"], scope))
    elif kind == AST_TYPE["If"]:
        if evaluate(ast["condition"], scope):
            for command in ast["body"]:
                execute(command, scope)
        else:
            for stmt in ast["otherwise"]:
                if stmt["type"] == AST_TYPE["Else"]:
                    for command in stmt["body"]:
                        execute(command, scope)
                    break
                if evaluate(stmt["condition"], scope):
                    for command in stmt["body"]:
                        execute(command, scope)
                    break
    elif kind == AST_TYPE["While"]:
        while evaluate(ast["condition"], scope):
            for command in ast["body"]:
                execute(command, scope)
    elif kind == AST_TYPE["For"]:
        for i in range(
            int(evaluate(ast["range"][0], scope)), int(evaluate(ast["range"][1], scope))
        ):
            for command in ast["body"]:
                scope[ast["var"]["value"]] = i
                execute(command, scope)
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
    elif kind == AST_TYPE["Chain"]:
        # Run the initial method/attribute and work off that initial value
        local = scope[ast["name"]["name"]]
        if ast["chain"][0]["type"] == AST_TYPE["Call"]:
            args = []
            for arg in ast["chain"][0]["args"]:
                args.append(evaluate(arg, scope))
            local = local(*args)
        elif ast["chain"][0]["type"] == AST_TYPE["Attr"]:
            local = local._getattr(ast["chain"][0]["args"])
            if (
                len(ast["chain"]) == 1 or ast["chain"][1]["type"] != AST_TYPE["Call"]
            ) and callable(local):
                local = local()
        for i, link in enumerate(ast["chain"][1:]):
            # Go through every "link" in the "chain" and apply it to the initial value
            # i + 2 because we're using enumerate, so it starts at zero but we have a sliced list
            if link["type"] == AST_TYPE["Call"]:
                args = []
                for arg in link["args"]:
                    args.append(evaluate(arg, scope))
                local = local(*args)
            elif link["type"] == AST_TYPE["Attr"]:
                local = local._getattr(link["args"])
                if (
                    len(ast["chain"]) != i + 2
                    and ast["chain"][i + 2]["type"] != AST_TYPE["Call"]
                    and callable(local)
                ):
                    local = local()
        if local is not None:
            return local
    elif kind == AST_TYPE["Array"]:
        items = []
        for item in ast["value"]:
            items.append(evaluate(item, scope))
        return Array(items)
    elif kind == AST_TYPE["Dict"]:
        obj = {}
        for key in ast["items"].keys():
            obj[key] = evaluate(ast["items"][key], scope)
        return Dict(obj)
    elif kind == AST_TYPE["Var"]:
        if ast["name"] in scope.keys():
            return scope[ast["name"]]
        raise Exception("Variable " + ast["name"] + " does not exist")
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


def run(ast):
    new_scope = initial
    for node in ast:
        execute(node, new_scope)
