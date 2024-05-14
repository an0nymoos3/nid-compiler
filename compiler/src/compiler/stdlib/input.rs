/// Generates code to test if kb scancode is pressed
pub fn is_pressed(scancode: u16, branch_name: &str) -> Vec<String> {
    vec![format!("kbd, {scancode}"), format!("byk, {branch_name}")]
}
