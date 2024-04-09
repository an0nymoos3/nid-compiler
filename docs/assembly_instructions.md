# Advanced System Scripting (ASS)

## Advanced System Scripting
Advanced System Scripting (ASS) is our assembly like language for our custom CPU architecture.  
The general structure of ASS is as follows.  
`Instruction, A-mode, Register, Value`.

## Instructions

The following instructions are the currently implemented.

| Instruction         | Pseudo code          | Description                                           |
| ------------------- | -------------------- | ----------------------------------------------------- |
| NOP                 | NOP                  | Does nothing.                                         |
| LD, A, Rd, Addr     | Rd <= Mem(Addr)      | Loads the value from Mem(address) to register Rd.     |
| LDI, A, Rd, const.  | Rd <= const.         | Loads the constant value const. to register Rd.       |
| ST, A, Rd, Addr     | Mem(Addr) <= Rd      | Store value from Rd in Mem(address).                  |
| PSH, A, Rd          | Mem(DC++) <= Rd      | Push the value from Rd to the next spot in memory. (DC = Data Counter; Pointer to current location in memory stack.) |
| POP                 | DC--                 | Decrements DC. Gives the effect of popping the stack. |
| ADD, A, Rd, Addr    | Rd <= Rd + Mem(Addr) | Add Rd and Mem(address).                              |
| ADDI, A, Rd, const. | Rd <= Rd + const.    | Add Rd and cosnt. value.                              |
| SUB, A, Rd, Addr    | Rd <= Rd - Mem(Addr) | Subtract Mem(address) from Rd.                        |
| SUBI, A, Rd, const. | Rd <= Rd - const.    | Subtract const. value from Rd.                        |
| MUL, A, Rd, Addr    | Rd <= Rd * Mem(Addr) | Multiplies Rd by Mem(address).                        |
| MULI, A, Rd, const. | Rd <= Rd * cosnt.    | Multiplies Rd by const. value.                        |
