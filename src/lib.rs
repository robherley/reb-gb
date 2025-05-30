pub mod cartridge;
pub mod cpu;
mod interrupts;
mod metadata;
mod mmu;
mod registers;
mod serial;
mod timer;

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("invalid cartridge kind: {0:#04x}")]
    InvalidCartridgeKind(u8),
    #[error("invalid old licensee code: {0:#04x}")]
    InvalidOldLicenseeCode(u8),
    #[error("invalid new licensee code: {0}{1}")]
    InvalidNewLicenseeCode(char, char),
    #[error("cpu not supported: {0:?}")]
    CPUNotSupported(cpu::Model),
    #[error("illegal instruction: {0:#04X}")]
    IllegalInstruction(u8),
    #[error("invalid interrupt: {0:#04X}")]
    InvalidInterrupt(u8),
}
