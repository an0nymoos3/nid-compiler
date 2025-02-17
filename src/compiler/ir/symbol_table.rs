use crate::{
    compiler::parsing::ast::{Ast, AstType, Block, Function, Node, TypeEnum, Variable},
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
    elem_type: TypeEnum,
    node: *mut dyn Node,
}

/// Creates a new symbol table from an AST.
pub fn generate_table(tree: &Ast<dyn Node>) -> SymbolTable {
    let mut elems: Vec<SymbolElem> = Vec::new();

    for node in tree.body.iter() {
        unsafe {
            // NOTE: This inner code is duplicate of traverse_ast_block
            // however it is because anything in this function is classed
            // as global, while in traverse_ast_block it is a local scope.

            // Variables are a simple case, just add them to the vec
            if (**node).get_type() == AstType::Variable {
                let var_ptr = *node as *mut Variable;
                elems.push(SymbolElem {
                    identifier: (*var_ptr).identifier.clone(),
                    scope: String::from("Global"),
                    elem_type: (*var_ptr).var_type.as_ref().unwrap().clone(),
                    node: *node,
                });
            }

            // Fucntions require that you add them and their body (the body requires that you
            // change the scope)
            if (**node).get_type() == AstType::Function {
                let func_ptr = *node as *mut Function;
                elems.push(SymbolElem {
                    identifier: (*func_ptr).identifier.clone(),
                    scope: String::from("Global"),
                    elem_type: (*func_ptr).return_type.as_ref().unwrap().clone(),
                    node: *node,
                });

                let mut inner_elems = traverse_ast_block(
                    (*func_ptr).body,
                    &format!(
                        "{}:{}:{}",
                        (**node).get_line().unwrap().filename,
                        (*func_ptr).identifier,
                        (**node).get_line().unwrap().line_num
                    ),
                );
                elems.append(&mut inner_elems);
            }

            // Treat Block types like the body of a Function
            if (**node).get_type() == AstType::Block {
                let mut inner_elems = traverse_ast_block(
                    *node as *mut Block,
                    &format!(
                        "{}:{}",
                        (**node).get_line().unwrap().filename,
                        (**node).get_line().unwrap().line_num
                    ),
                );
                elems.append(&mut inner_elems);
            }
        }
    }

    SymbolTable { elems }
}

/// Helper function for recursively traversing the AST
fn traverse_ast_block(block: *mut Block, scope_name: &str) -> Vec<SymbolElem> {
    let mut elems: Vec<SymbolElem> = Vec::new();

    unsafe {
        for node in (*block).body.iter() {
            // Variables are a simple case, just add them to the vec
            if (**node).get_type() == AstType::Variable {
                let var_ptr = *node as *mut Variable;
                elems.push(SymbolElem {
                    identifier: (*var_ptr).identifier.clone(),
                    scope: String::from(scope_name),
                    elem_type: (*var_ptr).var_type.as_ref().unwrap().clone(),
                    node: *node,
                });
            }

            // Fucntions require that you add them and their body (the body requires that you
            // change the scope)
            if (**node).get_type() == AstType::Function {
                let func_ptr = *node as *mut Function;
                elems.push(SymbolElem {
                    identifier: (*func_ptr).identifier.clone(),
                    scope: String::from(scope_name),
                    elem_type: (*func_ptr).return_type.as_ref().unwrap().clone(),
                    node: *node,
                });

                let mut inner_elems = traverse_ast_block(
                    (*func_ptr).body,
                    &format!(
                        "{}:{}:{}",
                        (**node).get_line().unwrap().filename,
                        (*func_ptr).identifier,
                        (**node).get_line().unwrap().line_num
                    ),
                );
                elems.append(&mut inner_elems);
            }

            // Treat Block types like the body of a Function
            if (**node).get_type() == AstType::Block {
                let mut inner_elems = traverse_ast_block(
                    *node as *mut Block,
                    &format!(
                        "{}:{}",
                        (**node).get_line().unwrap().filename,
                        (**node).get_line().unwrap().line_num
                    ),
                );
                elems.append(&mut inner_elems);
            }
        }
    }

    elems
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
                && elem_i.elem_type != elem_j.elem_type
            {
                unsafe {
                    print_err(
                        &(*elem_j.node).get_line().unwrap(),
                        &format!(
                            "Missmatched types! \nExpected {:?}, but got {:?}",
                            elem_i.elem_type, elem_j.elem_type
                        ),
                        Some(&format!(
                            "Change {:?} {} to {:?} {}",
                            elem_j.elem_type,
                            elem_j.identifier,
                            elem_i.elem_type,
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
