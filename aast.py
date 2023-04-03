AST_TYPE = {
    "Var": "Var",
    "Func": "Func",
    "Array": "Array",
    "Dict": "Dict",
    "Return": "Return",
    "If": "If",
    "Elif": "Elif",
    "Else": "Else",
    "While": "While",
    "For": "For",
    "Number": "Number",
    "String": "String",
    "Bool": "Bool",
    "BinOp": "BinOp",
    "Call": "Call",
    "Lambda": "Lambda",
}


def new_var(name, value=""):
    return {"type": AST_TYPE["Var"], "name": name, "value": value}


def new_func(name, args, body):
    return {"type": AST_TYPE["Func"], "name": name, "args": args, "body": body}


def new_call(func, args):
    return {"type": AST_TYPE["Call"], "func": func, "args": args}


def new_dict(name, items):
    return {"type": AST_TYPE["Dict"], "name": name, "items": items}


def new_array(items):
    return {"type": AST_TYPE["Array"], "value": items}


def new_return(value):
    return {"type": AST_TYPE["Return"], "value": value}


def new_if(condition, body):
    return {"type": AST_TYPE["If"], "condition": condition, "body": body}


def new_elif(condition, body):
    return {"type": AST_TYPE["Elif"], "condition": condition, "body": body}


def new_else(condition, body):
    return {"type": AST_TYPE["Else"], "condition": condition, "body": body}


def new_while(condition, body):
    return {"type": AST_TYPE["While"], "condition": condition, "body": body}


def new_for(var, through, body):
    return {"type": AST_TYPE["For"], "var": var, "range": through, "body": body}


def new_number(val):
    return {"type": AST_TYPE["Number"], "value": val}


def new_string(val):
    return {"type": AST_TYPE["String"], "value": val}


def new_bool(val):
    return {"type": AST_TYPE["Bool"], "value": val}


def new_binop(left, right, op):
    return {"type": AST_TYPE["BinOp"], "left": left, "right": right, "op": op}


def new_call(func, args):
    return {"type": AST_TYPE["Call"], "func": func, "args": args}


def new_lambda(args, body):
    return {"type": AST_TYPE["Lambda"], "args": args, "body": body}
