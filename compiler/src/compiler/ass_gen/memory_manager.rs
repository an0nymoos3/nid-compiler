/*
* This file is dedicated to generating ASS code related to memory.
*/

use lazy_static::lazy_static;
use std::collections::VecDeque;
use std::sync::Mutex;

/// Struct representing an item in memory, most likely variable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryItem {
    pub var_id: u32,
    pub reg: Option<u8>,
    pub addr: u16,
}

lazy_static! {
    static ref MEMORY_MAP: Mutex<Vec<MemoryItem>> = Mutex::new(Vec::new());
    static ref REG_MAP: Mutex<VecDeque<MemoryItem>> = Mutex::new(VecDeque::new());
}

// Acts as a stack pointer to allow the compiler to use the more optimized st and ld instructions,
// rather than psh or pop
static mut STACK_PTR: u16 = 0;
pub static MAX_ADDR: u16 = 512;

// Address range that is allocated at compile time by the user that is not allowed to be touched by
// the compiler. Useful if something in asm {} requires memory to not be overwritten by the
// compuiler.
pub static mut PREALLOC_START: u16 = MAX_ADDR;
pub static mut PREALLOC_END: u16 = MAX_ADDR;

/// Push variable to the next available position in the "DM stack"
pub fn push_to_stack(register: u8) -> String {
    unsafe {
        // Jump over preallocated range if one is set
        if PREALLOC_START <= STACK_PTR && STACK_PTR <= PREALLOC_END && PREALLOC_END < MAX_ADDR {
            STACK_PTR = PREALLOC_END + 1;
        }
        if PREALLOC_START <= STACK_PTR && STACK_PTR <= PREALLOC_END {
            panic!("Trying to allocated memory inside user defined range!")
        }
        if STACK_PTR >= MAX_ADDR {
            panic!("Trying to allocated outside of MAX_ADDR!")
        }
        let ass_output: String = format!("st, r{register}, {STACK_PTR}");
        STACK_PTR += 1;

        ass_output
    }
}

// TODO: Find a use for the function below
/// Pop variable from the last position in the "DM stack"
pub fn _pop_from_stack(register: u8) -> String {
    unsafe {
        // Jump over preallocated range if one is set
        if PREALLOC_START <= STACK_PTR && STACK_PTR <= PREALLOC_END && PREALLOC_START > 0 {
            STACK_PTR = PREALLOC_START - 1;
        }

        let ass_output: String = format!("ld, r{register}, {STACK_PTR}");

        STACK_PTR -= 1;

        ass_output
    }
}

/// Store data from regisster to addr in DM
pub fn write_to_dm(register: u8, addr: u16) -> String {
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
pub fn push_to_mem_map(var_id: u32, address: u16) {
    if address >= MAX_ADDR {
        panic!("Trying to allocate outside of MAX_ADDR!")
    }

    MEMORY_MAP
        .lock()
        .expect("Failed to lock on MEMORY_MAP")
        .push(MemoryItem {
            var_id,
            reg: None,
            addr: address,
        })
}

/// Read the memory address of a variable
pub fn read_from_mem_map(var_id: u32) -> Option<u16> {
    for item in MEMORY_MAP
        .lock()
        .expect("Failed to lock on MEMORY_MAP")
        .iter()
    {
        if item.var_id == var_id {
            return Some(item.addr);
        }
    }
    None
}

/// Remove variable from memory map
pub fn remove_from_mem_map(var_id: u32) {
    let mut mem_map = MEMORY_MAP.lock().expect("Failed to lock on MEMORY_MAP");

    let mut rm_index: usize = usize::MAX;
    for (index, item) in mem_map.iter().enumerate() {
        if item.var_id == var_id {
            rm_index = index;
        }
    }
    if rm_index != usize::MAX {
        mem_map.remove(rm_index);
    }
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

/// Registers a usage of a register in the REG_MAP
pub fn use_reg(item: &MemoryItem) {
    let mut reg_map = REG_MAP.lock().expect("Failed to lock on REG_MAP!");

    // Make sure compiler uses a register when adding it to reg_map
    if item.reg.is_none() {
        panic!("Trying to use new item item in REG_MAP without a set reg field!")
    }

    // Check if item exists in map already
    let mut map_index: usize = usize::MAX;
    for (index, iter_item) in reg_map.iter().enumerate() {
        if item == iter_item {
            map_index = index;
        }
    }
    if map_index != usize::MAX {
        reg_map.remove(map_index);
    }

    reg_map.push_back(item.clone());
}

/// Gets the optimal register to use.
pub fn get_reg(var_id: Option<u32>) -> u8 {
    let mut reg_map = REG_MAP.lock().expect("Failed to lock on REG_MAP!");

    // Return the currently used register of the variable, if one exists
    if let Some(var) = var_id {
        for item in reg_map.iter() {
            if item.var_id == var {
                return item.reg.unwrap();
            }
        }
    }

    // Else pop the least recently used item and return it's register
    // Use all 16 register available
    if reg_map.len() == 16 {
        return reg_map
            .pop_front()
            .expect("Failed to perform pop_front() on REG_MAP!")
            .reg
            .unwrap(); // use_reg() requires that a register is set on each item added to REG_MAP
    }
    reg_map.len() as u8 // Return the next available position, which should be the current
                        // length of the reg_map
}

/// Returns the variable stored at the specified address
pub fn get_var_id_from_addr(addr: u16) -> Option<u32> {
    let mem_map = MEMORY_MAP.lock().expect("Failed to lock MEMORY_MAP!");

    for item in mem_map.iter() {
        if item.addr == addr {
            return Some(item.var_id);
        }
    }

    None
}

/// Returns the register if a variable is already loaded, else None
pub fn already_in_reg(var_id: u32) -> Option<u8> {
    let reg_map = REG_MAP.lock().expect("Failed to lock REG_MAP!");

    for item in reg_map.iter() {
        if item.var_id == var_id {
            return item.reg;
        }
    }

    None
}
