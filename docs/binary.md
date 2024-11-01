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

### A-mode
The A-modes are encoded as follows:
| A-mode | Binary encoding |
| ------ | --------------- |
| 

### Registers
The registers are encoded as follows:


### Memory addresses & Constants
Memory and constants are encoded as follows:
