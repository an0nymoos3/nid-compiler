# Binary

## Why this file?
This file contains some information about how the assembler converts ASS instructions to binary or hex code.

## Conversion
### Bit layout
The layout in program memory that the assembler uses is as follows:  
```
| Operations | A-mode | Registers | Addresses or Constants |
| 6 bits     | 2 bits | 4 bits    | 16 bits                |
```

### Operations
This operations bit encoding is as follows:
| Operation | Binary encoding |
| --------- | --------------- |
| nop       | 000000          |
| ld        | 000001          |
| ldi       | 000010          |
| st        | 000011          |
| psh       | 000100          |
| pop       | 000101          |
| add       | 000110          |
| addi      | 000111          |
| sub       | 001000          |
| subi      | 001001          |
| cmp       | 001010          |
| cmpi      | 001011          |
| mul       | 001100          |
| muli      | 001101          |
| div       | 001110          |
| divi      | 001111          |
| and       | 010000          |
| andi      | 010001          |
| or        | 010010          |
| ori       | 010011          |
| not       | 010100          |
| xor       | 010101          |
| xori      | 010110          |
| call      | 010111          |
| ret       | 011000          |
| jmp       | 011001          |
| jmpi      | 011010          |
| beq       | 011011          |
| bne       | 011100          |
| bpr       | 011101          |
| bnr       | 011110          |
| bge       | 011111          |
| blt       | 100000          |

### A-mode
The A-mode value is not affected by the assembler process.

### Registers
The number passed in the register field is used to pass to a multiplexer to select
one of multiple general registers.

### Memory addresses & Constants
Just like with registers, the number passed in the memory/constants field is interpreted
either as a memory address or constant value depending on the instruction.
