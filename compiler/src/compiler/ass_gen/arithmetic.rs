/*
* Handles arithmetic generation. Such as generating the necessary algorithms to make multiplication and
* division work.
*
* Since these functions will be used by the compiler some sanity checks aren't performed since they
* are assumed to not be able to happen.
*
* Some assumptions that are also made:
* If only one register, addr or cosnt is used, it will use the first one. Therefore there are only
* checks for the first of these whenever you check for a field.
*/

use super::memory_manager::{
    decrement_stack_ptr, get_reg, get_var_id_from_addr, push_to_stack, read_from_dm,
};

/// Global variable to get the latest result register
pub static mut LATEST_RESULT: u8 = 0;

/// Performs addition on 2 operands. Only expects 2 parameters to be Some
pub fn add(
    reg1: Option<u8>,
    reg2: Option<u8>,
    addr1: Option<u16>,
    addr2: Option<u16>,
    const1: Option<i16>,
    const2: Option<i16>,
) -> Vec<String> {
    if const1.is_some() && const2.is_some() {
        let reg: u8 = get_reg(None);
        unsafe { LATEST_RESULT = reg };
        return vec![format!(
            "ldi, {}, {}",
            reg,
            const1.unwrap() + const2.unwrap()
        )];
    }
    if const1.is_some() {
        return perform_op("addi", reg1, reg2, addr1, addr2, const1);
    }
    perform_op("add", reg1, reg2, addr1, addr2, const1)
}

/// Performs subtractions on 2 operands. Only expects 2 parameters to be Some
pub fn sub(
    reg1: Option<u8>,
    reg2: Option<u8>,
    addr1: Option<u16>,
    addr2: Option<u16>,
    const1: Option<i16>,
    const2: Option<i16>,
) -> Vec<String> {
    if const1.is_some() && const2.is_some() {
        let reg: u8 = get_reg(None);
        unsafe { LATEST_RESULT = reg };
        return vec![format!(
            "ldi, {}, {}",
            reg,
            const1.unwrap() - const2.unwrap()
        )];
    }
    if const1.is_some() {
        return perform_op("subi", reg1, reg2, addr1, addr2, const1);
    }
    perform_op("sub", reg1, reg2, addr1, addr2, const1)
}

/// Performs multiplication on 2 operands. Only expects 2 parameters to be Some
pub fn mul(
    reg1: Option<u8>,
    reg2: Option<u8>,
    addr1: Option<u16>,
    addr2: Option<u16>,
    const1: Option<i16>,
    const2: Option<i16>,
) -> Vec<String> {
    if const1.is_some() && const2.is_some() {
        let reg: u8 = get_reg(None);
        unsafe { LATEST_RESULT = reg };
        return vec![format!(
            "ldi, {}, {}",
            reg,
            const1.unwrap() * const2.unwrap()
        )];
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
pub fn div(
    reg1: Option<u8>,
    reg2: Option<u8>,
    addr1: Option<u16>,
    addr2: Option<u16>,
    const1: Option<i16>,
    const2: Option<i16>,
) -> Vec<String> {
    if const1.is_some() && const2.is_some() {
        let reg: u8 = get_reg(None);
        unsafe { LATEST_RESULT = reg };
        return vec![format!(
            "ldi, {}, {}",
            reg,
            const1.unwrap() / const2.unwrap()
        )];
    }
    if let Some(val) = const1 {
        if val == 2 {
            return vec![lsr(reg1.unwrap())];
        }
        return perform_op("divi", reg1, reg2, addr1, addr2, const1);
    }
    perform_op("div", reg1, reg2, addr1, addr2, const1)
}

/// Performs comparison on 2 operands. Basically a subtraction below the hood, but does not return
/// anything. Only affects the flags set by ALU. Only expects 2 parameters to be Some
pub fn cmp(
    reg1: Option<u8>,
    reg2: Option<u8>,
    addr1: Option<u16>,
    addr2: Option<u16>,
    const1: Option<i16>,
    const2: Option<i16>,
) -> Vec<String> {
    if const1.is_some() && const2.is_some() {
        panic!("Compiler error! Two constants should not have entered the cmp() function!")
    } else {
        perform_op("cmp", reg1, reg2, addr1, addr2, const1)
    }
}

/// Logical shift left
pub fn lsl(register: u8) -> String {
    unsafe { LATEST_RESULT = register };
    format!("lsl, r{register}")
}

/// Logical shift right
pub fn lsr(register: u8) -> String {
    unsafe { LATEST_RESULT = register };
    format!("lsr, r{register}")
}

/// Hepler function to avoid code duplication
fn perform_op(
    op: &str,
    reg1: Option<u8>,
    reg2: Option<u8>,
    addr1: Option<u16>,
    addr2: Option<u16>,
    const1: Option<i16>,
) -> Vec<String> {
    let mut instructions: Vec<String> = Vec::new();
    let mut work_reg: String = String::from(""); // Default working register
    let mut const_val: Option<i16> = None;
    let mut asm_addr: Option<u16> = None;
    let mut stack_pushed: bool = false; // To know if stack needs to be popped after arithmetic
                                        // operation.

    if let Some(reg) = reg1 {
        work_reg = format!("r{reg}");
    }

    // If second register is used, it has to be pushed to the stack
    if let Some(reg) = reg2 {
        instructions.push(push_to_stack(reg));
        stack_pushed = true;
    }

    // Add addresses to instrucitons
    if reg1.is_none() && reg2.is_none() {
        // If no register was set, use addr1 as first term
        if let Some(addr) = addr1 {
            instructions.push(read_from_dm(0, addr));

            // Get a valid / optimal register to work on
            let var_id: Option<u32> = get_var_id_from_addr(addr);
            work_reg = format!("r{}", get_reg(var_id)); // Set register to work on.

            if let Some(addr) = addr2 {
                asm_addr = Some(addr);
            }
        }
    } else if reg2.is_none() {
        asm_addr = addr1; // Set to addr1, if addr1 is none it will be replaced with the const
                          // in the end of this function
    }

    // Add constants to instructions, if addresses werent added
    if asm_addr.is_none() && const1.is_some() {
        const_val = const1;
    }

    // Just sanity checks that compiler generated a correct instruction.
    if work_reg.is_empty() {
        panic!("No register set to work on!");
    }
    if asm_addr.is_none() && const_val.is_none() {
        panic!("Second operand not set!"); // Panic if invalid instruction
    }

    if let Some(addr) = asm_addr {
        instructions.push(format!("{op}, {work_reg}, {}", addr));
    } else {
        instructions.push(format!("{op}, {work_reg}, {}", const_val.unwrap()));
    }

    if stack_pushed {
        decrement_stack_ptr() // Just pop the stack_ptr
    }

    // Set latest result register
    work_reg.remove(0);
    unsafe { LATEST_RESULT = work_reg.parse::<u8>().unwrap() };

    instructions
}
