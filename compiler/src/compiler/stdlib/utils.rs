/// Generates the assembly code for sleeping ms amount of milliseconds
pub fn sleep(ms: u16) -> Vec<String> {
    vec![format!("wait, {ms}")]
}
