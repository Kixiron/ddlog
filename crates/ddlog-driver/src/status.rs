use std::process::{ExitCode, Termination};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DDlogStatus {
    Success = 0,
    CompileError = 1,
    InternalError = 101,
}

impl Termination for DDlogStatus {
    #[inline]
    fn report(self) -> ExitCode {
        ExitCode::from(self as u8)
    }
}
