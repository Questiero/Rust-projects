# Brainfuck
Simple Brainfuck interpretor.

## Execution
To run, simply use ``cargo run <FILE>``, with <FILE> the path to a brainfuck file.

## TODO
- Implement the ``,`` instruction.
- Optimization (?)
- Read instructions from a file

## Review
- Not really a fan of how I find the ``[`` and ``]`` for loops, but I don't really see another way right now.
- Implementing the ``,`` input instruction seems messy
- Is that much methods for the ``Stack`` struct good practice ? I thought it was great to add most functionnalities to it instead of cluttering the ``match`` statement, but I'm wondering if it was a good idea.
-