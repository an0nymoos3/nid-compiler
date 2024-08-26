/*
* This file contains logic for parsing a hardware config file. This enable the compiler to work on
* a wider range of hardware, given that the hardware implements the required ASS instructions.
*
* Currently only TOML files are supported as config files.
*
* NOTE: Currently only mem_addresses & registers actually affects compilation.
* TODO: Implement support for extended / custom intructions.
*/

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use toml;

#[derive(Serialize, Deserialize)]
pub struct Hardware {
    pub mem_addresses: u16,          // Number of memory addresses available
    pub registers: u8,               // Number of registers available
    pub extended_instructions: bool, // Whether or not to use extended instruction set
}

impl Hardware {
    /// Create a new hardware config from file
    pub fn from(filename: &Path) -> Self {
        let content: String =
            fs::read_to_string(filename).expect("Failed to read hardware config file!");
        let hardware_conf: Hardware =
            toml::from_str(content.trim()).expect("Failed to parse toml as hardware config!");
        hardware_conf
    }
}

impl Default for Hardware {
    /// Defines the default hardware config
    fn default() -> Self {
        Self {
            mem_addresses: 256,
            registers: 8,
            extended_instructions: false,
        }
    }
}
