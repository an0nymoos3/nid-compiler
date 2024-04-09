# Nid-Compiler

## Nid-Lang 
The new blazingly fast high-level programming language. Giving you the simplicity of writing C code with
the performance of writing native assembly code. Work with you favourite data structures like, `int`, `struct`, `enum`,
`pointer` and more!

## What is this project?
This is a compiler/assembler written in Rust and C++ for our custom high-level language we call Nid-Lang.  
This is meant to be a tool to help us develop a game on our custom CPU architecture in a high-level languages instead
of machine or assembly code.

## Can I run it?
Yes! But...  
You need to support our custom instructions that can be found (TODO: Link to instructionset).

## How to build.
Make sure you have Rust/Cargo installed.  
Downloading and compiling can be done in one simple command: 
```
git clone https://github.com/an0nymoos3/nid-compiler.git && cd nid-compiler/ && make build
```
For future builds you simply run:
```
make build
```

The binaries can then be found under `bin/`, called `compiler` and `assembler`. Running `compiler` 
will automatically also call `assembler`.

## Usage:
```
./nid-compiler my_file.nid
```
To view more options, simply run: 
```
./nid-compiler --help
```

## Features
| Feature                  | Status |
| -------                  | ------ |
| Working compiler         | 🟡 Developing |
| Working assembler        | 🟡 Developing |
| Heap allocations         | 🔴 Planned    |
| Imports between files    | 🔴 Planned    |
| std library              | 🔴 Planned    |

## Syntax
### Reserved keywords
| Keyword | Meaning  |
| ------- | -------- |
| void    | No type.                      |
| int     | 16 bit integer.               |
| float   | 16 bit floating point.        |
| string  | Char array type.              |
| char    | Single character type.        |
| if      | If-statments.                 |
| else    | Else condition.               |
| while   | Basic conditional while loop. |
| return  | Return instruction.           |
| asm     | Inline assembly code.         |

## Contributing
Anyone dumb enough is free to contribute to this project. I would love to see Nid-Lang
supported on more platforms.
