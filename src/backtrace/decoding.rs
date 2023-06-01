use std::path::Path;

use tricore_common::backtrace::{csa::SavedContext, Stacktrace};

use super::{BackTraceInfo, StackFrameInfo, TrapInfo, addr2line::Addr2LineRegistry, trap_info::TrapMetadata};

pub trait ParseInfo {
    fn addr2line(&self, elf_file: &Path) -> anyhow::Result<BackTraceInfo>;
}

impl ParseInfo for Stacktrace {
    fn addr2line(&self, elf_file: &Path) -> anyhow::Result<BackTraceInfo> {
        let mut registry = Addr2LineRegistry::new(elf_file);
        let trap_metadata = TrapMetadata::from_elf(elf_file)?;

        registry.load(
            self.stack_frames
                .iter()
                .map(|ctx| ctx.return_address())
                .chain([self.current_pc].into_iter())
                .chain([self.current_upper.a11].into_iter()),
        )?;

        let mut stack_frames = Vec::new();

        let current_trapinfo = trap_metadata
            .trap_class(self.current_pc)
            .map(|class| TrapInfo {
                class,
                trap_id: self.current_upper.d15 as u8,
            });

        stack_frames.push(StackFrameInfo {
            address: self.current_pc,
            is_trap: current_trapinfo,
            info: registry.get_address_info(self.current_pc)?,
        });

        stack_frames.push(StackFrameInfo {
            address: self.current_upper.a11,
            is_trap: None,
            info: registry.get_address_info(self.current_upper.a11)?,
        });

        for ctx in self.stack_frames.iter() {
            let is_trap = if let SavedContext::Upper(ctx) = ctx {
                trap_metadata.trap_class(ctx.a11).map(|class| TrapInfo {
                    class,
                    trap_id: ctx.d15.try_into().unwrap(),
                })
            } else {
                None
            };

            stack_frames.push(StackFrameInfo {
                address: ctx.return_address(),
                is_trap,
                info: registry.get_address_info(ctx.return_address())?,
            })
        }

        Ok(BackTraceInfo { stack_frames })
    }
}
