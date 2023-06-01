use std::{
    collections::HashMap,
    path::Path,
    process::{Command, Stdio},
};

use anyhow::Context;

#[derive(Clone)]
pub struct Addr2LineInfo {
    pub function: String,
    pub module: String,
}

pub struct Addr2LineRegistry<'a> {
    elf_file: &'a Path,
    registry: HashMap<u32, Addr2LineInfo>,
}

impl<'a> Addr2LineRegistry<'a> {
    pub fn new(elf_file: &'a Path) -> Self {
        Addr2LineRegistry {
            elf_file,
            registry: HashMap::new(),
        }
    }

    pub fn get_address_info(&mut self, address: u32) -> anyhow::Result<Addr2LineInfo> {
        let Some(info) = self.registry.get(&address) else {
            self.load([address].into_iter())?;
            return Ok(self.registry.get(&address).unwrap().clone())
        };

        Ok(info.clone())
    }

    pub fn load<I: Iterator<Item = u32>>(&mut self, addresses: I) -> anyhow::Result<()> {
        let mut defmt_print_process = Command::new("addr2line");
        let spawned_decoder = defmt_print_process
            .stdin(Stdio::piped())
            .stderr(Stdio::null())
            .stdout(Stdio::piped())
            .arg("-e")
            .arg(format!("{}", self.elf_file.display()))
            .arg("-f")
            .arg("-C");

        let addresses: Vec<u32> = addresses.collect();

        for a in addresses.iter() {
            spawned_decoder.arg(format!("{a:#X}"));
        }

        let addr2line_stdout = spawned_decoder
            .spawn()
            .with_context(|| "Cannot spawn addr2line to decode stack frame")?
            .wait_with_output()
            .with_context(|| "addr2line did not terminate properly")?
            .stdout;
        let string =
            String::from_utf8(addr2line_stdout).with_context(|| "Invalid addr2line output")?;
        let mut items: Vec<&str> = string.split('\n').collect();
        items.truncate(items.len() - 1);

        for (debug, address) in items.chunks_exact(2).zip(addresses.iter()) {
            self.registry.insert(
                *address,
                Addr2LineInfo {
                    function: debug[0].to_owned(),
                    module: debug[1].to_owned(),
                },
            );
        }

        Ok(())
    }
}
