use std::path::Path;

use anyhow::Context;
use elf::{endian::AnyEndian, ElfBytes};

pub struct TrapMetadata {
    trap_symbol: u32,
}

impl TrapMetadata {
    /// Check for the first_trap_table symbol to infer trap information from PC
    pub fn from_elf(elf_file: &Path) -> anyhow::Result<Self> {
        // first_trap_table
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

    pub fn trap_class(&self, address: u32) -> Option<u8> {
        let offset_from_start = address.checked_sub(self.trap_symbol)?;

        if offset_from_start > 8 * 32 {
            return None;
        }

        let class = offset_from_start / 32;

        Some(class as u8)
    }
}
