#![doc = include_str!("../README.md")]

use std::path::PathBuf;
use std::str::FromStr;
use anyhow::Context;
use clap::Parser;
use colored::Colorize;

use crate::backtrace::decoding::ParseInfo;
use crate::chip_interface::ChipInterface;
use crate::defmt::DefmtDecoder;

mod backtrace;
mod chip_interface;
mod defmt;
mod elf;

/// Rapidly prototype code on tricore chips
#[derive(Parser, Debug)]
struct Args {
    /// Set this flag to skip flashing; this just resets the chip and attaches the defmt decoder
    #[arg(long, default_value_t = false)]
    no_flash: bool,

    /// Path to the elf binary
    #[arg(value_parser = valid_path)]
    elf: PathBuf,

    /// Backend specific configuration
    #[command(flatten)]
    backend: chip_interface::Config,

    /// Stop after setting up the memtool. Memtool will stay open and tricore-probe
    /// will halt until memtool is closed by the user
    #[arg(long, default_value_t = false)]
    halt_memtool: bool,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

fn valid_path(input_path: &str) -> Result<PathBuf, anyhow::Error> {
    let path = PathBuf::from_str(input_path).with_context(|| "Value is not a correct path")?;

    Ok(path)
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    env_logger::init();

    if !args.verbose {
        log::set_max_level(log::LevelFilter::Info);
    }

    let command_server = ChipInterface::new(args.backend)?;

    if !args.no_flash {
        command_server.flash_elf(args.elf.as_path(), args.halt_memtool)?;
    } else {
        log::warn!("Flashing skipped - this might lead to malformed defmt data!")
    }

    let mut defmt_decoder = DefmtDecoder::spawn(args.elf.as_path())?;

    let backtrace = command_server.read_rtt(
        defmt_decoder.rtt_control_block_address(),
        &mut defmt_decoder,
    )?;

    let backtrace_info = backtrace.addr2line(args.elf.as_path())?;

    println!("{}", "Device halted, backtrace as follows".red());
    backtrace_info.log_stdout();

    Ok(()) as Result<(), anyhow::Error>
}
