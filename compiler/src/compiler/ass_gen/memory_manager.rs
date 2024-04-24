static mut STACK_PTR: i32 = 0; // Pointer to current address in data memory.

/// Pushes variable to the next available position in the "DM stack"
unsafe fn push_to_stack(register: i8) -> String {
    let ass_output: String = format!("st, r{register}, {STACK_PTR}");
    STACK_PTR += 1;
    ass_output
}

unsafe fn pop_from_stack(register: i8) -> String {
    let ass_output: String = format!("ld, r{register}, {STACK_PTR}");
    STACK_PTR -= 1;
    ass_output
}
