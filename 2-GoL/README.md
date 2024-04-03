# Game of Life
Implementation of Conway's Game of Life, with (hopefully) dynamic infinite space and easy first state import via file.

## Execution
To run, simply use ``cargo run <FILE>``, with <FILE> the path to a .gol state file.
A state file is a succession of characters, where ``_`` is considered a dead cell and every other character is considered alive.
Example state files are ``presets/gun.gol`` and ``presets/R-pentomino.gol``.

## Review
- Still some typing shenanigans...
- My way to look for neighors feels hacky and can probably be optimized.
- The ``#[derive(PartialEq, Eq)]`` seems insanely useful, compared to other languages "Two objects are equals only if they are the exact same object" default rule.
- The Infinite2DMatrix was really fun to make, I love the idea behind it. I'm sure it can easily be improved with more knowledge of Rust, tho (with Iterators and whatnot, making more useful methos available to the user and probably redefine the methods it currently has implemented too. At least, it works for what I want). Also, an ``HashMap<(isize, isize), T>`` is probably more efficient than a ``HashMap<isize, HashMap<isize, T>>``, although the search in the later is probably fastest. I can't tell if the time efficiency is worth the trade-off, though.
- Ratatui is alright, a bit confusing at first but I like it. I don't fully understand it yet tho, I should probably do another project with it from scratch to fully understand how it works.
- Keyboard events are hard to handle with how fast they come, I should learn how to do it better.
- The error handling for arguments can probably be better, but it works.