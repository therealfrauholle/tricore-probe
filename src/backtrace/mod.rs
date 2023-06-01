//! This module implements decoding stack trace information and their displaying
use colored::{Color, Colorize};

use self::addr2line::Addr2LineInfo;

pub mod decoding;

mod addr2line;
mod trap_info;

pub struct BackTraceInfo {
    stack_frames: Vec<StackFrameInfo>,
}

impl BackTraceInfo {
    pub fn log_stdout(&self) {
        for f in self.stack_frames.iter() {
            f.log_stdout();
        }
    }
}

pub struct StackFrameInfo {
    address: u32,
    is_trap: Option<TrapInfo>,
    info: Addr2LineInfo,
}

#[derive(Debug)]
#[allow(dead_code)]
struct TrapInfo {
    class: u8,
    trap_id: u8,
}

impl StackFrameInfo {
    fn log_stdout(&self) {
        let address = self.address;
        let function = &self.info.function;
        let module = &self.info.module;
        let trap_info = self
            .is_trap
            .as_ref()
            .map(|info| format!("-> detected as trap handler {info:?}"))
            .unwrap_or_else(|| "".into());

        println!(
            "{} -> {} {}\n{}",
            format!("{address:#8X}").white(),
            function.bold().blue(),
            trap_info.bold().on_white().red(),
            format!("└────────── @ {module}").color(Color::TrueColor {
                r: 100,
                g: 100,
                b: 100
            })
        );
    }
}
