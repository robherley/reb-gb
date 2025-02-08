use super::cpu::Model;
use crate::cartridge::Cartridge;

/// Macro to set one or flags on the registers
#[macro_export]
macro_rules! flags {
  ($regs:expr, $( $flag:ident : $expr:expr ),* $(,)?) => {
      $(
          $regs.set_flag($flag, $expr);
      )*
  };
}

#[derive(Debug, Clone, Copy)]
pub enum Flags {
    /// Bit 7: Zero Flag (Z) - Set if the result of an operation is zero
    Z = 0b1000_0000,
    /// Bit 6: Subtract Flag (N) - Set if the last operation was a subtraction
    N = 0b0100_0000,
    /// Bit 5: Half Carry Flag (H) - Set if the last operation produced a carry from bit 3 to bit 4
    H = 0b0010_0000,
    /// Bit 4: Carry Flag (C) - Set if the last operation produced a carry from bit 7
    C = 0b0001_0000,
}

#[derive(PartialEq, Eq)]
/// CPU Registers can be accessed as single 16 bit OR separate 8 bit.
///
/// |16|Hi|Lo|
/// |--|--|--|
/// |AF|A |* |
/// |BC|B |C |
/// |DE|D |E |
/// |HL|H |L |
pub struct Registers {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,

    pub pc: u16,
    pub sp: u16,
}

impl std::fmt::Debug for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let flags: String = vec![Flags::Z, Flags::N, Flags::H, Flags::C]
            .iter()
            .map(|flag| {
                if self.flag(*flag) {
                    format!("{:?}", flag)
                } else {
                    "-".to_string()
                }
            })
            .collect();

        f.debug_struct("Registers")
            .field("a", &format!("{:#04X}", &self.a))
            .field("f", &flags)
            .field("b", &format!("{:#04X}", &self.b))
            .field("c", &format!("{:#04X}", &self.c))
            .field("d", &format!("{:#04X}", &self.d))
            .field("e", &format!("{:#04X}", &self.e))
            .field("h", &format!("{:#04X}", &self.h))
            .field("l", &format!("{:#04X}", &self.l))
            .field("pc", &format!("{:#06X}", &self.pc))
            .field("sp", &format!("{:#06X}", &self.sp))
            .finish()
    }
}

impl Registers {
    pub fn new(model: Model, cartridge: &Cartridge) -> Registers {
        // https://gbdev.io/pandocs/Power_Up_Sequence.html#cpu-registers
        let mut registers = Registers {
            a: 0x01,
            f: 0x00,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            pc: 0x0100,
            sp: 0xFFFE,
        };

        registers.set_flag(Flags::Z, true);

        match model {
            Model::DMG | Model::MGB => {
                // for DMG and MGB, the carry and half-carry flags are set if the checksum != 0
                if cartridge.header_checksum() != 0x00 {
                    registers.set_flag(Flags::H, true);
                    registers.set_flag(Flags::C, true);
                }
            } // TODO(robherley): implement the other models
        }

        registers
    }

    pub fn af(&self) -> u16 {
        (self.a as u16) << 8 | self.f as u16
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        // TODO(robherley): figure out if masking is necessary
        self.f = value as u8 & 0xF0;
    }

    pub fn bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    pub fn de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    pub fn hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }

    pub fn flag(&self, flag: Flags) -> bool {
        self.f & flag as u8 != 0
    }

    pub fn set_flag(&mut self, flag: Flags, value: bool) {
        if value {
            self.f |= flag as u8;
        } else {
            self.f &= !(flag as u8);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CPU_INSTRS_ROM: &[u8; 65536] = include_bytes!("../../test/fixtures/cpu_instrs.gb");

    #[test]
    fn test_init_dmg() {
        let cart = Cartridge::new(CPU_INSTRS_ROM.to_vec());
        let registers = Registers::new(Model::DMG, &cart);

        assert_eq!(
            registers,
            Registers {
                a: 0x01,
                f: 0xB0,
                b: 0x00,
                c: 0x13,
                d: 0x00,
                e: 0xD8,
                h: 0x01,
                l: 0x4D,
                pc: 0x0100,
                sp: 0xFFFE,
            }
        );
    }
}
