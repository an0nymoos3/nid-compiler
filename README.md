# NID-Compiler

## NID-Lang
The new blazingly fast high-level programming language. Giving you the simplicity of writing C code with
the performance of writing native assembly code.

## What is this project?
This is a compiler/assembler written in Rust and C++ for our custom high-level language we call NID-Lang.  
This is meant to be a tool to help us develop a game on our custom CPU architecture in a high-level languages instead
of machine or assembly code. Learn more about the languages under [docs/](https://github.com/an0nymoos3/nid-compiler/tree/assembler/docs)

## Can I run it?
Yes! But...  
You need to support our custom instructions that can be found in [docs/](https://github.com/an0nymoos3/nid-compiler/tree/assembler/docs)

## How to build.
Make sure you have Rust/Cargo installed, as well as the GCC compiler.  
Downloading and compiling can be done in one simple command: 
```
git clone https://github.com/an0nymoos3/nid-compiler.git && cd nid-compiler/ && make
```
For future builds you simply run:
```
make
```

The binaries can then be found under `bin/`, called `compiler` and `assembler`. Running `compiler` 
will automatically also call `assembler`.

## Usage:
```
./compiler my_file.nid
```
To view more options, simply run: 
```
./compiler --help
```

To compile a manually written `.ass` file you can use the `assembler` found under `bin/`.
```
./assembler my_file.ass
```

## Features
| Feature                  | Status |
| -------                  | ------ |
| Working compiler         | 🟡 Developing |
| Working assembler        | 🟡 Developing |
| Heap allocations         | 🔴 Planned    |
| Imports between files    | 🔴 Planned    |
| std library              | 🔴 Planned    |

## Contributing
Anyone with a lot of free time on their hands is free to contribute to this project. I would love to see NID-Lang
supported on more platforms.
