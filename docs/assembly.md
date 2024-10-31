# Advanced System Scripting (ASS)

## Advanced System Scripting
Advanced System Scripting (ASS) is our assembly like language for our custom CPU architecture.  
The general structure of ASS is as follows.  
`Instruction, A-mode, Register, Value`.

## Instructions

The following instructions are the currently implemented.

| Instruction         | Pseudo code           | Description                                                     |
| ------------------- | --------------------- | --------------------------------------------------------------- |
| nop                 | NOP                   | Does nothing.                                                   |
| ld, A, Rd, Addr     | Rd <= Mem(Addr)       | Loads the value from Mem(address) to register Rd.               |
| ldi, A, Rd, const.  | Rd <= const.          | Loads the constant value const. to register Rd.                 |
| st, A, Rd, Addr     | Mem(Addr) <= Rd       | Store value from Rd in Mem(address).                            |
| psh, A, Rd          | Mem(DC++) <= Rd       | Push the value from Rd to the next spot in memory. (DC = Data Counter; Pointer to current location in memory stack.) |
| pop                 | DC--                  | Decrements DC. Gives the effect of popping the stack.           |
| add, A, Rd, Addr    | Rd <= Rd + Mem(Addr)  | Add Rd and Mem(address).                                        |
| addi, A, Rd, const. | Rd <= Rd + const.     | Add Rd and cosnt. value.                                        |
| sub, A, Rd, Addr    | Rd <= Rd - Mem(Addr)  | Subtract Mem(address) from Rd.                                  |
| subi, A, Rd, const. | Rd <= Rd - const.     | Subtract const. value from Rd.                                  |
| cmp, A, Rd, Addr    | Rd <= Rd - Mem(Addr)  | Compare Rd with Mem(addr) (No return value, only affects flags) |
| cmpi, A, Rd, const. | Rd <= Rd - const.     | Compare Rd with const. (No return value, only affects flags)    |
| mul, A, Rd, Addr    | Rd <= Rd * Mem(Addr)  | Multiplies Rd by Mem(address).                                  |
| muli, A, Rd, const. | Rd <= Rd * cosnt.     | Multiplies Rd by const. value.                                  |
| div, A, Rd, const.  | Rd <= Rd / Mem(addr)  | Divides Rd by Mem(address).                                     |
| divi, A, Rd, const. | Rd <= Rd / cosnt.     | Divides Rd by const. value.                                     |
| and, A, Rd, const.  | Rd <= Rd & Mem(addr)  | Performs and between Rd and Mem(addr).                          |
| andi, A, Rd, const. | Rd <= Rd & cosnt.     | Performs and between Rd and const.                              |
| or, A, Rd, const.   | Rd <= Rd \| Mem(addr) | Performs and between Rd and Mem(addr).                          |
| ori, A, Rd, const.  | Rd <= Rd \| cosnt.    | Performs and between Rd and const.                              |
| call, proc.         | push call_stack       | Performs a call to procedure (Similar to calling a function)    |
| ret                 | pop call_stack        | Returns to where it was in execution before a call.             |
| jmp, branch_name    | jump -> branch_name   | Performs a jump to the line of assembly with 'branch_name'.     |
| jmpi, n             | jump -> cur_row + n   | Performs a relative jump to current row + n.                    |
