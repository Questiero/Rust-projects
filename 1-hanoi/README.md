# Hanoï
Simple Hanoï tower solver, using binary strategy and with a pretty-ish print.

## Execution
To run, simply use ``cargo run <n>`` with ``n`` the number of disks.

## Review
- Although it makes the moves easy-ish (could have handled errors better) with ``.pop()`` and ``.push()``, I'm not a fan of using vectors for ``Tower.rods`` and ``Rod.discs`` since the number and height of each rods are known from the initialization.
- With the binary strategy, the whole ``Tower`` and ``Rod`` shenanigans are probably way too overkill anyway, but the print seems easier this way.
- While using ``u8`` seemed like a great idea at the beginning (255 seems way too big to be run with this program anyway), I feel like the repetitive use of the ``as`` keyword might be bad practice (Probably source of error, if used wrongly) for how little it changes.
- I don't really like the ``swap_end_if_pair()`` closure, I'm sure there is a more elegant way to do that I'm not aware of.
