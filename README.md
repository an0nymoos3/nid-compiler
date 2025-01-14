# NID-Compiler

## NID-Lang
The new blazingly fast high-level programming language. Giving you the simplicity of writing C code with
the performance of writing native assembly code.

## What is this project?
This is a compiler/assembler written in Rust and C++ for our custom high-level language we call NID-Lang.  
This is meant to be a tool to help us develop a game on our custom CPU architecture in a high-level languages instead
of machine or assembly code. Learn more about the languages under [docs/](https://github.com/an0nymoos3/nid-compiler/tree/main/docs)

## Can I run it?
Yes! But...  
You need to support our custom instructions that can be found in [docs/](https://github.com/an0nymoos3/nid-compiler/tree/main/docs)
Alternatively, you can use the compiler and write your own assembler that's compatible with our assembly code,
but allows you to extend some functionality by adding custom instructions, and so on.

## How to build.
Make sure you have Rust/Cargo installed.  
Downloading and compiling can be done in one simple command: 
```
git clone https://github.com/an0nymoos3/nid-compiler.git && cd nid-compiler/ && cargo build --release
```
If you ever have to rebuild, it can be done with the following command:
```
cargo build --release
```

The binaries can then be found under `targer/release/`, called `nidc`. Running `nidc` 
will both compile to ASS and assemble the ASS to binary, outputting 2 files, one `.ass` and one `.out`.

## Usage:
```
./nidc my_file.nid
```
To view more options, simply run: 
```
./nidc --help
```

If you don't have the same number of registers or memory addresses as the reference
CPU that this compiler was built for, you can specify it in a .toml file.  
Below is an example of such a file.
`custom_hardware.toml`
```
mem_addresses = 512
registers = 16
extended_instructions = false
```
To use this file for compilation, you simple add the `--hardware-conf` flag.
```
./nidc my_file.nid --hardware-conf custom_hardware.toml
```

## Features
| Feature                      | Status |
| -------                      | ------ |
| Working compiler             | 🟢 Working state, missing features  |
| Working assembler            | 🟢 Mostly done                      |
| Dynamic memory allocations   | 🔴 Planned                          |
| Imports between files        | 🔴 Planned                          |
| std library                  | 🔴 Planned                          |

## Contributing
Anyone with a lot of free time on their hands is free to contribute to this project. I would love to see NID-Lang
supported on more platforms and projects.
