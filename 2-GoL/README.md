# Game of Life
Implementation of Conway's Game of Life, with (hopefully) dynamic infinite space and easy first state import via file.

## Execution
To run, simply use ``cargo run``.

Implementation of the file import system will be done at a later time.

## TODO
- Load initial state from file.
- Print of GameState with dynamic infinite space
- Possibility to exit, choose the speed, pause go step by step in the UI.
- GUI ?

## Review
- Still some typing shenanigans...
- The ``print!("\x1B[2J\x1b[1;1H");`` to clear the console doesn't look great to me.
- My way to look for neighors feels hacky and can probably be optimized.
- Hardcoding ■ and □ doesn't feel right either.
- The ``#[derive(PartialEq, Eq)]`` seems insanely useful, compared to other languages "Two objects are equals only if they are the exact same object" default rule.
- The Infinite2DMatrix was really fun to make, I love the idea behind it. I'm sure it can easily be improved with more knowledge of Rust, tho (with Iterators and whatnot, making more useful methos available to the user and probably redefine the methods it currently has implemented too. At least, it works for what I want).