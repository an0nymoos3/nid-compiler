/*
* This file is dedicated to generating ASS code related to memory.
*/

// Acts as a stack pointer to allow the compiler to use the more optimized st and ld instructions,
// rather than psh or pop
static mut STACK_PTR: i16 = 0;

/// Push variable to the next available position in the "DM stack"
unsafe fn push_to_stack(register: i8) -> String {
    let ass_output: String = format!("st, r{register}, {STACK_PTR}");
    STACK_PTR += 1;
    ass_output
}

/// Pop variable from the last position in the "DM stack"
unsafe fn pop_from_stack(register: i8) -> String {
    let ass_output: String = format!("ld, r{register}, {STACK_PTR}");
    STACK_PTR -= 1;
    ass_output
}

/// Store data from regisster to addr in DM
fn heap_alloc(register: i8, addr: i16) -> String {
    format!("st, r{register}, {addr}")
}

/// Load data from addr in DM to register
fn read_from_heap(register: i8, addr: i16) -> String {
    format!("ld, r{register}, {addr}")
}

/// Generates ldi instruction
fn load_const(register: i8, const_val: i16) -> String {
    format!("ldi, r{register}, {const_val}")
}
