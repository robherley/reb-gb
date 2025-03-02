use super::interrupts::Interrupts;
use super::registers::{Flags::*, Registers};
use crate::cartridge::Cartridge;
use crate::flags;
use crate::mmu::Memory;

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("cpu not supported: {0:?}")]
    CPUNotSupported(Model),
    #[error("illegal instruction: {0:#04X}")]
    IllegalInstruction(u8),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Model {
    /// Original Game Boy
    DMG,
    /// Game Boy Pocket
    MGB,
    // /// Game Boy Color
    // CGB,
    // /// Super Game Boy
    // SGB,
    // /// Super Game Boy 2
    // SGB2,
    // /// Gameboy Advance
    // AGB,
}

pub struct CPU {
    registers: Registers,
    mmu: Memory,
    halted: bool,
    interrupts: Interrupts,
}

impl CPU {
    pub fn new(model: Model, cartridge: Cartridge) -> CPU {
        CPU {
            registers: Registers::new(model, &cartridge),
            mmu: Memory::new(cartridge),
            halted: false,
            interrupts: Interrupts::default(),
        }
    }

    pub fn boot(&mut self) {
        loop {
            self.interrupts.update();

            if self.interrupts.ime {
                self.interrupt();
            }

            if self.halted {
                continue;
            }

            match self.next() {
                Ok(_cycles) => {
                    // TODO(robherley): implement clocks
                }
                Err(err) => {
                    eprintln!("crash: {:?}", err);
                    return;
                }
            }
        }
    }

    fn interrupt(&mut self) {
        // TODO(robherley): handle interrupts
    }

    fn fetch8(&mut self) -> u8 {
        let pc = self.registers.pc;
        self.registers.pc += 1;
        self.mmu.read8(pc)
    }

    fn fetch16(&mut self) -> u16 {
        let pc = self.registers.pc;
        self.registers.pc += 2;
        self.mmu.read16(pc)
    }

    /// Executes the next instruction and returns the number of t-cycles (system clock ticks) it took.
    /// https://gbdev.io/gb-opcodes/optables/
    fn next(&mut self) -> Result<usize, Error> {
        let cycles = match self.fetch8() {
            // NOP  | ----
            0x00 => 4,
            // LD BC, n16 | ----
            0x01 => {
                let value = self.fetch16();
                self.registers.set_bc(value);
                12
            }
            // LD [BC], A | ----
            0x02 => {
                self.mmu.write8(self.registers.bc(), self.registers.a);
                8
            }
            // INC BC | ----
            0x03 => {
                self.registers.set_bc(self.registers.bc().wrapping_add(1));
                8
            }
            // INC B | Z0H-
            0x04 => {
                self.registers.b = self.inc8(self.registers.b);
                4
            }
            // DEC B | Z1H-
            0x05 => {
                self.registers.b = self.dec8(self.registers.b);
                4
            }
            // LD B, n8 | ----
            0x06 => {
                self.registers.b = self.fetch8();
                8
            }
            // RLCA  | 000C
            0x07 => {
                self.registers.a = self.rlc(self.registers.a);
                4
            }
            // LD [a16], SP | ----
            0x08 => {
                let address = self.fetch16();
                self.mmu.write16(address, self.registers.sp);
                20
            }
            // ADD HL, BC | -0HC
            0x09 => {
                self.add16(self.registers.bc());
                8
            }
            // LD A, [BC] | ----
            0x0A => {
                self.registers.a = self.mmu.read8(self.registers.bc());
                4
            }
            // DEC BC | ----
            0x0B => {
                self.registers.set_bc(self.registers.bc().wrapping_sub(1));
                8
            }
            // INC C | Z0H-
            0x0C => {
                self.registers.c = self.inc8(self.registers.c);
                4
            }
            // DEC C | Z1H-
            0x0D => {
                self.registers.c = self.dec8(self.registers.c);
                4
            }
            // LD C, n8 | ----
            0x0E => {
                self.registers.c = self.fetch8();
                8
            }
            // RRCA  | 000C
            0x0F => {
                self.registers.a = self.rrc(self.registers.a);
                4
            }
            // STOP n8 | ----
            0x10 => {
                // TODO(robherley): 2.7.2. Stop Mode
                // https://gbdev.io/pandocs/Reducing_Power_Consumption.html?highlight=stop#using-the-stop-instruction
                // "No licensed rom makes use of STOP outside of CGB speed switching."
                4
            }
            // LD DE, n16 | ----
            0x11 => {
                let value = self.fetch16();
                self.registers.set_de(value);
                12
            }
            // LD [DE], A | ----
            0x12 => {
                self.mmu.write8(self.registers.de(), self.registers.a);
                8
            }
            // INC DE | ----
            0x13 => {
                self.registers.set_de(self.registers.de().wrapping_add(1));
                8
            }
            // INC D | Z0H-
            0x14 => {
                self.registers.c = self.inc8(self.registers.d);
                4
            }
            // DEC D | Z1H-
            0x15 => {
                self.registers.d = self.dec8(self.registers.d);
                4
            }
            // LD D, n8 | ----
            0x16 => {
                self.registers.d = self.fetch8();
                8
            }
            // RLA  | 000C
            0x17 => {
                self.registers.a = self.rl(self.registers.a);
                4
            }
            // JR e8 | ----
            0x18 => {
                self.jr();
                8
            }
            // ADD HL, DE | -0HC
            0x19 => {
                self.add16(self.registers.de());
                8
            }
            // LD A, [DE] | ----
            0x1A => {
                self.registers.a = self.mmu.read8(self.registers.de());
                4
            }
            // DEC DE | ----
            0x1B => {
                self.registers.set_de(self.registers.de().wrapping_sub(1));
                8
            }
            // INC E | Z0H-
            0x1C => {
                self.registers.e = self.inc8(self.registers.e);
                4
            }
            // DEC E | Z1H-
            0x1D => {
                self.registers.e = self.dec8(self.registers.e);
                4
            }
            // LD E, n8 | ----
            0x1E => {
                self.registers.e = self.fetch8();
                8
            }
            // RRA  | 000C
            0x1F => {
                self.registers.a = self.rr(self.registers.a);
                4
            }
            // JR NZ, e8 | ----
            0x20 => {
                if self.registers.flag(Z) {
                    8
                } else {
                    self.jr();
                    12
                }
            }
            // LD HL, n16 | ----
            0x21 => {
                let value = self.fetch16();
                self.registers.set_hl(value);
                12
            }
            // LD [HL+], A | ----
            // Put A into memory address HL. Increment HL. Same as: LD HL,A - INC HL
            0x22 => {
                self.mmu.write8(self.registers.hl(), self.registers.a);
                self.registers.set_hl(self.registers.hl().wrapping_add(1));
                8
            }
            // INC HL | ----
            0x23 => {
                self.registers.set_hl(self.registers.hl().wrapping_add(1));
                8
            }
            // INC H | Z0H-
            0x24 => {
                self.registers.h = self.inc8(self.registers.h);
                4
            }
            // DEC H | Z1H-
            0x25 => {
                self.registers.h = self.dec8(self.registers.h);
                4
            }
            // LD H, n8 | ----
            0x26 => {
                self.registers.h = self.fetch8();
                8
            }
            // DAA  | Z-0C
            0x27 => {
                self.daa();
                4
            }
            // JR Z, e8 | ----
            0x28 => {
                if self.registers.flag(Z) {
                    self.jr();
                    12
                } else {
                    8
                }
            }
            // ADD HL, HL | -0HC
            0x29 => {
                self.add16(self.registers.hl());
                8
            }
            // LD A, [HL+] | ----
            // Put value at address HL into A. Increment HL. Same as: LD A,HL - INC HL
            0x2A => {
                self.registers.a = self.mmu.read8(self.registers.hl());
                self.registers.set_hl(self.registers.hl().wrapping_add(1));
                8
            }
            // DEC HL | ----
            0x2B => {
                self.registers.set_hl(self.registers.hl().wrapping_sub(1));
                8
            }
            // INC L | Z0H-
            0x2C => {
                self.registers.l = self.inc8(self.registers.l);
                4
            }
            // DEC L | Z1H-
            0x2D => {
                self.registers.l = self.dec8(self.registers.l);
                4
            }
            // LD L, n8 | ----
            0x2E => {
                self.registers.l = self.fetch8();
                8
            }
            // CPL  | -11-
            0x2F => {
                self.registers.a = !self.registers.a;
                flags!(self.registers,
                    N: true,
                    H: true,
                );
                4
            }
            // JR NC, e8 | ----
            0x30 => {
                if self.registers.flag(C) {
                    8
                } else {
                    self.jr();
                    12
                }
            }
            // LD SP, n16 | ----
            0x31 => {
                self.registers.sp = self.fetch16();
                12
            }
            // LD [HL+], A | ----
            // Put A into memory address HL. Decrement HL. Same as: LD HL,A - DEC HL
            0x32 => {
                self.mmu.write8(self.registers.hl(), self.registers.a);
                self.registers.set_hl(self.registers.hl().wrapping_sub(1));
                8
            }
            // INC SP | ----
            0x33 => {
                self.registers.sp = self.registers.sp.wrapping_add(1);
                8
            }
            // INC [HL] | Z0H-
            0x34 => {
                let addr = self.registers.hl();
                let value = self.mmu.read8(addr);
                let result = self.inc8(value);
                self.mmu.write8(addr, result);
                12
            }
            // DEC [HL] | Z1H-
            0x35 => {
                let addr = self.registers.hl();
                let value = self.mmu.read8(addr);
                let result = self.dec8(value);
                self.mmu.write8(addr, result);
                12
            }
            // LD [HL], n8 | ----
            0x36 => {
                let value = self.fetch8();
                self.mmu.write8(self.registers.hl(), value);
                12
            }
            // SCF  | -001
            0x37 => {
                flags!(self.registers,
                    N: false,
                    H: false,
                    C: true,
                );
                4
            }
            // JR C, e8 | ----
            0x38 => {
                if self.registers.flag(C) {
                    self.jr();
                    12
                } else {
                    8
                }
            }
            // ADD HL, SP | -0HC
            0x39 => {
                self.add16(self.registers.sp);
                8
            }
            // LD A, [HL-] | ----
            // Put value at address HL into A. Decrement HL. Same as: LD A,HL - DEC HL
            0x3A => {
                self.registers.a = self.mmu.read8(self.registers.hl());
                self.registers.set_hl(self.registers.hl().wrapping_sub(1));
                8
            }
            // DEC SP | ----
            0x3B => {
                self.registers.sp = self.registers.sp.wrapping_sub(1);
                8
            }
            // INC A | Z0H-
            0x3C => {
                self.registers.a = self.inc8(self.registers.a);
                4
            }
            // DEC A | Z1H-
            0x3D => {
                self.registers.a = self.dec8(self.registers.a);
                4
            }
            // LD A, n8 | ----
            0x3E => {
                self.registers.a = self.fetch8();
                8
            }
            // CCF  | -00C
            0x3F => {
                flags!(self.registers,
                    N: false,
                    H: false,
                    C: !self.registers.flag(C),
                );
                4
            }
            // LD B, B | ----
            0x40 => 4,
            // LD B, C | ----
            0x41 => {
                self.registers.b = self.registers.c;
                4
            }
            // LD B, D | ----
            0x42 => {
                self.registers.b = self.registers.d;
                4
            }
            // LD B, E | ----
            0x43 => {
                self.registers.b = self.registers.e;
                4
            }
            // LD B, H | ----
            0x44 => {
                self.registers.b = self.registers.h;
                4
            }
            // LD B, L | ----
            0x45 => {
                self.registers.b = self.registers.l;
                4
            }
            // LD B, [HL] | ----
            0x46 => {
                self.registers.b = self.mmu.read8(self.registers.hl());
                8
            }
            // LD B, A | ----
            0x47 => {
                self.registers.b = self.registers.a;
                4
            }
            // LD C, B | ----
            0x48 => {
                self.registers.c = self.registers.b;
                4
            }
            // LD C, C | ----
            0x49 => 4,
            // LD C, D | ----
            0x4A => {
                self.registers.c = self.registers.d;
                4
            }
            // LD C, E | ----
            0x4B => {
                self.registers.c = self.registers.e;
                4
            }
            // LD C, H | ----
            0x4C => {
                self.registers.c = self.registers.h;
                4
            }
            // LD C, L | ----
            0x4D => {
                self.registers.c = self.registers.l;
                4
            }
            // LD C, [HL] | ----
            0x4E => {
                self.registers.c = self.mmu.read8(self.registers.hl());
                8
            }
            // LD C, A | ----
            0x4F => {
                self.registers.c = self.registers.a;
                4
            }
            // LD D, B | ----
            0x50 => {
                self.registers.d = self.registers.b;
                4
            }
            // LD D, C | ----
            0x51 => {
                self.registers.d = self.registers.c;
                4
            }
            // LD D, D | ----
            0x52 => 4,
            // LD D, E | ----
            0x53 => {
                self.registers.d = self.registers.e;
                4
            }
            // LD D, H | ----
            0x54 => {
                self.registers.d = self.registers.h;
                4
            }
            // LD D, L | ----
            0x55 => {
                self.registers.d = self.registers.l;
                4
            }
            // LD D, [HL] | ----
            0x56 => {
                self.registers.d = self.mmu.read8(self.registers.hl());
                8
            }
            // LD D, A | ----
            0x57 => {
                self.registers.d = self.registers.a;
                4
            }
            // LD E, B | ----
            0x58 => {
                self.registers.e = self.registers.b;
                4
            }
            // LD E, C | ----
            0x59 => {
                self.registers.e = self.registers.c;
                4
            }
            // LD E, D | ----
            0x5A => {
                self.registers.e = self.registers.d;
                4
            }
            // LD E, E | ----
            0x5B => 4,
            // LD E, H | ----
            0x5C => {
                self.registers.e = self.registers.h;
                4
            }
            // LD E, L | ----
            0x5D => {
                self.registers.e = self.registers.l;
                4
            }
            // LD E, [HL] | ----
            0x5E => {
                self.registers.e = self.mmu.read8(self.registers.hl());
                8
            }
            // LD E, A | ----
            0x5F => {
                self.registers.e = self.registers.a;
                4
            }
            // LD H, B | ----
            0x60 => {
                self.registers.h = self.registers.b;
                4
            }
            // LD H, C | ----
            0x61 => {
                self.registers.h = self.registers.c;
                4
            }
            // LD H, D | ----
            0x62 => {
                self.registers.h = self.registers.d;
                4
            }
            // LD H, E | ----
            0x63 => {
                self.registers.h = self.registers.e;
                4
            }
            // LD H, H | ----
            0x64 => 4,
            // LD H, L | ----
            0x65 => {
                self.registers.h = self.registers.l;
                4
            }
            // LD H, [HL] | ----
            0x66 => {
                self.registers.h = self.mmu.read8(self.registers.hl());
                8
            }
            // LD H, A | ----
            0x67 => {
                self.registers.h = self.registers.a;
                4
            }
            // LD L, B | ----
            0x68 => {
                self.registers.l = self.registers.b;
                4
            }
            // LD L, C | ----
            0x69 => {
                self.registers.l = self.registers.c;
                4
            }
            // LD L, D | ----
            0x6A => {
                self.registers.l = self.registers.d;
                4
            }
            // LD L, E | ----
            0x6B => {
                self.registers.l = self.registers.e;
                4
            }
            // LD L, H | ----
            0x6C => {
                self.registers.l = self.registers.h;
                4
            }
            // LD L, L | ----
            0x6D => 4,
            // LD L, [HL] | ----
            0x6E => {
                self.registers.l = self.mmu.read8(self.registers.hl());
                8
            }
            // LD L, A | ----
            0x6F => {
                self.registers.l = self.registers.a;
                4
            }
            // LD [HL], B | ----
            0x70 => {
                self.mmu.write8(self.registers.hl(), self.registers.b);
                8
            }
            // LD [HL], C | ----
            0x71 => {
                self.mmu.write8(self.registers.hl(), self.registers.c);
                8
            }
            // LD [HL], D | ----
            0x72 => {
                self.mmu.write8(self.registers.hl(), self.registers.d);
                8
            }
            // LD [HL], E | ----
            0x73 => {
                self.mmu.write8(self.registers.hl(), self.registers.e);
                8
            }
            // LD [HL], H | ----
            0x74 => {
                self.mmu.write8(self.registers.hl(), self.registers.h);
                8
            }
            // LD [HL], L | ----
            0x75 => {
                self.mmu.write8(self.registers.hl(), self.registers.l);
                8
            }
            // HALT  | ----
            0x76 => {
                self.halted = true;
                1
            }
            // LD [HL], A | ----
            0x77 => {
                self.mmu.write8(self.registers.hl(), self.registers.a);
                8
            }
            // LD A, B | ----
            0x78 => {
                self.registers.a = self.registers.b;
                4
            }
            // LD A, C | ----
            0x79 => {
                self.registers.a = self.registers.c;
                4
            }
            // LD A, D | ----
            0x7A => {
                self.registers.a = self.registers.d;
                4
            }
            // LD A, E | ----
            0x7B => {
                self.registers.a = self.registers.e;
                4
            }
            // LD A, H | ----
            0x7C => {
                self.registers.a = self.registers.h;
                4
            }
            // LD A, L | ----
            0x7D => {
                self.registers.a = self.registers.l;
                4
            }
            // LD A, [HL] | ----
            0x7E => {
                self.registers.a = self.mmu.read8(self.registers.hl());
                8
            }
            // LD A, A | ----
            0x7F => 4,
            // ADD A, B | Z0HC
            0x80 => {
                self.add8(self.registers.b);
                4
            }
            // ADD A, C | Z0HC
            0x81 => {
                self.add8(self.registers.c);
                4
            }
            // ADD A, D | Z0HC
            0x82 => {
                self.add8(self.registers.d);
                4
            }
            // ADD A, E | Z0HC
            0x83 => {
                self.add8(self.registers.e);
                4
            }
            // ADD A, H | Z0HC
            0x84 => {
                self.add8(self.registers.h);
                4
            }
            // ADD A, L | Z0HC
            0x85 => {
                self.add8(self.registers.l);
                4
            }
            // ADD A, [HL] | Z0HC
            0x86 => {
                let value = self.mmu.read8(self.registers.hl());
                self.add8(value);
                8
            }
            // ADD A, A | Z0HC
            0x87 => {
                self.add8(self.registers.a);
                4
            }
            // ADC A, B | Z0HC
            0x88 => {
                self.adc8(self.registers.b);
                4
            }
            // ADC A, C | Z0HC
            0x89 => {
                self.adc8(self.registers.c);
                4
            }
            // ADC A, D | Z0HC
            0x8A => {
                self.adc8(self.registers.d);
                4
            }
            // ADC A, E | Z0HC
            0x8B => {
                self.adc8(self.registers.e);
                4
            }
            // ADC A, H | Z0HC
            0x8C => {
                self.adc8(self.registers.h);
                4
            }
            // ADC A, L | Z0HC
            0x8D => {
                self.adc8(self.registers.l);
                4
            }
            // ADC A, [HL] | Z0HC
            0x8E => {
                let value = self.mmu.read8(self.registers.hl());
                self.adc8(value);
                8
            }
            // ADC A, A | Z0HC
            0x8F => {
                self.adc8(self.registers.a);
                4
            }
            // SUB A, B | Z1HC
            0x90 => {
                self.sub8(self.registers.b);
                4
            }
            // SUB A, C | Z1HC
            0x91 => {
                self.sub8(self.registers.c);
                4
            }
            // SUB A, D | Z1HC
            0x92 => {
                self.sub8(self.registers.d);
                4
            }
            // SUB A, E | Z1HC
            0x93 => {
                self.sub8(self.registers.e);
                4
            }
            // SUB A, H | Z1HC
            0x94 => {
                self.sub8(self.registers.h);
                4
            }
            // SUB A, L | Z1HC
            0x95 => {
                self.sub8(self.registers.l);
                4
            }
            // SUB A, [HL] | Z1HC
            0x96 => {
                let value = self.mmu.read8(self.registers.hl());
                self.sub8(value);
                8
            }
            // SUB A, A | 1100
            0x97 => {
                self.sub8(self.registers.a);
                4
            }
            // SBC A, B | Z1HC
            0x98 => {
                self.sbc8(self.registers.b);
                4
            }
            // SBC A, C | Z1HC
            0x99 => {
                self.sbc8(self.registers.c);
                4
            }
            // SBC A, D | Z1HC
            0x9A => {
                self.sbc8(self.registers.d);
                4
            }
            // SBC A, E | Z1HC
            0x9B => {
                self.sbc8(self.registers.e);
                4
            }
            // SBC A, H | Z1HC
            0x9C => {
                self.sbc8(self.registers.h);
                4
            }
            // SBC A, L | Z1HC
            0x9D => {
                self.sbc8(self.registers.l);
                4
            }
            // SBC A, [HL] | Z1HC
            0x9E => {
                let value = self.mmu.read8(self.registers.hl());
                self.sbc8(value);
                8
            }
            // SBC A, A | Z1H-
            0x9F => {
                self.sbc8(self.registers.a);
                4
            }
            // AND A, B | Z010
            0xA0 => {
                self.and8(self.registers.b);
                4
            }
            // AND A, C | Z010
            0xA1 => {
                self.and8(self.registers.c);
                4
            }
            // AND A, D | Z010
            0xA2 => {
                self.and8(self.registers.d);
                4
            }
            // AND A, E | Z010
            0xA3 => {
                self.and8(self.registers.e);
                4
            }
            // AND A, H | Z010
            0xA4 => {
                self.and8(self.registers.h);
                4
            }
            // AND A, L | Z010
            0xA5 => {
                self.and8(self.registers.l);
                4
            }
            // AND A, [HL] | Z010
            0xA6 => {
                let value = self.mmu.read8(self.registers.hl());
                self.and8(value);
                8
            }
            // AND A, A | Z010
            0xA7 => {
                self.and8(self.registers.a);
                4
            }
            // XOR A, B | Z000
            0xA8 => {
                self.xor8(self.registers.b);
                4
            }
            // XOR A, C | Z000
            0xA9 => {
                self.xor8(self.registers.c);
                4
            }
            // XOR A, D | Z000
            0xAA => {
                self.xor8(self.registers.d);
                4
            }
            // XOR A, E | Z000
            0xAB => {
                self.xor8(self.registers.e);
                4
            }
            // XOR A, H | Z000
            0xAC => {
                self.xor8(self.registers.h);
                4
            }
            // XOR A, L | Z000
            0xAD => {
                self.xor8(self.registers.l);
                4
            }
            // XOR A, [HL] | Z000
            0xAE => {
                let value = self.mmu.read8(self.registers.hl());
                self.xor8(value);
                8
            }
            // XOR A, A | 1000
            0xAF => {
                self.xor8(self.registers.a);
                4
            }
            // OR A, B | Z000
            0xB0 => {
                self.or8(self.registers.b);
                4
            }
            // OR A, C | Z000
            0xB1 => {
                self.or8(self.registers.c);
                4
            }
            // OR A, D | Z000
            0xB2 => {
                self.or8(self.registers.d);
                4
            }
            // OR A, E | Z000
            0xB3 => {
                self.or8(self.registers.e);
                4
            }
            // OR A, H | Z000
            0xB4 => {
                self.or8(self.registers.h);
                4
            }
            // OR A, L | Z000
            0xB5 => {
                self.or8(self.registers.l);
                4
            }
            // OR A, [HL] | Z000
            0xB6 => {
                let value = self.mmu.read8(self.registers.hl());
                self.or8(value);
                8
            }
            // OR A, A | Z000
            0xB7 => {
                self.or8(self.registers.a);
                4
            }
            // CP A, B | Z1HC
            0xB8 => {
                self.cp8(self.registers.b);
                4
            }
            // CP A, C | Z1HC
            0xB9 => {
                self.cp8(self.registers.c);
                4
            }
            // CP A, D | Z1HC
            0xBA => {
                self.cp8(self.registers.d);
                4
            }
            // CP A, E | Z1HC
            0xBB => {
                self.cp8(self.registers.e);
                4
            }
            // CP A, H | Z1HC
            0xBC => {
                self.cp8(self.registers.h);
                4
            }
            // CP A, L | Z1HC
            0xBD => {
                self.cp8(self.registers.l);
                4
            }
            // CP A, [HL] | Z1HC
            0xBE => {
                let value = self.mmu.read8(self.registers.hl());
                self.cp8(value);
                8
            }
            // CP A, A | 1100
            0xBF => {
                self.cp8(self.registers.a);
                4
            }
            // RET NZ | ----
            0xC0 => unimplemented!(),
            // POP BC | ----
            0xC1 => {
                let value = self.pop();
                self.registers.set_bc(value);
                12
            }
            // JP NZ, a16 | ----
            0xC2 => {
                let addr = self.fetch16();
                if !self.registers.flag(Z) {
                    self.registers.pc = addr;
                    16
                } else {
                    12
                }
            }
            // JP a16 | ----
            0xC3 => {
                let addr = self.fetch16();
                self.registers.pc = addr;
                16
            }
            // CALL NZ, a16 | ----
            0xC4 => unimplemented!(),
            // PUSH BC | ----
            0xC5 => {
                self.push(self.registers.bc());
                16
            }
            // ADD A, n8 | Z0HC
            0xC6 => {
                let value = self.fetch8();
                self.add8(value);
                8
            }
            // RST $00 | ----
            0xC7 => unimplemented!(),
            // RET Z | ----
            0xC8 => unimplemented!(),
            // RET  | ----
            0xC9 => unimplemented!(),
            // JP Z, a16 | ----
            0xCA => {
                let addr = self.fetch16();
                if self.registers.flag(Z) {
                    self.registers.pc = addr;
                    16
                } else {
                    12
                }
            }
            // PREFIX  | ----
            0xCB => self.cb(),
            // CALL Z, a16 | ----
            0xCC => unimplemented!(),
            // CALL a16 | ----
            0xCD => unimplemented!(),
            // ADC A, n8 | Z0HC
            0xCE => {
                let value = self.fetch8();
                self.adc8(value);
                8
            }
            // RST $08 | ----
            0xCF => unimplemented!(),
            // RET NC | ----
            0xD0 => unimplemented!(),
            // POP DE | ----
            0xD1 => {
                let value = self.pop();
                self.registers.set_de(value);
                12
            }
            // JP NC, a16 | ----
            0xD2 => {
                let addr = self.fetch16();
                if !self.registers.flag(C) {
                    self.registers.pc = addr;
                    16
                } else {
                    12
                }
            }
            // ILLEGAL(0xD3) | ----
            0xD3 => return Err(Error::IllegalInstruction(0xD3)),
            // CALL NC, a16 | ----
            0xD4 => unimplemented!(),
            // PUSH DE | ----
            0xD5 => {
                self.push(self.registers.de());
                16
            }
            // SUB A, n8 | Z1HC
            0xD6 => {
                let value = self.fetch8();
                self.sub8(value);
                8
            }
            // RST $10 | ----
            0xD7 => unimplemented!(),
            // RET C | ----
            0xD8 => unimplemented!(),
            // RETI  | ----
            0xD9 => {
                self.registers.pc = self.pop();
                self.interrupts.enable(true);
                16
            }
            // JP C, a16 | ----
            0xDA => {
                let addr = self.fetch16();
                if self.registers.flag(C) {
                    self.registers.pc = addr;
                    16
                } else {
                    12
                }
            }
            // ILLEGAL(0xDB) | ----
            0xDB => return Err(Error::IllegalInstruction(0xDB)),
            // CALL C, a16 | ----
            0xDC => unimplemented!(),
            // ILLEGAL(0xDD) | ----
            0xDD => return Err(Error::IllegalInstruction(0xDD)),
            // SBC A, n8 | Z1HC
            0xDE => {
                let value = self.fetch8();
                self.sbc8(value);
                8
            }
            // RST $18 | ----
            0xDF => unimplemented!(),
            // LDH [a8], A | ----
            // Put A into memory address $FF00+n
            0xE0 => {
                let addr = 0xFF00 + self.fetch8() as u16;
                self.mmu.write8(addr, self.registers.a);
                12
            }
            // POP HL | ----
            0xE1 => {
                let value = self.pop();
                self.registers.set_hl(value);
                12
            }
            // LD [C], A | ----
            0xE2 => {
                self.mmu
                    .write8(0xFF00 + self.registers.c as u16, self.registers.a);
                8
            }
            // ILLEGAL(0xE3) | ----
            0xE3 => return Err(Error::IllegalInstruction(0xE3)),
            // ILLEGAL(0xE4) | ----
            0xE4 => return Err(Error::IllegalInstruction(0xE4)),
            // PUSH HL | ----
            0xE5 => {
                self.push(self.registers.hl());
                16
            }
            // AND A, n8 | Z010
            0xE6 => {
                let value = self.fetch8();
                self.and8(value);
                8
            }
            // RST $20 | ----
            0xE7 => unimplemented!(),
            // ADD SP, e8 | 00HC
            0xE8 => {
                let value = self.fetch8() as u16;
                let sp = self.registers.sp;
                self.registers.sp = sp.wrapping_add(value);
                flags!(self.registers,
                    Z: false,
                    N: false,
                    H: (sp & 0x0FFF) + (value & 0x0FFF) > 0x0FFF,
                    C: (sp as u32 + value as u32) > 0xFFFF
                );
                16
            }
            // JP HL | ----
            0xE9 => {
                self.registers.pc = self.registers.hl();
                4
            }
            // LD [a16], A | ----
            0xEA => {
                let addr = self.fetch16();
                self.mmu.write8(addr, self.registers.a);
                16
            }
            // ILLEGAL(0xEB) | ----
            0xEB => return Err(Error::IllegalInstruction(0xEB)),
            // ILLEGAL(0xEC) | ----
            0xEC => return Err(Error::IllegalInstruction(0xEC)),
            // ILLEGAL(0xED) | ----
            0xED => return Err(Error::IllegalInstruction(0xED)),
            // XOR A, n8 | Z000
            0xEE => {
                let value = self.fetch8();
                self.xor8(value);
                8
            }
            // RST $28 | ----
            0xEF => unimplemented!(),
            // LDH A, [a8] | ----
            // Put memory address $FF00+n into A
            0xF0 => {
                let addr = 0xFF00 + self.fetch8() as u16;
                self.registers.a = self.mmu.read8(addr);
                12
            }
            // POP AF | ZNHC
            0xF1 => {
                let value = self.pop();
                self.registers.set_af(value);
                12
            }
            // LD A, [C] | ----
            0xF2 => {
                self.registers.a = self.mmu.read8(0xFF00 + self.registers.c as u16);
                8
            }
            // DI  | ----
            0xF3 => {
                self.interrupts.disable();
                4
            }
            // ILLEGAL(0xF4) | ----
            0xF4 => return Err(Error::IllegalInstruction(0xF4)),
            // PUSH AF | ----
            0xF5 => {
                self.push(self.registers.af());
                16
            }
            // OR A, n8 | Z000
            0xF6 => {
                let value = self.fetch8();
                self.or8(value);
                8
            }
            // RST $30 | ----
            0xF7 => unimplemented!(),
            // LD HL, SP, e8 | 00HC
            0xF8 => {
                let value = self.fetch8() as u16;
                let sp = self.registers.sp;
                let result = sp.wrapping_add(value);
                self.registers.set_hl(result);
                flags!(self.registers,
                    Z: false,
                    N: false,
                    H: (sp & 0x0FFF) + (value & 0x0FFF) > 0x0FFF,
                    C: (sp as u32 + value as u32) > 0xFFFF
                );

                12
            }
            // LD SP, HL | ----
            0xF9 => {
                self.registers.sp = self.registers.hl();
                8
            }
            // LD A, [a16] | ----
            0xFA => {
                let addr = self.fetch16();
                self.registers.a = self.mmu.read8(addr);
                16
            }
            // EI  | ----
            0xFB => {
                self.interrupts.enable(false);
                4
            }
            // ILLEGAL(0xFC) | ----
            0xFC => return Err(Error::IllegalInstruction(0xFC)),
            // ILLEGAL(0xFD) | ----
            0xFD => return Err(Error::IllegalInstruction(0xFD)),
            // CP A, n8 | Z1HC
            0xFE => {
                let value = self.fetch8();
                self.cp8(value);
                8
            }
            // RST $38 | ----
            0xFF => unimplemented!(),
        };

        Ok(cycles)
    }

    fn cb(&mut self) -> usize {
        match self.fetch8() {
            _ => unimplemented!(),
        }
    }

    // 8-bit increment
    fn inc8(&mut self, value: u8) -> u8 {
        let result = value.wrapping_add(1);
        flags!(self.registers,
          Z: result == 0,
          N: false,
          H: (value & 0x0F) == 0x0F
        );
        result
    }

    // 8-bit decrement
    fn dec8(&mut self, value: u8) -> u8 {
        let result = value.wrapping_sub(1);
        flags!(self.registers,
          Z: result == 0,
          N: true,
          H: (value & 0x0F) == 0
        );
        result
    }

    // 8-bit add (LHS is always register A)
    fn add8(&mut self, value: u8) {
        let result = self.registers.a.wrapping_add(value);
        flags!(self.registers,
          Z: result == 0,
          N: false,
          H: (self.registers.a & 0x0F) + (value & 0x0F) > 0x0F,
          C: (self.registers.a as u16 + value as u16) > 0xFF
        );
        self.registers.a = result;
    }

    // 8-bit add with carry (LHS is always register A)
    fn adc8(&mut self, value: u8) {
        self.add8(value.wrapping_add(self.registers.flag(C) as u8));
    }

    // 8-bit subtract (LHS is always register A)
    fn sub8(&mut self, value: u8) {
        let result = self.registers.a.wrapping_sub(value);
        flags!(self.registers,
          Z: result == 0,
          N: true,
          H: (self.registers.a & 0x0F) < (value & 0x0F),
          C: (self.registers.a as u16) < (value as u16)
        );
        self.registers.a = result;
    }

    // 8-bit subtract with carry (LHS is always register A)
    fn sbc8(&mut self, value: u8) {
        self.sub8(value.wrapping_add(self.registers.flag(C) as u8));
    }

    // 8-bit AND (LHS is always register A)
    fn and8(&mut self, value: u8) {
        let result = self.registers.a & value;
        flags!(self.registers,
          Z: result == 0,
          N: false,
          H: true,
          C: false
        );
        self.registers.a = result;
    }

    // 8-bit XOR (LHS is always register A)
    fn xor8(&mut self, value: u8) {
        let result = self.registers.a ^ value;
        flags!(self.registers,
          Z: result == 0,
          N: false,
          H: false,
          C: false
        );
        self.registers.a = result;
    }

    // 8-bit OR (LHS is always register A)
    fn or8(&mut self, value: u8) {
        let result = self.registers.a | value;
        flags!(self.registers,
          Z: result == 0,
          N: false,
          H: false,
          C: false
        );
        self.registers.a = result;
    }

    // 8-bit compare (LHS is always register A)
    // From CPU manual: "This is basically an A - n subtraction instruction but the results are thrown away."
    fn cp8(&mut self, value: u8) {
        let a = self.registers.a;
        self.sub8(value);
        self.registers.a = a;
    }

    // 16-bit add to HL
    // Reset N, set carry and half-carry flags
    fn add16(&mut self, value: u16) {
        let hl = self.registers.hl();
        self.registers.set_hl(hl.wrapping_add(value));
        flags!(self.registers,
          N: false,
          H: (hl & 0x0FFF) + (value & 0x0FFF) > 0x0FFF,
          C: (hl as u32 + value as u32) > 0xFFFF
        );
    }

    // Push register pair nn onto stack.
    // Decrement Stack Pointer (SP) twice.
    fn push(&mut self, value: u16) {
        // ref: 3.2.4. Stack Pointer
        // "The Stack Pointer automatically decrements before it puts something onto the stack"
        self.registers.sp = self.registers.sp.wrapping_sub(2);
        self.mmu.write16(self.registers.sp, value);
    }

    // Pop two bytes from the stack into register pair nn.
    // Increment Stack Pointer (SP) twice.
    fn pop(&mut self) -> u16 {
        let value = self.mmu.read16(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(2);
        value
    }

    // Rotate left
    // Old bit 7 to carry flag
    fn rlc(&mut self, value: u8) -> u8 {
        let carry = value & 0x80 != 0;
        let result = (value << 1) | carry as u8;
        flags!(self.registers,
          Z: result == 0,
          N: false,
          H: false,
          C: carry
        );
        result
    }

    // Rotate left _through_ carry
    // Old bit 7 to carry flag
    fn rl(&mut self, value: u8) -> u8 {
        let carry = value & 0x80 != 0;
        let result = (value << 1) | self.registers.flag(C) as u8;
        flags!(self.registers,
          Z: result == 0,
          N: false,
          H: false,
          C: carry
        );
        result
    }

    // Rotate right
    // Old bit 0 to carry flag
    fn rrc(&mut self, value: u8) -> u8 {
        let carry = value & 0x01 != 0;
        let result = (value >> 1) | (carry as u8) << 7;
        flags!(self.registers,
          Z: result == 0,
          N: false,
          H: false,
          C: carry
        );
        result
    }

    // Rotate right _through_ carry
    // Old bit 0 to carry flag
    fn rr(&mut self, value: u8) -> u8 {
        let carry = value & 0x01 != 0;
        let result = (value >> 1) | (self.registers.flag(C) as u8) << 7;
        flags!(self.registers,
          Z: result == 0,
          N: false,
          H: false,
          C: carry
        );
        result
    }

    // Relative jump.
    // Add n to current address and jump to it.
    fn jr(&mut self) {
        let value = self.fetch8();
        self.registers.pc = self.registers.pc.wrapping_add(value as u16);
    }

    // Decimal adjust register A. This instruction adjusts register A so that the correct representation of Binary
    // Coded Decimal (BCD) is obtained.
    fn daa(&mut self) {
        let mut offset = 0x0_u8;

        // we're moving between base 10 and 16, so we adjust carries with 6
        let carry = self.registers.a > 0x99;
        if self.registers.flag(C) || (!self.registers.flag(N) && carry) {
            offset |= 0x60;
        }

        let half_carry = self.registers.a & 0x0F > 0x09;
        if self.registers.flag(H) || (!self.registers.flag(N) && half_carry) {
            offset |= 0x06;
        }

        self.registers.a = if self.registers.flag(N) {
            self.registers.a.wrapping_sub(offset)
        } else {
            self.registers.a.wrapping_add(offset)
        };

        flags!(self.registers,
            Z: self.registers.a == 0,
            H: false,
            C: offset >= 0x60
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_flags {
      ($cpu:expr, $($flag:ident: $value:expr),*) => {
        $(
          assert_eq!($cpu.registers.flag($flag), $value, concat!("Flag ", stringify!($flag), " is incorrect"));
        )*
      };
    }

    #[test]
    fn test_inc8() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        let result = cpu.inc8(0x00);
        assert_eq!(result, 0x01);
        assert_flags!(cpu,
          Z: false,
          N: false,
          H: false
        );
    }

    #[test]
    fn test_inc8_carry() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        let result = cpu.inc8(0x0F);
        assert_eq!(result, 0x10);
        assert_flags!(cpu,
          Z: false,
          N: false,
          H: true
        );
    }

    #[test]
    fn test_inc8_wrap() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        let result = cpu.inc8(0xFF);
        assert_eq!(result, 0x00);
        assert_flags!(cpu,
          Z: true,
          N: false,
          H: true
        );
    }

    #[test]
    fn test_dec8() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        let result = cpu.dec8(0x02);
        assert_eq!(result, 0x01);
        assert_flags!(cpu,
          Z: false,
          N: true,
          H: false
        );
    }

    #[test]
    fn test_dec8_carry() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        let result = cpu.dec8(0x10);
        assert_eq!(result, 0x0F);
        assert_flags!(cpu,
          Z: false,
          N: true,
          H: true
        );
    }

    #[test]
    fn test_dec8_zero() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        let result = cpu.dec8(0x01);
        assert_eq!(result, 0x00);
        assert_flags!(cpu,
          Z: true,
          N: true,
          H: false
        );
    }

    #[test]
    fn test_dec8_wrap() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        let result = cpu.dec8(0x00);
        assert_eq!(result, 0xFF);
        assert_flags!(cpu,
          Z: false,
          N: true,
          H: true
        );
    }

    #[test]
    fn test_add8() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        cpu.registers.a = 0x01;
        cpu.add8(0x01);
        assert_eq!(cpu.registers.a, 0x02);
        assert_flags!(cpu,
          Z: false,
          N: false,
          H: false,
          C: false
        );
    }

    #[test]
    fn test_add8_half_carry() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        cpu.registers.a = 0x0F;
        cpu.add8(0x01);
        assert_eq!(cpu.registers.a, 0x10);
        assert_flags!(cpu,
          Z: false,
          N: false,
          H: true,
          C: false
        );
    }

    #[test]
    fn test_add8_carry() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        cpu.registers.a = 0xFF;
        cpu.add8(0x01);
        assert_eq!(cpu.registers.a, 0x00);
        assert_flags!(cpu,
          Z: true,
          N: false,
          H: true,
          C: true
        );
    }

    #[test]
    fn test_add8_wrap() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        cpu.registers.a = 0xFF;
        cpu.add8(0x01);
        assert_eq!(cpu.registers.a, 0x00);
        assert_flags!(cpu,
          Z: true,
          N: false,
          H: true,
          C: true
        );
    }

    #[test]
    fn test_adc8() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        cpu.registers.a = 0x01;
        flags!(cpu.registers, C: true);
        cpu.adc8(0x01);
        assert_eq!(cpu.registers.a, 0x03);
        assert_flags!(cpu,
          Z: false,
          N: false,
          H: false,
          C: false
        );
    }

    #[test]
    fn test_sub8() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        cpu.registers.a = 0x01;
        cpu.sub8(0x01);
        assert_eq!(cpu.registers.a, 0x00);
        assert_flags!(cpu,
          Z: true,
          N: true,
          H: false,
          C: false
        );
    }

    #[test]
    fn test_sub8_half_borrow() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        cpu.registers.a = 0x10;
        cpu.sub8(0x01);
        assert_eq!(cpu.registers.a, 0x0F);
        assert_flags!(cpu,
          Z: false,
          N: true,
          H: true,
          C: false
        );
    }

    #[test]
    fn test_sub8_borrow() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        cpu.registers.a = 0x00;
        cpu.sub8(0x01);
        assert_eq!(cpu.registers.a, 0xFF);
        assert_flags!(cpu,
          Z: false,
          N: true,
          H: true,
          C: true
        );
    }

    #[test]
    fn test_sbc8() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        cpu.registers.a = 0x02;
        flags!(cpu.registers, C: true);
        cpu.sbc8(0x01);
        assert_eq!(cpu.registers.a, 0x00);
        assert_flags!(cpu,
          Z: true,
          N: true,
          H: false,
          C: false
        );
    }

    #[test]
    fn test_and8() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        cpu.registers.a = 0x0F;
        cpu.and8(0x0A);
        assert_eq!(cpu.registers.a, 0x0A);
        assert_flags!(cpu,
          Z: false,
          N: false,
          H: true,
          C: false
        );
    }

    #[test]
    fn test_and8_zero() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        cpu.registers.a = 0x0F;
        cpu.and8(0xF0);
        assert_eq!(cpu.registers.a, 0x00);
        assert_flags!(cpu,
          Z: true,
          N: false,
          H: true,
          C: false
        );
    }

    #[test]
    fn test_xor8() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        cpu.registers.a = 0xF0;
        cpu.xor8(0x0F);
        assert_eq!(cpu.registers.a, 0xFF);
        assert_flags!(cpu,
          Z: false,
          N: false,
          H: false,
          C: false
        );
    }

    #[test]
    fn test_xor8_zero() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        cpu.registers.a = 0xF0;
        cpu.xor8(0xF0);
        assert_eq!(cpu.registers.a, 0x00);
        assert_flags!(cpu,
          Z: true,
          N: false,
          H: false,
          C: false
        );
    }

    #[test]
    fn test_or8() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        cpu.registers.a = 0xFF;
        cpu.or8(0xF0);
        assert_eq!(cpu.registers.a, 0xFF);
        assert_flags!(cpu,
          Z: false,
          N: false,
          H: false,
          C: false
        );
    }

    #[test]
    fn test_or8_zero() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        cpu.registers.a = 0x00;
        cpu.or8(0x00);
        assert_eq!(cpu.registers.a, 0x00);
        assert_flags!(cpu,
          Z: true,
          N: false,
          H: false,
          C: false
        );
    }

    #[test]
    fn test_cp8() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        cpu.registers.a = 0x01;
        cpu.cp8(0x01);
        assert_eq!(cpu.registers.a, 0x01);
        assert_flags!(cpu,
          Z: true,
          N: true,
          H: false,
          C: false
        );
    }

    #[test]
    fn test_cp8_half_borrow() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        cpu.registers.a = 0x10;
        cpu.cp8(0x01);
        assert_eq!(cpu.registers.a, 0x10);
        assert_flags!(cpu,
          Z: false,
          N: true,
          H: true,
          C: false
        );
    }

    #[test]
    fn test_cp8_borrow() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        cpu.registers.a = 0x00;
        cpu.cp8(0x01);
        assert_eq!(cpu.registers.a, 0x00);
        assert_flags!(cpu,
          Z: false,
          N: true,
          H: true,
          C: true
        );
    }

    #[test]
    fn test_add16() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        cpu.registers.set_hl(0x1234);
        cpu.add16(0x5678);
        assert_eq!(cpu.registers.hl(), 0x68AC);
        assert_flags!(cpu,
          N: false,
          H: false,
          C: false
        );
    }

    #[test]
    fn test_add16_half_carry() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        cpu.registers.set_hl(0x0FFF);
        cpu.add16(0x0001);
        assert_eq!(cpu.registers.hl(), 0x1000);
        assert_flags!(cpu,
          N: false,
          H: true,
          C: false
        );
    }

    #[test]
    fn test_add16_full_carry() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        cpu.registers.set_hl(0xFFFF);
        cpu.add16(0x0001);
        assert_eq!(cpu.registers.hl(), 0x0000);
        assert_flags!(cpu,
          N: false,
          H: true,
          C: true
        );
    }

    #[test]
    fn test_push() {
        // TODO(robherley): fake mmu
    }

    #[test]
    fn test_pop() {
        // TODO(robherley): fake mmu
    }

    #[test]
    fn test_rlc() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        let result = cpu.rlc(0x85);
        assert_eq!(result, 0x0B);
        assert_flags!(cpu,
          Z: false,
          N: false,
          H: false,
          C: true
        );
    }

    #[test]
    fn test_rlc_zero() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        let result = cpu.rlc(0x00);
        assert_eq!(result, 0x00);
        assert_flags!(cpu,
          Z: true,
          N: false,
          H: false,
          C: false
        );
    }

    #[test]
    fn test_rl() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        flags!(cpu.registers, C: true);
        let result = cpu.rl(0x85);
        assert_eq!(result, 0x0B);
        assert_flags!(cpu,
          Z: false,
          N: false,
          H: false,
          C: true
        );
    }

    #[test]
    fn test_rl_zero() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        let result = cpu.rl(0x00);
        assert_eq!(result, 0x00);
        assert_flags!(cpu,
          Z: true,
          N: false,
          H: false,
          C: false
        );
    }

    #[test]
    fn test_rrc() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        let result = cpu.rrc(0x85);
        assert_eq!(result, 0xC2);
        assert_flags!(cpu,
          Z: false,
          N: false,
          H: false,
          C: true
        );
    }

    #[test]
    fn test_rrc_zero() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        let result = cpu.rrc(0x00);
        assert_eq!(result, 0x00);
        assert_flags!(cpu,
          Z: true,
          N: false,
          H: false,
          C: false
        );
    }

    #[test]
    fn test_rr() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        flags!(cpu.registers, C: true);
        let result = cpu.rr(0x85);
        assert_eq!(result, 0xC2);
        assert_flags!(cpu,
          Z: false,
          N: false,
          H: false,
          C: true
        );
    }

    #[test]
    fn test_rr_zero() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        let result = cpu.rr(0x00);
        assert_eq!(result, 0x00);
        assert_flags!(cpu,
          Z: true,
          N: false,
          H: false,
          C: false
        );
    }

    #[test]
    fn test_jr() {
        // TODO(robherley): fake mmu
    }

    #[test]
    fn test_daa_none() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        cpu.registers.a = 0x10;
        flags!(cpu.registers,
          N: false,
          H: false,
          C: false
        );
        cpu.daa();
        assert_eq!(cpu.registers.a, 0x10);
        assert_flags!(cpu,
          Z: false,
          N: false,
          H: false,
          C: false
        );
    }

    #[test]
    fn test_daa_add_half_carry() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        cpu.registers.a = 0x0A;
        flags!(cpu.registers,
          N: false,
          H: true,
          C: false
        );
        cpu.daa();
        assert_eq!(cpu.registers.a, 0x10);
        assert_flags!(cpu,
          Z: false,
          N: false,
          H: false,
          C: false
        );
    }

    #[test]
    fn test_daa_add_carry() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        cpu.registers.a = 0x9B;
        flags!(cpu.registers,
          N: false,
          H: false,
          C: true
        );
        cpu.daa();
        assert_eq!(cpu.registers.a, 0x01);
        assert_flags!(cpu,
          Z: false,
          N: false,
          H: false,
          C: true
        );
    }

    #[test]
    fn test_daa_sub_half_carry() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        cpu.registers.a = 0x0C;
        flags!(cpu.registers,
          N: true,
          H: true,
          C: false
        );
        cpu.daa();
        assert_eq!(cpu.registers.a, 0x06);
        assert_flags!(cpu,
          Z: false,
          N: true,
          H: false,
          C: false
        );
    }

    #[test]
    fn test_daa_sub_carry() {
        let mut cpu = CPU::new(Model::DMG, Cartridge::default());
        cpu.registers.a = 0xC4;
        flags!(cpu.registers,
          N: true,
          H: false,
          C: true
        );
        cpu.daa();
        assert_eq!(cpu.registers.a, 0x64);
        assert_flags!(cpu,
          Z: false,
          N: true,
          H: false,
          C: true
        );
    }
}
