## What's this?

A programming language built for fun (no speed optimizations, probably has bugs, etc.)... around arson. (It's a joke though. Seriously).

Usage: `python3 arson.py <program>.ars` (yes, I know, `ars`, haha). I'm learning more about how to package Python files as executables (`pyinstaller`?)

**Make sure you have the latest version of Python! I'm using the lovely new keyword `match`.**

## Syntax

`/examples/readme/example.ars`

```ars
# This is a comment. 
bring "example2.ars" containing location  # Importing a module

burn x 1  # Variable
for i through (0, 10) {
    burn x x + 1  # No prefix notation currently
}

prepmatch countdown(num) {
    # This is a function. Function declarations start with "prep match".
    while (num > 0) {
        lightmatch fire(num)  # fire = "print" in every other language ever; fire is still a built-in function so you still have to use "lightmatch"
        burn num num - 1
    }
}

lightmatch countdown(x)  # Countdown back to 0

lightertype Flamethrower {
    prepmatch ignite(type, desc) {
        # "ignite" is the constructor method
        self.type = type
        self.description = desc
    }

    prepmatch use() {
        fire("In use...")
        fire("""
            TODO: Some random fire ASCII art
        """)
    }
}

burn flamethrower pullout Flamethrower("DIY", "Your favorite unreliable flamethrower")
flamethrower.use()  # => In use...

# Using the module from the beginning
# Arrays work
# This is probably an actual place, whoopsies
burn placeToBurn location([43.55, 42.55]) 
fire(placeToBurn)

# What about dictionaries
burn todo {
    "1": "Burn",
    "2": "Clean",
    "3": "Escape"
}
fire(todo{"1"})  # => Burn
burn todo{"2"} "Vandalize"
fire(todo{"2"})  # => Vandalize

# Module exports
leave countdown, Flamethrower
```

`example2.ars`

```ars
prepmatch location(coords) {
    if coords{0} == 43.55 && coords{1} == 42.55 {
        return coords
    } else {
        return [0, 0]
    }
}

leave location
```

Granted, some names don't make sense technically. But all of them revolve around being a pyromaniac, so +1 for that in my humble opinion.

## How it works

1. Source
2. Lexer creates tokens based on source
3. Parser creates AST based on tokens
4. Evaluator runs the AST, accounting for scope.

I might write a more detailed README later.

## Quirks I noticed working with Python

This is coming from someone who's worked primarily with JavaScript

* I can't apply it to a reference value, e.g. `TOKEN_TYPE["Word"]`, when using `match`. Don't know if this is intentional or not.
* Had to rename my Python files to start with `a` because they were interfering with actual Python files, e.g. `ast.py`.