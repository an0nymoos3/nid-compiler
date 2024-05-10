/*
* This file is dedicated to generating ASS code related to memory.
*/

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref MEMORY_MAP: Mutex<HashMap<u32, u16>> = Mutex::new(HashMap::new());
}

// Acts as a stack pointer to allow the compiler to use the more optimized st and ld instructions,
// rather than psh or pop
static mut STACK_PTR: u16 = 0;

/// Push variable to the next available position in the "DM stack"
pub unsafe fn push_to_stack(register: u8) -> String {
    let ass_output: String = format!("st, r{register}, {STACK_PTR}");
    STACK_PTR += 1;
    ass_output
}

/// Pop variable from the last position in the "DM stack"
pub unsafe fn pop_from_stack(register: u8) -> String {
    let ass_output: String = format!("ld, r{register}, {STACK_PTR}");
    STACK_PTR -= 1;
    ass_output
}

/// Store data from regisster to addr in DM
pub fn heap_alloc(register: u8, addr: u16) -> String {
    format!("st, r{register}, {addr}")
}

/// Load data from addr in DM to register
pub fn read_from_dm(register: u8, addr: u16) -> String {
    format!("ld, r{register}, {addr}")
}

/// Generates ldi instruction
pub fn load_const(register: u8, const_val: i16) -> String {
    format!("ldi, r{register}, {const_val}")
}

/// Returns current memory address of stack pointer
pub unsafe fn get_stack_ptr() -> u16 {
    STACK_PTR
}

/// Decrements the stack ptr to symbolize stack being popped.
pub unsafe fn decrement_stack_ptr() {
    STACK_PTR -= 1;
}

/// Push new variable to memory map
pub unsafe fn push_to_mem_map(var_id: u32, address: u16) {
    MEMORY_MAP
        .lock()
        .expect("Failed to lock on MEMORY_MAP")
        .insert(var_id, address);
}

/// Read the memory address of a variable
pub unsafe fn read_from_mem_map(var_id: u32) -> Option<u16> {
    if let Some(addr) = MEMORY_MAP
        .lock()
        .expect("Failed to lock on MEMORY_MAP")
        .get(&var_id)
    {
        return Some(*addr);
    }
    None
}

/// Remove variable from memory map
pub unsafe fn remove_from_mem_map(var_id: u32) {
    MEMORY_MAP
        .lock()
        .expect("Failed to lock on MEMORY_MAP")
        .remove(&var_id);
}
