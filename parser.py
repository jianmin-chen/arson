from lexer import TOKEN_TYPE

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
    
def stmt()
