use crate::compiler::ass_gen::memory_manager::{
    already_in_reg, get_reg, push_to_mem_map, read_from_dm, read_from_mem_map, remove_from_mem_map,
    MAX_ADDR, PREALLOC_END, PREALLOC_START,
};

/// Generates the required assembly code for allocating a variable in a specific mem_address
pub fn move_to(var_id: u32, addr: u16) -> Vec<String> {
    let mut instructions: Vec<String> = Vec::new();

    unsafe {
        if PREALLOC_START > addr || PREALLOC_END < addr {
            println!("Warning: Trying to allocate memory inside compiler space! This may result in memory being overwritten/corrupted!");
        }
    }
    if addr > MAX_ADDR {
        panic!("addr outside MAX_ADDR!")
    }

    if let Some(reg) = already_in_reg(var_id) {
        instructions.push(format!("st, r{reg}, {addr}"));

        // Change the location of var_id in mem_map
        remove_from_mem_map(var_id);
        push_to_mem_map(var_id, addr);
    } else {
        let reg = get_reg(Some(var_id));
        let var_addr = read_from_mem_map(var_id);

        if var_addr.is_none() {
            panic!("Invalid var_id supplied to write_to()!")
        }

        // Push relevant instructions to run
        instructions.push(read_from_dm(reg, var_addr.unwrap()));
        instructions.push(format!("st, r{reg}, {addr}"));

        // Change the location of var_id in mem_map
        remove_from_mem_map(var_id);
        push_to_mem_map(var_id, addr);
    }

    instructions
}
