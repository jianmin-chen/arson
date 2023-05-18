## What's this?

A programming language built for fun (no speed optimizations, probably has bugs, etc.)... around arson. (It's a joke though. Seriously.)

Usage: `python3 arson.py <program>.ars` (yes, I know, `ars`, haha). You can also check out the [online playground](https://arson.jianminchen.com).

**Make sure you have the latest version of Python! I'm using the lovely new keyword `match`.**

## Syntax

`/examples/readme/example.ars`

Granted, some names don't make sense technically. But all of them revolve around being a pyromaniac, so +1 for that in my humble opinion.

## How it works

1. Source
2. Lexer creates tokens based on source
3. Parser creates AST based on tokens
4. Evaluator runs the AST, accounting for scope

## Quirks I noticed with working with Python

This is coming from someone who's worked primarily with JavaScript. (Although Python was my first language.)

* I can't apply it to a reference value, e.g. `TOKEN_TYPE["Word"]`, when using `match`.
* Had to rename my Python files to start with `a` because they were interfering with actual Python files, e.g. `ast.py`.
