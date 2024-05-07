/*
* Handles general conversion from high-level (NID) to assembly (ASS) languge.
*/
use crate::compiler::ast::{self, Node};

/// Converts assignment in nid-lang to an equivalent instruction in ASS.
pub fn parse_assignment(
    var: ast::Variable,
    other_var: Option<ast::Variable>,
    value: Option<ast::Value>,
    type_specifier: Option<ast::ValueEnum>,
) -> Vec<String> {
    let mut instructions: Vec<String> = Vec::new();

    if let Some(o_var) = other_var {
        // Var assigned to var
        if let Some(type_spec) = &type_specifier {
            if let Some(var_type) = &var.var_type {
                if var_type != type_spec {
                    // Warn the user!
                    println!(
                        "Warning: Type of {} does not match {}",
                        var.get_name(),
                        o_var.get_name()
                    )
                }
            }
        }
    } else if let Some(val) = value {
        if let Some(var_type) = &var.var_type {
            // Value assigned to var
            if &val.value != var_type {
                // Warn the user!
                println!(
                    "Warning: Type of {} does not match {}! You should only do this if you know what you are doing.",
                    var.get_name(),
                    val.get_name()
                )
            }
        }
    }

    instructions
}
