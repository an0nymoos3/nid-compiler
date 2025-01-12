# Advanced System Scripting (ASS)

## Advanced System Scripting

Advanced System Scripting (ASS) is our assembly like language for our custom CPU architecture.  
The general structure of ASS is as follows.  
`Instruction, A-mode, Register, Value`.

## Flags

To understand the instructions below it is good to familiarise yourself with some of the hardware flags
that can be set and unset during execution. These are the hardware flags used in the project CPU.

- Z: Stands for **Z**ero, which is set when the result in the ALU is equal to 0.
- N: Stands for **N**egative, which is set when the result in the ALU is less than 0.
- C: Stands for **C**arry, which is set when the result from the ALU has a bit carried over, or shifted out.
- V: Stands for o**V**erflow, which is set when the result is larger than the registers can handle.

## Routines

ASS has support for assembly routines, they can be thought of as functions in higher level languages.
To return to the previously run ASS code from a routine you *have* to use the `ret` keyword. If `ret` is
left out the program will simply continue with the next line below the routine. To define a routine you
simply give it a name and finish with a `:`.

```
...
call, my_routine ; Run the routine
some, code, goes, here ; Continue running here after my_routine returns
...

my_routine:
  ldi, a00, r1, 100
  ret ; Return to the calling routine
```

## Instructions

The following instructions are the currently implemented.

| Instruction         | Pseudo code           | Description                                                                                                          |
|---------------------|-----------------------|----------------------------------------------------------------------------------------------------------------------|
| nop                 | NOP                   | Does nothing.                                                                                                        |
| ld, A, Rd, Addr     | Rd <= Mem(Addr)       | Loads the value from Mem(address) to register Rd.                                                                    |
| ldi, A, Rd, const.  | Rd <= const.          | Loads the constant value const. to register Rd.                                                                      |
| st, A, Rd, Addr     | Mem(Addr) <= Rd       | Store value from Rd in Mem(address).                                                                                 |
| psh, A, Rd          | Mem(DC++) <= Rd       | Push the value from Rd to the next spot in memory. (DC = Data Counter; Pointer to current location in memory stack.) |
| pop, A, Rd          | Rd <= Mem(DC--)       | Decrements DC. Gives the effect of popping the stack. Puts value in Rd.                                              |
| add, A, Rd, Addr    | Rd <= Rd + Mem(Addr)  | Add Rd and Mem(address).                                                                                             |
| addi, A, Rd, const. | Rd <= Rd + const.     | Add Rd and cosnt. value.                                                                                             |
| sub, A, Rd, Addr    | Rd <= Rd - Mem(Addr)  | Subtract Mem(address) from Rd.                                                                                       |
| subi, A, Rd, const. | Rd <= Rd - const.     | Subtract const. value from Rd.                                                                                       |
| cmp, A, Rd, Addr    | Rd - Mem(Addr)        | Compare Rd with Mem(addr) (No return value, only affects flags)                                                      |
| cmpi, A, Rd, const. | Rd - const.           | Compare Rd with const. (No return value, only affects flags)                                                         |
| mul, A, Rd, Addr    | Rd <= Rd * Mem(Addr)  | Multiplies Rd by Mem(address).                                                                                       |
| muli, A, Rd, const. | Rd <= Rd * cosnt.     | Multiplies Rd by const. value.                                                                                       |
| div, A, Rd, const.  | Rd <= Rd / Mem(addr)  | Divides Rd by Mem(address).                                                                                          |
| divi, A, Rd, const. | Rd <= Rd / cosnt.     | Divides Rd by const. value.                                                                                          |
| and, A, Rd, Addr    | Rd <= Rd & Mem(addr)  | Performs and between Rd and Mem(addr).                                                                               |
| andi, A, Rd, const. | Rd <= Rd & cosnt.     | Performs and between Rd and const.                                                                                   |
| or, A, Rd, Addr     | Rd <= Rd \| Mem(addr) | Performs or between Rd and Mem(addr).                                                                                |
| ori, A, Rd, const.  | Rd <= Rd \| cosnt.    | Performs or between Rd and const.                                                                                    |
| not, A, Rd, Addr    | Rd != Mem(addr)       | Performs a not operation between Rd and Mem(Addr).                                                                   |
| xor, A, Rd, Addr    | Rd <= Rd ^ Mem(addr)  | Performs xor between Rd and Mem(addr).                                                                               |
| xori, A, Rd, const. | Rd <= Rd ^ cosnt.     | Performs xor between Rd and const.                                                                                   |
| call, subr.         | push call_stack       | Performs a call to subroutine (Similar to calling a function).                                                       |
| ret                 | pop call_stack        | Returns to where it was in execution before a call.                                                                  |
| jmp, routine_name   | jump -> routine_name  | Performs a jump to the line of assembly with 'routine_name'.                                                         |
| jmpi, const.        | jump -> cur_row + n   | Performs a relative jump to current row + n.                                                                         |
| beq                 | jump if Z = 1         | Branch (jump) if equal.                                                                                              |
| bne                 | jump if Z = 0         | Branch (jump) if not equal.                                                                                          |
| bpr                 | jump if N = 0         | Branch (jump) positive result.                                                                                       |
| bnr                 | jump if N = 1         | Branch (jump) negative result.                                                                                       |
| bge                 | jump if N ^ V = 0     | Branch (jump) if greater than or equal.                                                                              |
| blt                 | jump if N ^ V = 1     | Branch (jump) if less than.                                                                                          |

## NOTE:

For `call` and `ret` to work properly it is important that the CPU implements them correctly.
The following instructions have to be implemented on a u-code level for it to work properly.

- `call` should save the program counter (`PC`) to some memory location. Either a separate
  register or it can push the memory address and save it there.
- `ret` should read the saved `PC` from the designated location and replace `PC` with it.
  If it was pushed onto the memory, make sure to pop the memory.

If the above instructions are implemented correctly the program should seamlessly be able to jump
between different functions. If not, you'll likely encounter undefined behaviour.