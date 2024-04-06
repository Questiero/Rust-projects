# Brainfuck
Simple Brainfuck interpretor.

## Execution
To run, simply use ``cargo run <FILE>``, with <FILE> the path to a brainfuck file.

## TODO
- Optimization (?)

## Review
- Not really a fan of how I find the ``[`` and ``]`` for loops, but I don't really see another way right now.
- Implementing the ``,`` input instruction seems messy
- Is that much methods for the ``Stack`` struct good practice ? I thought it was great to add most functionnalities to it instead of cluttering the ``match`` statement, but I'm wondering if it was a good idea.
- Often times, I get an overflow while trying online programs. My guess is that it's a limitation of brainfuck (My version is "faithful", with u8 for the cell value and 30000 cells maximum, I'm guessing others might not be to allow for more fun possibilites) and I won't check for bugs since I'm lazy, though.