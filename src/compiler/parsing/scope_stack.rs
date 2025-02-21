use super::ast::Variable;
use std::collections::VecDeque;

pub struct ParserStack {
    stack: VecDeque<Vec<*mut Variable>>,
}

impl ParserStack {
    /// Create a new ParserStack
    pub fn new() -> Self {
        Self {
            stack: VecDeque::new(),
        }
    }

    /// Creates a new empty scope in the stack
    pub fn add_scope(&mut self) {
        self.stack.push_back(Vec::new());
    }

    /// Removes the last scope in the stack
    pub fn remove_scope(&mut self) {
        self.stack.pop_back();
    }

    /// Pushes a variable to the back of the stack, in the bottom scope
    pub fn push_var(&mut self, var_ptr: *mut Variable) -> Result<(), ()> {
        if let Some(v) = self.stack.get_mut(self.stack.len() - 1) {
            v.push(var_ptr);
            return Ok(());
        }
        Err(())
    }

    /// Returns a variable stored in the stack with a matching identifier
    pub fn find_var(&self, identifier: &str) -> Option<*mut Variable> {
        for i in (0..self.stack.len()).rev() {
            if let Some(v) = self.stack.get(i) {
                for var_ptr in v {
                    unsafe {
                        if (**var_ptr).identifier == identifier {
                            return Some(*var_ptr);
                        }
                    }
                }
            }
        }
        None
    }
}
