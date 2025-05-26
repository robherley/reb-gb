use super::{Serial, Timer};
use crate::cartridge::Cartridge;

// Fixed size of each memory region
const WRAM_SIZE: usize = 0x2000;
const HRAM_SIZE: usize = 0x80;

// Offsets of where the memory region starts
const WRAM_OFFSET: u16 = 0xC000;
const HRAM_OFFSET: u16 = 0xFF80;

pub trait Memory {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}

pub struct Mapper {
    pub cartridge: Cartridge,
    pub ienable: Byte,
    pub iflag: Byte,
    pub noop: Noop,
    pub serial: Serial,
    pub wram: Ram,
    pub hram: Ram,
    pub timer: Timer,
    pub debug: bool,

    // TODO(robherley): cleanup after test rom debugging
    pub tmp_dummy_lcd: Byte,
}

impl Mapper {
    pub fn new(cartridge: Cartridge) -> Self {
        Mapper {
            cartridge,
            ienable: Byte::default(),
            iflag: Byte::default(),
            noop: Noop::new(false),
            serial: Serial::default(),
            wram: Ram::new(WRAM_SIZE, WRAM_OFFSET),
            hram: Ram::new(HRAM_SIZE, HRAM_OFFSET),
            timer: Timer::new(),
            debug: false,
            tmp_dummy_lcd: Byte::new(0x90, true),
        }
    }

    pub fn debug_serial(&mut self) {
        if !self.debug {
            return;
        }

        if self.serial.control == 0x81 {
            // TODO(robherley): implement serial callback
            eprint!("{}", self.serial.transfer as char);
            self.serial.control = 0x00;
        }
    }

    pub fn map(&mut self, address: u16) -> &mut dyn Memory {
        if self.debug && address == 0xFF44 {
            return &mut self.tmp_dummy_lcd;
        }

        // https://gbdev.io/pandocs/Memory_Map.html
        match address {
            // 0x0000 - 0x3FFF : ROM Bank 0
            // 0x4000 - 0x7FFF : ROM Bank 1 - Switchable
            0x0000..=0x7FFF => &mut self.cartridge,
            // 0x8000 - 0x97FF : CHR RAM
            // 0x9800 - 0x9BFF : BG Map 1
            // 0x9C00 - 0x9FFF : BG Map 2
            0x8000..=0x9FFF => &mut self.noop, // TODO(robherley): implement char map
            // 0xA000 - 0xBFFF : Cartridge RAM
            0xA000..=0xBFFF => &mut self.cartridge,
            // 0xC000 - 0xCFFF : RAM Bank 0
            // 0xD000 - 0xDFFF : RAM Bank 1-7 - switchable - Color only
            0xC000..=0xDFFF => &mut self.wram,
            // 0xE000 - 0xFDFF : Reserved - Echo RAM
            0xE000..=0xFDFF => panic!("reserved echo memory access: {:#06x}", address),
            // 0xFE00 - 0xFE9F : Object Attribute Memory
            0xFE00..=0xFE9F => &mut self.noop, // TODO(robherley): implement OAM
            // 0xFEA0 - 0xFEFF : Reserved - Unusable
            0xFEA0..=0xFEFF => panic!("reserved unusable memory access: {:#06x}", address),
            // 0xFF00 : Joypad input
            0xFF00 => &mut self.noop, // TODO(robherley): implement joypad
            // 0xFF01 - 0xFF02 : Serial transfer
            0xFF01..=0xFF02 => &mut self.serial,
            // 0xFF04 - 0xFF07 : Timer and divider
            0xFF04..=0xFF07 => &mut self.timer,
            // 0xFF0F : Interrupt flag register
            0xFF0F => &mut self.iflag,
            // 0xFF10 - 0xFF26 : Audio
            // 0xFF30 - 0xFF3F : Wave pattern
            0xFF10..=0xFF3F => &mut self.noop, // TODO(robherley): implement audio & wave pattern
            // 0xFF40 - 0xFF4B : LCD
            0xFF40..=0xFF4B => &mut self.noop, // TODO(robherley): implement LCD
            // 0xFF4D : CGB Speed Switch
            0xFF4D => &mut self.noop, // TODO(robherley): implement CGB speed switch
            // 0xFF4F : VRAM Bank Select
            0xFF4F => &mut self.noop, // TODO(robherley): implement VRAM bank select
            // 0xFF50 : Disable boot rom
            0xFF50 => &mut self.noop, // TODO(robherley): implement boot rom disable
            // 0xFF51 - 0xFF55 : VRAM DMA
            0xFF51..=0xFF55 => &mut self.noop, // TODO(robherley): implement VRAM DMA
            // 0xFF68 - 0xFF69 : BG / OBJ Palettes
            0xFF68..=0xFF69 => &mut self.noop, // TODO(robherley): implement bg obj palettes
            // 0xFF70 : WRAM Bank Select
            0xFF70 => &mut self.noop, // TODO(robherley): implement WRAM bank select
            // 0xFF80 - 0xFFFE : Zero Page (HRAM)
            0xFF80..=0xFFFE => &mut self.hram,
            // 0xFFFF : Interrupt enable register
            0xFFFF => &mut self.ienable,
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

    pub fn write16(&mut self, address: u16, value: u16) {
        let low = value as u8;
        let high = (value >> 8) as u8;
        self.write8(address, low);
        self.write8(address + 1, high);
    }
}

pub struct Byte {
    pub value: u8,
    pub read_only: bool,
}

impl Byte {
    pub fn new(value: u8, read_only: bool) -> Self {
        Byte { value, read_only }
    }
}

impl Default for Byte {
    fn default() -> Self {
        Byte {
            value: 0,
            read_only: false,
        }
    }
}

impl Memory for Byte {
    fn read(&self, _address: u16) -> u8 {
        self.value
    }

    fn write(&mut self, _address: u16, value: u8) {
        if self.read_only {
            return;
        }
        self.value = value;
    }
}

pub struct Noop {
    pub panic: bool,
}

impl Noop {
    pub fn new(panic: bool) -> Self {
        Noop { panic }
    }
}

impl Memory for Noop {
    fn read(&self, addr: u16) -> u8 {
        if self.panic {
            unimplemented!("noop serial read in panic mode: {:#06x}", addr);
        }
        0x00
    }

    fn write(&mut self, addr: u16, value: u8) {
        if self.panic {
            unimplemented!(
                "noop serial write in panic mode: {:#06x} to {:#06x}",
                value,
                addr
            );
        }
    }
}

pub struct Ram {
    memory: Vec<u8>,
    offset: u16,
}

impl Ram {
    pub fn new(size: usize, offset: u16) -> Self {
        Ram {
            memory: vec![0; size],
            offset,
        }
    }
}

impl Memory for Ram {
    fn read(&self, address: u16) -> u8 {
        self.memory[(address - self.offset) as usize]
    }

    fn write(&mut self, address: u16, value: u8) {
        self.memory[(address - self.offset) as usize] = value;
    }
}
