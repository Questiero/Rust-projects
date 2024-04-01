# Game of Life
Implementation of Conway's Game of Life, with (hopefully) dynamic infinite space and easy first state import via file.

## Execution
To run, simply use ``cargo run``.

Implementation of the file import system will be done at a later time.

## TODO
- Load initial state from file.
- Dynamic infinite space ?
- Possibility to exit, choose the speed, pause go step by step in the UI.
- GUI ?

## Review
- Still some typing shenanigans...
- The ``print!("\x1B[2J\x1b[1;1H");`` to clear the console doesn't look great to me.
- My way to look for neighors feels hacky and can probably be optimized.
- Hardcoding ■ and □ doesn't feel right either.
- The ``#[derive(PartialEq, Eq)]`` seems insanely useful, compared to other languages "Two objects are equals only if they are the exact same object" default rule.