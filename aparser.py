from alexer import TOKEN_TYPE
from aast import (
    new_var,
    new_func,
    new_dict,
    new_array,
    new_return,
    new_if,
    new_elif,
    new_else,
    new_while,
    new_for,
    new_number,
    new_string,
    new_bool,
    new_binop,
    new_call,
    new_lambda,
)
from sys import exit


class Parser:
    def __init__(self, tokens):
        self.tokens = tokens
        self.current = 0

    def peek_token(self):
        if self.current >= len(self.tokens):
            return None
        return self.tokens[self.current]

    def peek_token_type(self):
        if self.current >= len(self.tokens):
            return None
        return self.tokens[self.current].get("type")

    def eat(self, type):
        if self.peek_token_type() == type:
            res = self.tokens[self.current]
            self.current += 1
            return res
        token = self.peek_token()
        raise Exception(f"Expected {type} but got {token.get('TYPE')} at {token.line}")


def simple(parser):
    # Simple values: id | number | string | true | false | func | expr
    token = parser.eat(parser.peek_token_type())
    kind = token["type"]
    if kind == TOKEN_TYPE["CallFunc"]:
        token = parser.eat(parser.peek_token_type())
        kind = token["type"]
    if kind == TOKEN_TYPE["Word"]:
        return new_var(token["value"])
    elif kind == TOKEN_TYPE["Number"]:
        return new_number(token["value"])
    elif kind == TOKEN_TYPE["String"]:
        return new_string(token["value"])
    elif kind == TOKEN_TYPE["True"]:
        return new_bool(True)
    elif kind == TOKEN_TYPE["False"]:
        return new_bool(False)
    elif kind == TOKEN_TYPE["Func"]:
        parser.eat(TOKEN_TYPE["LeftParen"])
        params = id_list(parser)
        parser.eat(TOKEN_TYPE["RightParen"])
        parser.eat(TOKEN_TYPE["LeftBrace"])
        body = []
        while parser.peek_token_type() != TOKEN_TYPE["RightBrace"]:
            body.append(stmt(parser))
        parser.eat(TOKEN_TYPE["RightBrace"])
        return new_lambda(params, body)
    elif kind == TOKEN_TYPE["LeftParen"]:
        result = expr(parser)
        parser.eat(TOKEN_TYPE["RightParen"])
        return result
    else:
        print("Expected expression but got " + token["type"])
        exit(1)


def call(parser):
    expr = simple(parser)
    if parser.peek_token_type() == TOKEN_TYPE["LeftParen"]:
        parser.eat(TOKEN_TYPE["LeftParen"])
        args = expr_list(parser)
        parser.eat(TOKEN_TYPE["RightParen"])
        return new_call(expr, args)
    return expr


def is_op(token):
    return token["type"] in [
        TOKEN_TYPE["Plus"],
        TOKEN_TYPE["Minus"],
        TOKEN_TYPE["Times"],
        TOKEN_TYPE["Divide"],
        TOKEN_TYPE["LessThan"],
        TOKEN_TYPE["GreaterThan"],
        TOKEN_TYPE["Equality"],
        TOKEN_TYPE["Equal"],
        TOKEN_TYPE["And"],
        TOKEN_TYPE["Or"],
    ]


def expr(parser):
    left = call(parser)
    if is_op(parser.peek_token()):
        op = parser.eat(parser.peek_token_type())["value"]
        right = expr(parser)
        return new_binop(left, right, op)
    return left


def var_stmt(parser):
    parser.eat(TOKEN_TYPE["Var"])
    id = parser.eat(TOKEN_TYPE["Word"])["value"]
    parser.eat(TOKEN_TYPE["Equal"])
    value = expr(parser)
    return new_var(id, value)


def id_list(parser):
    # References in a group ()
    values = []
    if parser.peek_token_type() == TOKEN_TYPE["Word"]:
        values.append(parser.eat(TOKEN_TYPE["Word"])["value"])
        while parser.peek_token_type() == TOKEN_TYPE["Comma"]:
            parser.eat(TOKEN_TYPE["Comma"])
            parser.append(parser.eat(TOKEN_TYPE["Word"])["value"])
    return values


def expr_list(parser):
    # Expressions in a group ()
    exprs = []
    if parser.peek_token_type() != TOKEN_TYPE["RightParen"]:
        exprs.append(expr(parser))
        while parser.peek_token_type() == TOKEN_TYPE["Comma"]:
            parser.eat(TOKEN_TYPE["Comma"])
            exprs.append(expr(parser))
    return exprs


def func_stmt(parser):
    parser.eat(TOKEN_TYPE["Func"])
    id = parser.eat(TOKEN_TYPE["Word"])["value"]
    parser.eat(TOKEN_TYPE["LeftParen"])
    params = id_list(parser)
    parser.eat(TOKEN_TYPE["RightParen"])
    parser.eat(TOKEN_TYPE["LeftBrace"])
    body = []
    while parser.peek_token_type() != TOKEN_TYPE["RightBrace"]:
        body.append(stmt(parser))
    parser.eat(TOKEN_TYPE["RightBrace"])
    return new_func(id, params, body)


def return_stmt(parser):
    parser.eat(TOKEN_TYPE["Return"])
    value = expr(parser)
    return new_return(value)


def if_stmt(parser):
    pass


def for_stmt(parser):
    parser.eat(TOKEN_TYPE["For"])
    id = parser.eat(TOKEN_TYPE["Word"])
    parser.eat(TOKEN_TYPE["Range"])
    parser.eat(TOKEN_TYPE["LeftParen"])
    through = expr_list(parser)
    parser.eat(TOKEN_TYPE["RightParen"])
    parser.eat(TOKEN_TYPE["LeftBrace"])
    body = []
    while parser.peek_token_type() != TOKEN_TYPE["RightBrace"]:
        body.append(stmt(parser))
    parser.eat(TOKEN_TYPE["RightBrace"])
    return new_for(id, through, body)


def while_stmt(parser):
    parser.eat(TOKEN_TYPE["While"])
    condition = expr(parser)
    parser.eat(TOKEN_TYPE["LeftBrace"])
    body = []
    while parser.peek_token_type() != TOKEN_TYPE["RightBrace"]:
        body.append(stmt(parser))
    parser.eat(TOKEN_TYPE["RightBrace"])
    return new_while(condition, body)


def stmt(parser):
    curr = parser.peek_token_type()
    if curr == TOKEN_TYPE["Var"]:
        return var_stmt(parser)
    elif curr == TOKEN_TYPE["Func"]:
        return func_stmt(parser)
    elif curr == TOKEN_TYPE["Return"]:
        return return_stmt(parser)
    elif curr == TOKEN_TYPE["For"]:
        return for_stmt(parser)
    elif curr == TOKEN_TYPE["While"]:
        return while_stmt(parser)
    else:
        return expr(parser)


def program(parser):
    parsed = []
    while parser.peek_token_type() != TOKEN_TYPE["Eof"]:
        parsed.append(stmt(parser))
    return parsed
