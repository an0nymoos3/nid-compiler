# Advanced System Scripting (ASS)

## Advanced System Scripting
Advanced System Scripting (ASS) is our assembly like language for our custom CPU architecture.  
The general structure of ASS is as follows.  
`Instruction, A-mode, Register, Value`.

## Flags
To understand the instructions below it is good to familiarise yourself with some of the hardware flags
that can be set and unset during execution. These are the hardware flags used in the project CPU.
 - Z: Stands for Zero, which is set when the result in the ALU is equal to 0.
 - N: Stands for Negative, which is set when the result in the ALU is less than 0.
 - C: Stands for Carry, which is set when the result from the ALU has a bit carried over, or shifted out.
 - V: Stands for oVerflow, which is set when the result is larger than the registers can handle.

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
| cmp, A, Rd, Addr    | Rd - Mem(Addr)        | Compare Rd with Mem(addr) (No return value, only affects flags) |
| cmpi, A, Rd, const. | Rd - const.           | Compare Rd with const. (No return value, only affects flags)    |
| mul, A, Rd, Addr    | Rd <= Rd * Mem(Addr)  | Multiplies Rd by Mem(address).                                  |
| muli, A, Rd, const. | Rd <= Rd * cosnt.     | Multiplies Rd by const. value.                                  |
| div, A, Rd, const.  | Rd <= Rd / Mem(addr)  | Divides Rd by Mem(address).                                     |
| divi, A, Rd, const. | Rd <= Rd / cosnt.     | Divides Rd by const. value.                                     |
| and, A, Rd, Addr    | Rd <= Rd & Mem(addr)  | Performs and between Rd and Mem(addr).                          |
| andi, A, Rd, const. | Rd <= Rd & cosnt.     | Performs and between Rd and const.                              |
| or, A, Rd, Addr     | Rd <= Rd \| Mem(addr) | Performs or between Rd and Mem(addr).                           |
| ori, A, Rd, const.  | Rd <= Rd \| cosnt.    | Performs or between Rd and const.                               |
| not, A, Rd, Addr    | Rd != Mem(addr)       | Performs a not operation between Rd and Mem(Addr).              |
| xor, A, Rd, Addr    | Rd <= Rd ^ Mem(addr)  | Performs xor between Rd and Mem(addr).                          |
| xori, A, Rd, const. | Rd <= Rd ^ cosnt.     | Performs xor between Rd and const.                              |
| call, subr.         | push call_stack       | Performs a call to subroutine (Similar to calling a function).  |
| ret                 | pop call_stack        | Returns to where it was in execution before a call.             |
| jmp, branch_name    | jump -> branch_name   | Performs a jump to the line of assembly with 'branch_name'.     |
| jmpi, const.        | jump -> cur_row + n   | Performs a relative jump to current row + n.                    |
| beq                 | jump if Z = 1         | Branch (jump) if equal.                                         |
| bne                 | jump if Z = 0         | Branch (jump) if not equal.                                     |
| bpr                 | jump if N = 0         | Branch (jump) positive result.                                  |
| bnr                 | jump if N = 1         | Branch (jump) negative result.                                  |
| bge                 | jump if N ^ V = 0     | Branch (jump) if greater than or equal.                         |
| blt                 | jump if N ^ V = 1     | Branch (jump) if less than.                                     |
