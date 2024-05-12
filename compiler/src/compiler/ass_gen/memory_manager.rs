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
static MAX_ADDR: u16 = 513;

// Address range that is allocated at compile time by the user that is not allowed to be touched by
// the compiler. Useful if something in asm {} requires memory to not be overwritten by the
// compuiler.
static mut PREALLOC_START: u16 = MAX_ADDR;
static mut PREALLOC_END: u16 = MAX_ADDR;

/// Push variable to the next available position in the "DM stack"
pub fn push_to_stack(register: u8) -> String {
    unsafe {
        if PREALLOC_START <= STACK_PTR && STACK_PTR <= PREALLOC_END {
            panic!("Trying to allocated memory inside user defined range!")
        }
        if STACK_PTR >= MAX_ADDR {
            panic!("Trying to allocated outside of MAX_ADDR!")
        }
        let ass_output: String = format!("st, r{register}, {STACK_PTR}");
        STACK_PTR += 1;

        // Jump over preallocated range if one is set
        if PREALLOC_START <= STACK_PTR && STACK_PTR <= PREALLOC_END && PREALLOC_END < MAX_ADDR {
            STACK_PTR = PREALLOC_END + 1;
        }

        ass_output
    }
}

/// Pop variable from the last position in the "DM stack"
pub fn pop_from_stack(register: u8) -> String {
    unsafe {
        let ass_output: String = format!("ld, r{register}, {STACK_PTR}");

        STACK_PTR -= 1;

        // Jump over preallocated range if one is set
        if PREALLOC_START <= STACK_PTR && STACK_PTR <= PREALLOC_END && PREALLOC_START > 0 {
            STACK_PTR = PREALLOC_START - 1;
        }

        ass_output
    }
}

/// Store data from regisster to addr in DM
pub fn write_to_dm(register: u8, addr: u16) -> String {
    unsafe {
        if PREALLOC_START <= addr && addr <= PREALLOC_END {
            panic!("Trying to allocated memory inside user defined range!")
        }
    }
    if addr >= MAX_ADDR {
        panic!("Trying to allocated outside of MAX_ADDR!")
    }

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
pub fn get_stack_ptr() -> u16 {
    unsafe { STACK_PTR }
}

/// Decrements the stack ptr to symbolize stack being popped.
pub fn decrement_stack_ptr() {
    unsafe {
        STACK_PTR -= 1;
    }
}

/// Push new variable to memory map
pub unsafe fn push_to_mem_map(var_id: u32, address: u16) {
    if address >= MAX_ADDR {
        panic!("Trying to allocate outside of MAX_ADDR!")
    }

    if PREALLOC_START <= address && address <= PREALLOC_END {
        panic!("Trying to allocate memory inside of PREALLOC range!")
    }

    MEMORY_MAP
        .lock()
        .expect("Failed to lock on MEMORY_MAP")
        .insert(var_id, address);
}

/// Read the memory address of a variable
pub fn read_from_mem_map(var_id: u32) -> Option<u16> {
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
pub fn remove_from_mem_map(var_id: u32) {
    MEMORY_MAP
        .lock()
        .expect("Failed to lock on MEMORY_MAP")
        .remove(&var_id);
}

/// Pre-allocate memory space that is not allowed to be touched by the compiler
pub fn remove_mem_from_compiler(start: Option<u16>, end: Option<u16>) {
    if let Some(start_addr) = start {
        let mut end_addr = MAX_ADDR;

        if let Some(addr) = end {
            end_addr = addr;
        }

        if start_addr > end_addr {
            panic!("Invalid memory range set with PREALLOC macro!")
        }

        unsafe {
            PREALLOC_START = start_addr;
            PREALLOC_END = end_addr;
        }
    }
}
