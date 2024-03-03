# Rid-Lang

## What is this project?
This is a rewrite of a compiler in Rust for our custom high-level language we call Nid-lang. I got pissed at
C++ and decided to redo it in Rust.  
This is meant to be a tool to help us develop a game on our custom CPU architecture in a high-level languages instead
of machine or assembly code.

## Can I run it?
Yes! But...
You need to support our custom instructions that can be found (TODO: Link to instructionset).

## How to build.
This is partially why i switched to Rust.  
Just clone this repo and run:
```
cargo build --release
```

The binary can then be found under `target/release/`
