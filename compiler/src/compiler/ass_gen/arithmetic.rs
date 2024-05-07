/*
* Handles arithmetic generation. Such as generating the necessary algorithms to make multiplication and
* division work.
*
* To make the compiler more consistent it has 4 designated arithmetic registers.
* r0 - num1/result
* r1 - num2
* r2 - helper register, useful for things like mul, or div
* r3 - helper register, useful for things like mul, or div
*
* Since these functions will be used by the compiler some sanity checks aren't performed since they
* are assumed to not be able to happen.
*
* Some assumptions that are also made:
* If only one register, addr or cosnt is used, it will use the first one. Therefore there are only
* checks for the first of these whenever you check for a field.
*/

use super::memory_manager::{pop_from_stack, push_to_stack, read_from_dm};

/// Performs addition on 2 operands. Only expects 2 parameters to be Some
pub fn add(
    reg1: Option<i8>,
    reg2: Option<i8>,
    addr1: Option<i16>,
    addr2: Option<i16>,
    const1: Option<i16>,
    const2: Option<i16>,
) -> Vec<String> {
    if const1.is_some() && const2.is_some() {
        return vec![format!("ldi, r0, {}", const1.unwrap() + const2.unwrap())];
    }
    if const1.is_some() {
        return perform_op("addi", reg1, reg2, addr1, addr2, const1);
    }
    perform_op("add", reg1, reg2, addr1, addr2, const1)
}

/// Performs subtractions on 2 operands. Only expects 2 parameters to be Some
pub fn sub(
    reg1: Option<i8>,
    reg2: Option<i8>,
    addr1: Option<i16>,
    addr2: Option<i16>,
    const1: Option<i16>,
    const2: Option<i16>,
) -> Vec<String> {
    if const1.is_some() && const2.is_some() {
        return vec![format!("ldi, r0, {}", const1.unwrap() - const2.unwrap())];
    }
    if const1.is_some() {
        return perform_op("subi", reg1, reg2, addr1, addr2, const1);
    }
    perform_op("sub", reg1, reg2, addr1, addr2, const1)
}

/// Performs multiplication on 2 operands. Only expects 2 parameters to be Some
pub fn multiplication(
    reg1: Option<i8>,
    reg2: Option<i8>,
    addr1: Option<i16>,
    addr2: Option<i16>,
    const1: Option<i16>,
    const2: Option<i16>,
) -> Vec<String> {
    if const1.is_some() && const2.is_some() {
        return vec![format!("ldi, r0, {}", const1.unwrap() * const2.unwrap())];
    }
    if let Some(val) = const1 {
        if val == 2 {
            return vec![lsl(reg1.unwrap())];
        }
        return perform_op("muli", reg1, reg2, addr1, addr2, const1);
    }
    perform_op("mul", reg1, reg2, addr1, addr2, const1)
}

/// Performs division on 2 operands. Only expects 2 parameters to be Some
pub fn division(
    reg1: Option<i8>,
    reg2: Option<i8>,
    addr1: Option<i16>,
    addr2: Option<i16>,
    const1: Option<i16>,
    const2: Option<i16>,
) -> Vec<String> {
    if const1.is_some() && const2.is_some() {
        return vec![format!("ldi, r0, {}", const1.unwrap() / const2.unwrap())];
    }
    if let Some(val) = const1 {
        if val == 2 {
            return vec![lsr(reg1.unwrap())];
        }
        return perform_op("divi", reg1, reg2, addr1, addr2, const1);
    }
    perform_op("div", reg1, reg2, addr1, addr2, const1)
}

/// Logical shift left
pub fn lsl(register: i8) -> String {
    format!("lsl, r{register}")
}

/// Logical shift right
pub fn lsr(register: i8) -> String {
    format!("lsr, r{register}")
}

/// Hepler function to avoid code duplication
fn perform_op(
    op: &str,
    reg1: Option<i8>,
    reg2: Option<i8>,
    addr1: Option<i16>,
    addr2: Option<i16>,
    const1: Option<i16>,
) -> Vec<String> {
    let mut instructions: Vec<String> = Vec::new();
    let mut work_reg: String = String::from(""); // Default working register
    let mut const_or_addr: Option<i16> = None;
    let mut stack_pushed: bool = false; // To know if stack needs to be popped after arithmetic
                                        // operation.

    if let Some(reg) = reg1 {
        work_reg = reg.to_string();
    }

    // If second register is used, it has to be pushed to the stack
    if let Some(reg) = reg2 {
        unsafe { instructions.push(push_to_stack(reg)) }
        stack_pushed = true;
    }

    // Add addresses to instrucitons
    if reg1.is_none() && reg2.is_none() {
        // If no register was set, use addr1 as first term
        if let Some(addr) = addr1 {
            instructions.push(read_from_dm(0, addr));
            work_reg = String::from("r0"); // Set register to work on.

            if let Some(addr) = addr2 {
                const_or_addr = Some(addr);
            }
        }
    } else if reg2.is_none() {
        const_or_addr = addr1; // Set to addr1, if addr1 is none it will be replaced with the const
                               // in the end of this function
    }

    // Add constants to instructions, if addresses werent added
    if const_or_addr.is_none() && const1.is_some() {
        const_or_addr = const1;
    }

    // Just sanity checks that compiler generated a correct instruction.
    if work_reg.is_empty() {
        panic!("No register set to work on!");
    }
    if const_or_addr.is_none() {
        panic!("Second operand not set!"); // Panic if invalid instruction
    }

    instructions.push(format!("{op}, {work_reg}, {}", const_or_addr.unwrap()));

    if stack_pushed {
        unsafe { instructions.push(pop_from_stack(2)) } // Just pop to helper register for now.
    }

    instructions
}
