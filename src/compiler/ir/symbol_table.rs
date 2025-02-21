use std::collections::VecDeque;
use std::fmt::Write;

use crate::{
    compiler::parsing::{
        ast::{Ast, AstType, Block, EmptyNode, Function, Node, TypeEnum, Variable},
        lexer::TokenType,
    },
    utils::error::print_err,
};

/// NID representation of a symbol table, simply a Vec of symbol elements.
/// To learn more about symbol tables, check out ir.md
pub struct SymbolTable {
    elems: Vec<SymbolElem>,
}

/// Each element in the table, must have 3 properties to do proper parsing.
/// Identifier is the name of the function or variable.
/// Scope is where in the program the function or variable is accessable from.
/// Elem type is the return type of a function or variable type.
///
/// Node pointer is used in the case of a type error to get the line of
/// the node and use print_err() defined in error.rs.
struct SymbolElem {
    identifier: String,
    scope: String,
    data_type: TypeEnum,
    node: *mut dyn Node,
}

struct SymbolStack {
    stack: Vec<String>,
}

impl SymbolStack {
    pub fn push_stack(&mut self, scope: &str) {
        if self.stack.len() == 1 && &self.stack[0] == "global" {
            self.stack.remove(0);
        }
        self.stack.push(scope.to_string());
    }

    pub fn pop_stack(&mut self) {
        self.stack.pop();

        if self.stack.is_empty() {
            self.stack.push(String::from("global"));
        }
    }

    pub fn get_current_stack(&self) -> String {
        self.stack.iter().fold(String::new(), |mut output, s| {
            let _ = write!(output, "-{}", s);
            output
        })
    }
}

/// Creates a new symbol table from an AST.
pub fn generate_table(tree: &Ast<dyn Node>) -> SymbolTable {
    let mut elems: Vec<SymbolElem> = Vec::new();
    let mut stack = SymbolStack {
        stack: vec![String::from("global")],
    };

    let mut body = VecDeque::from(tree.body.clone());

    while !body.is_empty() {
        let ptr = body.pop_front().unwrap();

        unsafe {
            // Add vairables to table
            if (*ptr).get_type() == AstType::Variable {
                let var_ptr = ptr as *mut Variable;
                let var_elem = SymbolElem {
                    identifier: (*var_ptr).identifier.clone(),
                    scope: stack.get_current_stack(),
                    data_type: (*var_ptr).var_type.clone().unwrap(),
                    node: ptr,
                };
                elems.push(var_elem);
            }

            if (*ptr).get_type() == AstType::Block {
                // When new block starts, create a new scope, for potentially new declared
                // variables
                stack.push_stack("block");

                let block_ptr = ptr as *mut Block;
                let mut block_body = VecDeque::from((*block_ptr).body.clone());

                while !block_body.is_empty() {
                    body.push_front(block_body.pop_back().unwrap());
                }
            }

            if (*ptr).get_type() == AstType::Function {
                let func_ptr = ptr as *mut Function;

                stack.push_stack(&(*func_ptr).identifier);

                stack.push_stack("params");
                for param_ptr in (*func_ptr).params.iter() {
                    if (**param_ptr).get_type() == AstType::Variable {
                        let var_ptr = *param_ptr as *mut Variable;
                        let var_elem = SymbolElem {
                            identifier: (*var_ptr).identifier.clone(),
                            scope: stack.get_current_stack(),
                            data_type: (*var_ptr).var_type.clone().unwrap(),
                            node: ptr,
                        };
                        elems.push(var_elem);
                    }
                }
                stack.pop_stack();

                // Append the body to the program
                let mut func_body = VecDeque::from((*(*func_ptr).body).body.clone());
                while !func_body.is_empty() {
                    body.push_front(func_body.pop_back().unwrap());
                }
            }

            // Remove a scope when moving out of a body
            if (*ptr).get_type() == AstType::Empty
                && (*(ptr as *mut EmptyNode)).token_type == TokenType::CloseScope
            {
                stack.pop_stack();
            }
        }
    }

    SymbolTable { elems }
}

/// Performs a type check of all variables in the
pub fn type_check(table: &SymbolTable) -> Result<(), ()> {
    let mut failed_check: bool = false;

    for i in 0..table.elems.len() - 1 {
        let elem_i = &table.elems[i];
        for j in 1..table.elems.len() {
            let elem_j = &table.elems[j];

            if elem_i.identifier == elem_j.identifier
                && elem_i.scope == elem_j.scope
                && elem_i.data_type != elem_j.data_type
            {
                unsafe {
                    print_err(
                        &(*elem_j.node).get_line().unwrap(),
                        &format!(
                            "Missmatched types! \nExpected {:?}, but got {:?}",
                            elem_i.data_type, elem_j.data_type
                        ),
                        Some(&format!(
                            "Change {:?} {} to {:?} {}",
                            elem_j.data_type,
                            elem_j.identifier,
                            elem_i.data_type,
                            elem_j.identifier
                        )),
                    );
                    failed_check = true;
                }
            }
        }
    }

    if failed_check {
        return Err(());
    }
    Ok(())
}

/// Debugging function for printing SymbolTable to verify that
/// compiler correctly detected identfiers, scopes and types
pub fn display_table(table: &SymbolTable) {
    let ident_offset = 20;
    let scope_offset = 20;
    let type_offset = 7;

    println!("| Identifier           | Scope                | Type    |");
    println!("|----------------------|----------------------|---------|");

    for var in table.elems.iter() {
        println!(
            "| {:<ident_offset$} | {:<scope_offset$} | {:<type_offset$} |",
            var.identifier,
            var.scope,
            format!("{:?}", var.data_type)
        );
    }

    println!("|-------------------------------------------------------|");
}
