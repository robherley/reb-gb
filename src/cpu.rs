use crate::cartridge::Cartridge;
use crate::registers::Registers;

#[derive(Debug, PartialEq, Eq)]
pub enum Model {
    // Original Game Boy
    DMG,
    // Game Boy Pocket
    MGB,
    // Game Boy Color
    CGB,
    // Super Game Boy
    SGB,
    // Super Game Boy 2
    SGB2,
    // Gameboy Advance
    AGB,
}

pub struct CPU {
    pub registers: Registers,
    pub cartridge: Cartridge,
}

impl CPU {
    pub fn new(model: Model, cart: Cartridge) -> CPU {
        CPU {
            registers: Registers::new(model, &cart),
            // TODO(robherley): mmu
            cartridge: cart,
        }
    }
}
