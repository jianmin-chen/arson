from lexer import Lexer, scan_tokens
from sys import argv, exit

if len(argv) != 2:
    print("Usage: python arson.py <file>")
    exit(1)

with open(argv[1]) as file:
    source = file.read()
    lexer = Lexer(source)
    scan_tokens(lexer)
    print(lexer.tokens)

