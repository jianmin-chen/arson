from sys import exit

TOKEN_TYPE = {
    "Word": "Word",
    "Number": "Number",
    "String": "String",
    "Array": "Array",
    "Dict": "Dict",
    "Func": "Func",
    "CallFunc": "CallFunc",
    "Var": "Var",
    "If": "If",
    "ElseIf": "ElseIf",
    "Else": "Else",
    "While": "While",
    "For": "For",
    "True": "True",
    "False": "False",
    "And": "And",
    "Or": "Or",
    "Return": "Return",
    "Import": "Import",
    "Export": "Export",
    "Module": "Module",
    "LeftParen": "LeftParen",
    "RightParen": "RightParen",
    "LeftBrace": "LeftBrace",
    "RightBrace": "RightBrace",
    "LeftBracket": "LeftBracket",
    "RightBracket": "RightBracket",
    "Comma": "Comma",
    "Plus": "Plus",
    "Minus": "Minus",
    "Times": "Times",
    "Divide": "Divide",
    "Equal": "Equal",
    "Equality": "Equality",
    "LessThan": "LessThan",
    "GreaterThan": "GreaterThan",
    "Eof": "Eof",
    "Class": "Class",
    "Constructor": "Constructor",
    "Self": "Self",
    "New": "New",
    "Range": "Range"
}

KEYWORDS = {
    "being": TOKEN_TYPE["Import"],
    "containing": TOKEN_TYPE["Module"],
    "burn": TOKEN_TYPE["Var"],
    "for": TOKEN_TYPE["For"],
    "through": TOKEN_TYPE["Range"],
    "prepmatch": TOKEN_TYPE["Func"],
    "while": TOKEN_TYPE["While"],
    "lightmatch": TOKEN_TYPE["CallFunc"],
    "lightertype": TOKEN_TYPE["Class"],
    "ignite": TOKEN_TYPE["Constructor"],
    "self": TOKEN_TYPE["Self"],
    "pullout": TOKEN_TYPE["New"],
    "leave": TOKEN_TYPE["Export"],
    "if": TOKEN_TYPE["If"],
    "elif": TOKEN_TYPE["ElseIf"],
    "else": TOKEN_TYPE["Else"]
}

def new_token(kind, value, content):
    return {
        "type": kind,
        "value": value,
        "content": content
    }

class Lexer:
    def __init__(self, source="", current=0, tokens=[], line=0):
        self.current = current
        self.source = source
        self.tokens = tokens
        self.line = line

    def peek(self):
        if self.current >= len(self.source): return "\0"
        return self.source[self.current]

    def peek_next(self):
        if self.current >= len(self.source): return "\0"
        return self.source[self.current + 1]

    def advance(self):
        if self.current >= len(self.source): return "\0"
        res = self.peek()
        self.current += 1
        return res

    def match(self, char):
        if self.peek() == char:
            self.advance()
            return True
        return False
    
    def add_token(self, kind, value, content):
        self.tokens.append(new_token(kind, value, content))

def scan_token(lexer):
    char = lexer.advance()

    def is_alphanumeric(char):
        return char != " " and (char.isalpha() or char.isnumeric() or char == "_")

    def string(kind):
        text = ""
        while lexer.peek() != kind and lexer.peek() != "\0":
            if lexer.peek() == "\n":
                lexer.line += 1
            text += lexer.advance()
        if lexer.peek() == "\0":
            # Reached end of file, but string hasn't been terminated
            raise Exception(f"Unterminated string: {lexer.line}")
        lexer.advance()  # Consume the closing quote
        lexer.add_token(TOKEN_TYPE["String"], text, text)

    def number():
        text = ""
        while lexer.peek().isnumeric():
            text += lexer.advance()
        if lexer.peek() == "." and lexer.peek_next().isnumeric():
            text += lexer.advance()
            while lexer.peek().isnumeric():
                text += lexer.advance()
        lexer.add_token(TOKEN_TYPE["Number"], float(text), text)

    def identifier():
        text = ""
        while is_alphanumeric(lexer.peek()):
            text += lexer.advance()
        kind = KEYWORDS.get(text, None)
        if kind is None:
            kind = TOKEN_TYPE["Word"]
        lexer.add_token(kind, text, text)

    match char:
        case "(":
            lexer.add_token(TOKEN_TYPE["LeftParen"], "(", "(")
        case ")":
            lexer.add_token(TOKEN_TYPE["RightParen"], ")", ")")
        case "{":
            lexer.add_token(TOKEN_TYPE["LeftBrace"], "{", "{")
        case "}":
            lexer.add_token(TOKEN_TYPE["RightBrace"], "}", "}")
        case "[":
            lexer.add_token(TOKEN_TYPE["LeftBracket"], "[", "[")
        case "]":
            lexer.add_token(TOKEN_TYPE["RightBracket"], "]", "]")
        case ",":
            lexer.add_token(TOKEN_TYPE["Comma"], ",", ",")
        case "+":
            lexer.add_token(TOKEN_TYPE["Plus"], "+", "+")
        case "-":
            lexer.add_token(TOKEN_TYPE["Minus"], "-", "-")
        case "*":
            lexer.add_token(TOKEN_TYPE["Times"], "*", "*")
        case "/":
            lexer.add_token(TOKEN_TYPE["Divide"], "/", "/")
        case "=":
            lexer.add_token(TOKEN_TYPE["Equal"], "=", "=")
        case '"':
            string('"')
        case "'":
            string("'")
        case "<":
            lexer.add_token(TOKEN_TYPE["LessThan"], "<", "<")
        case ">":
            lexer.add_token(TOKEN_TYPE["GreaterThan"], ">", ">")
        case "#":
            while lexer.peek() != "\n":
                lexer.advance()
            lexer.line += 1
        case "\n":
            lexer.line += 1
        case " ":
            return
        case _:
            if char.isalpha():
                lexer.current -= 1
                identifier()
            elif char.isnumeric():
                lexer.current -= 1
                number()
            else:
                raise Exception(f"Unexpected character: {char} at {lexer.line + 1}:{lexer.current + 1}")

def scan_tokens(lexer):
    # Main wrapper function
    # Let's scan the tokens one by one, and place them in lexer.tokens
    while lexer.current < len(lexer.source):
       scan_token(lexer)
    lexer.add_token(TOKEN_TYPE["Eof"], "", "")
    return lexer.tokens

