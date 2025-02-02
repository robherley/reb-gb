pub mod cpu {
    pub mod cpu;
    mod registers;
}

pub mod cartridge {
    mod cartridge;
    mod metadata;
    pub use cartridge::Cartridge;
}

pub mod mmu {
    mod memory;
    pub use memory::Memory;
    pub use memory::RW;
}
