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
    if const1.is_some() || const2.is_some() {
        return add_or_sub("addi", reg1, reg2, addr1, addr2, const1, const2);
    }
    add_or_sub("add", reg1, reg2, addr1, addr2, const1, const2)
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
        return vec![format!("ldi, r0, {}", const1.unwrap() + const2.unwrap())];
    }
    if const1.is_some() || const2.is_some() {
        return add_or_sub("subi", reg1, reg2, addr1, addr2, const1, const2);
    }
    add_or_sub("sub", reg1, reg2, addr1, addr2, const1, const2)
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
    Vec::new()
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
    Vec::new()
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
fn add_or_sub(
    op: &str,
    reg1: Option<i8>,
    reg2: Option<i8>,
    addr1: Option<i16>,
    addr2: Option<i16>,
    const1: Option<i16>,
    const2: Option<i16>,
) -> Vec<String> {
    let mut instructions: Vec<String> = Vec::new();
    let mut work_reg: String = String::from("r0"); // Default working register
    let mut const_or_addr: Option<i16> = None;
    let mut stack_pushed: bool = false; // To know if stack needs to be popped after arithmetic
                                        // operation.

    // If reg1 is specified, change work_reg
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

            if let Some(addr) = addr2 {
                const_or_addr = Some(addr);
            }
        }
    }

    // Add constants to instructions
    if const_or_addr.is_none() {
        if const1.is_some() {
            const_or_addr = const1;
        } else if const2.is_some() {
            const_or_addr = const2;
        }
    }

    instructions.push(format!("{op}, {work_reg}, {}", const_or_addr.unwrap()));

    if stack_pushed {
        unsafe { instructions.push(pop_from_stack(2)) } // Just pop to helper register for now.
    }

    instructions
}

/// Helper function for performing multiplication and division, and avoids code duplication
fn mul_or_div(
    op: &str,
    reg1: Option<i8>,
    reg2: Option<i8>,
    addr1: Option<i16>,
    addr2: Option<i16>,
    const1: Option<i16>,
    const2: Option<i16>,
) -> Vec<String> {
    Vec::new()
}
