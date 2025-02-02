use crate::cartridge::Cartridge;

pub trait ReadWriter {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}

pub struct Memory {
    cartridge: Cartridge,
}

impl Memory {
    pub fn new(cartridge: Cartridge) -> Self {
        Memory { cartridge }
    }

    pub fn map(&mut self, address: u16) -> &mut dyn ReadWriter {
        // https://gbdev.io/pandocs/Memory_Map.html
        match address {
            // 0x0000 - 0x3FFF : ROM Bank 0
            // 0x4000 - 0x7FFF : ROM Bank 1 - Switchable
            // 0x8000 - 0x97FF : CHR RAM
            // 0x9800 - 0x9BFF : BG Map 1
            // 0x9C00 - 0x9FFF : BG Map 2
            // 0xA000 - 0xBFFF : Cartridge RAM
            0xA000..=0xBFFF => &mut self.cartridge,
            // 0xC000 - 0xCFFF : RAM Bank 0
            // 0xD000 - 0xDFFF : RAM Bank 1-7 - switchable - Color only
            // 0xE000 - 0xFDFF : Reserved - Echo RAM
            // 0xFE00 - 0xFE9F : Object Attribute Memory
            // 0xFEA0 - 0xFEFF : Reserved - Unusable
            // 0xFF00 : Joypad input
            // 0xFF01 - 0xFF02 : Serial transfer
            // 0xFF04 - 0xFF07 : Timer and divider
            // 0xFF0F : Interrupt flag register
            // 0xFF10 - 0xFF26 : Audio
            // 0xFF30 - 0xFF3F : Wave pattern
            // 0xFF40 - 0xFF4B : LCD
            // 0xFF4D : CGB Speed Switch
            // 0xFF4F : VRAM Bank Select
            // 0xFF50 : Disable boot rom
            // 0xFF51 - 0xFF55 : VRAM DMA
            // 0xFF68 - 0xFF69 : BG / OBJ Palettes
            // 0xFF70 : WRAM Bank Select
            // 0xFF80 - 0xFFFE : Zero Page (HRAM)
            // 0xFFFF : Interrupt enable register
            _ => unimplemented!("unimplemented memory access: {:#06x}", address),
        }
    }

    pub fn read8(&mut self, address: u16) -> u8 {
        self.map(address).read(address)
    }

    pub fn write8(&mut self, address: u16, value: u8) {
        self.map(address).write(address, value);
    }

    pub fn read16(&mut self, address: u16) -> u16 {
        let low = self.read8(address) as u16;
        let high = self.read8(address + 1) as u16;
        (high << 8) | low
    }
}
