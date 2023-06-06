## What's this?

A programming language built for fun (no speed optimizations, probably has bugs, etc.)... around arson. (It's a joke though. Seriously.)

Usage: `python3 arson.py <program>.ars` (yes, I know, `ars`, haha).

**Make sure you have the latest version of Python! I'm using the lovely new keyword `match`.**

## Syntax

`/examples/readme/example.ars`:

```
burn x = 1  # Variable
for i through (0, 10) {
    burn x = x + 1  # No prefix notation currently
}

burn time = True
fire(time)  # => True

prepmatch countdown(num) {
    # This is a function. Function declarations start with "prepmatch".
    while (num > 0) {
        fire(num)  # fire = "print" in every other language
        burn num = num - 2
    }
}

countdown(x)  # Countdown back to 0

burn coords = [43.55, 42.55, "hi"]
fire(coords)

prepmatch power(num, exp) {
    # Basic power function
    burn res = 1
    for i through (0, exp) {
        burn res = res * num
    }
    return res
}

burn y = power(8, 3)
fire(y)
fire(power(8, 2) + (19 - 8) * (10 + 4))

prepmatch min(a, b) {
    if (a < b) {
        return a
    } else {
        return b
    }
}

fire(min(2, -1))

burn coords = [43.55, 42.55]
fire(coords[.length])
fire(coords[1])

for i through (0, coords[.length]) {
    fire("index: ", i)
    fire(coords[i])
}

coords[.push](2 * 3)
fire(coords)
coords[.push](power(8, 2) + (19 - 8) * (10 + 4))
fire(coords)
coords[.push](min)
fire(coords)
coords[.push]([1, 2])
fire(coords)

coords[.update](1, "Hello world!")
coords[5][.update](1, coords[5][1] * 2)
fire(coords)

# What about dictionaries
burn todo = {
    "1": "Burn",
    "2": "Clean",
    "3": "Escape"
}
fire(todo)
fire(todo["1"])  # => Burn
todo[.update]("2", [1, 2, {"a": "foo", "b": "bar"}])
fire("Updated: ", todo)
fire(todo["2"][2]["b"])

burn test = [1, 1]
fire(test[0] == test[1])
```

Granted, some names don't make sense technically. But all of them revolve around being a pyromaniac, so +1 for that in my humble opinion.

### Also, it's extensible! 

Okay, what I mean is that you can create your own files of "builtins", I suppose, that you write in Python. For example, this is a snippet from `abuiltins.py`, which contains the code for the class `Dict`:

```python
class Dict:
    def __init__(self, obj):
        self.obj = obj

    def _getattr(self, token):
        """
            Every builtin class has a _getattr "private" method.
        """
        if kind(token, "String"):
            return self.obj[token["value"]]
        elif kind(token, "Var"):
            return getattr(self, token["name"])
        raise Exception("Expected Number or Var for Attribute but got " + token["type"])

    def update(self, key, value):
        self.obj[key] = value

    def __repr__(self):
        """
            And a __repr__ method of course.
        """
        return pformat(self.obj)
```

## How it works

1. Source
2. Lexer creates tokens based on source
3. Parser creates AST based on tokens
4. Evaluator runs the AST, accounting for scope

## Quirks I noticed with working with Python

This is coming from someone who's worked primarily with JavaScript. (Although Python was my first language.)

* I can't apply it to a reference value, e.g. `TOKEN_TYPE["Word"]`, when using `match`.
* Had to rename my Python files to start with `a` because they were interfering with actual Python files, e.g. `ast.py`.
