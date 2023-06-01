use std::{fs, io::Write, path::Path};

use crate::elf::elf_to_hex;

pub use imp::Config;
use tricore_common::{backtrace::Stacktrace, Chip};

cfg_if::cfg_if! {
    if #[cfg(feature = "docker")] {
        #[cfg(feature = "windows")]
        compile_error!("Select 'docker' and 'windows' are mutually exclusive")
        use tricore_docker as imp;
    } else if #[cfg(feature = "windows")] {
        use tricore_windows as imp;
    } else {
        compile_error!("Select a backend through the 'docker' and 'windows' features")
    }
}

pub type ChipInterface = StandardChipInterface<imp::ChipInterface>;

/// Wraps a chip interface implementation
pub struct StandardChipInterface<C: Chip> {
    implementation: C,
}

impl<C: Chip> StandardChipInterface<C> {
    /// Initiate a new connection to a chip
    pub fn new(interface_configuration: C::Config) -> anyhow::Result<Self> {
        Ok(StandardChipInterface {
            implementation: C::new(interface_configuration)?,
        })
    }

    /// Like [Chip::flash_hex], but the binary is specified as a path to an elf
    /// file instead of provided as Intel hex in memory.
    pub fn flash_elf(&self, elf_file: &Path, halt_memtool: bool) -> anyhow::Result<()> {
        log::info!("Converting elf {} to hex file", elf_file.display());
        let elf_data = fs::read(elf_file).unwrap();
        let ihex = elf_to_hex(&elf_data)?;
        log::info!("Flashing hex file");
        self.implementation.flash_hex(ihex, halt_memtool)
    }

    /// Like [Chip::read_rtt]
    pub fn read_rtt<W: Write>(
        &self,
        rtt_control_block_address: u64,
        decoder: W,
    ) -> anyhow::Result<Stacktrace> {
        self.implementation
            .read_rtt(rtt_control_block_address, decoder)
    }
}
