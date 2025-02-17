# NID IR

This document describes the intermediate representation (IR) that the NID compiler uses.
This is mostly here for development purposes.

## General structure
The NID compiler IR is structured in a pseudo code style. It converts the AST into a series of steps 
that are required to accomplish tasks. These steps are structed in a linear fashion, similar to 
procedural code.

## Definitions
### Basic terminology
 - IR - [Intermediate Representation](https://en.wikipedia.org/wiki/Intermediate_representation). The compiler specific data structure/ language used to get a better understanding of the 
 execution of the program.
 - TAC - [Three Address Code](https://en.wikipedia.org/wiki/Three-address_code). A common method for implementing IR.
 - [Backpatching](https://www.geeksforgeeks.org/backpatching-in-compiler-design/) - A method for handling more complex conditions, jumps instructions in IR.
 - Basic Blocks - Chunks of IR that are always run together. IR that is in between goto statements or conditions. Enables jumps to blocks
 rather than line numbers, which allows the compiler to change the order of instructions inside the basic block without causing 
 side effects for code outside the block.
 - [Flow graph](https://www.geeksforgeeks.org/flow-graph-in-code-generation/) - A data structure used for implementing IR with basic blocks.
 - Linked list - Data structure. Alternative way of storing multiple elements, where each element can point to the element before it, 
 after it or in both directions. Can be useful when implementing flow graphs.
 - Stack machine code - Pseudo code, used as a type of IR. Can be used to view the order of execution of a program. Instruction order 
 optimizations can then be done on it.
 - [Symbol table](https://en.wikipedia.org/wiki/Symbol_table) - A data structure used to verify the type safety of a program. Organizes identifiers by types and scopes, then cross 
 examines the types with each other.

### NID IR Generation
The working idea behind how the NID compiler generates IR is using multiple steps/approaches. 

**First** the compiler uses the AST to create a `symbol table`.  
**Secondly** the compiler converts the AST into a type of `stack machine code`. Basically a pseudo assembly code.  
**Third** it creates `basic blocks`.  

Once the steps above have been done the compiler can start optimizing the code. Some of these optimizations include 
reordering instructions, as long as they don't affect the behaviour of the program, performing certain calculations 
or instructions during compile time and more.

### Keywords (Stack machine code)

| Instruction         | Description                                                                                                          |
|---------------------|----------------------------------------------------------------------------------------------------------------------|
| call                | Function call                                                                                                        |
| goto                | Jump instruction                                                                                                     |
| iff                 | Short for "if false", a comparison, used to get rid of redundant gotos                                               |
| ptr                 | Memory address                                                                                                       |
| *type*              | A primitive type, such as int, float, char, etc...                                                                   |
| define              | Function decleration                                                                                                 |
| declare             | Variable decleration                                                                                                 |
| :=                  | Assignment operator                                                                                                  |
| +,-,*,/             | Arithmetic operations                                                                                                |
