# TODO

## Parsing steps

- Add another AST optimization step

## Variables/Functions

- Rework naming scheme from hashing to appending names of each level donw to func or var.
- Only use `st` when variable is about to be overwritten in register, to cut down on instructions used.

## Variable storing rework

- Keep the current "push and pop" approach.
- Keep the memory map for mapping where memory is stored in the program?

## Call stack

- Implement some sort of call stack.
- Implement return register.
- Implement some sort of parameter passing.

## Function stacks

- Parse entire function at the beginning to find all variable declerations.
- Allocate entire region in memory for the function variables? (Could create seperation of variables)
- Pop entire region at the end.

## Globals

- Allocate all global variables in the beginning of the program.
- Dedicated memory map for globals?
