use crate::mmu::Memory;

pub struct Serial {
    pub transfer: u8,
    pub control: u8,
}

impl Default for Serial {
    fn default() -> Self {
        Serial {
            transfer: 0,
            control: 0,
        }
    }
}

impl Memory for Serial {
    fn read(&self, address: u16) -> u8 {
        match address {
            0xFF01 => self.transfer,
            0xFF02 => self.control,
            _ => panic!("invalid serial read: {:#06x}", address),
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF01 => self.transfer = value,
            0xFF02 => self.control = value,
            _ => panic!("invalid serial write: {:#06x}", address),
        }
    }
}
