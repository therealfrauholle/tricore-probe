use std::path::Path;

use anyhow::Context;
use elf::{endian::AnyEndian, ElfBytes};

/// Information about where the trap table is located
pub struct TrapMetadata {
    /// The start of the trap table as an absolute address in code sapce
    trap_symbol: u32,
}

impl TrapMetadata {
    /// Infer information about the trap table by looking into the elf file
    /// 
    /// FIXME: This is a workaround that relies on implementation details to work,
    /// the proper way to provide this functionality is to read the BTV register on
    /// the device.
    pub fn from_elf(elf_file: &Path) -> anyhow::Result<Self> {
        let elf_data = std::fs::read(elf_file).unwrap();
        let elf = ElfBytes::<'_, AnyEndian>::minimal_parse(&elf_data).unwrap();

        let (symbols, strings) = elf
            .symbol_table()
            .with_context(|| "Could not parse symbol table from elf file")?
            .with_context(|| "Elf file does not have symbol table")?;

        let trap_symbol = symbols
            .iter()
            .find_map(|symbol| {
                let Ok(symbol_name) = strings.get(symbol.st_name as usize) else {
                    return None
                };

                if symbol_name != "first_trap_table" {
                    return None;
                }

                Some(symbol.st_value)
            })
            .with_context(|| "Elf file does not have 'first_trap_table' symbol")?;

        Ok(TrapMetadata {
            trap_symbol: trap_symbol.try_into().unwrap(),
        })
    }

    /// Attempt to infer the trap class based on the given program counter
    /// 
    /// The program counter must still point into the trap service routine defined int
    /// the trap table itself.
    pub fn trap_class(&self, program_counter: u32) -> Option<u8> {
        let offset_in_trap_table = program_counter.checked_sub(self.trap_symbol)?;

        if offset_in_trap_table > 8 * 32 {
            return None;
        }

        let class = offset_in_trap_table / 32;

        Some(class as u8)
    }
}
