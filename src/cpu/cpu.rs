use super::registers::{Flags::*, Registers};
use crate::cartridge::Cartridge;
use crate::flags;
use crate::mmu::Memory;

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("cpu not supported: {0:?}")]
    CPUNotSupported(Model),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Model {
    /// Original Game Boy
    DMG,
    /// Game Boy Pocket
    MGB,
    /// Game Boy Color
    CGB,
    /// Super Game Boy
    SGB,
    /// Super Game Boy 2
    SGB2,
    /// Gameboy Advance
    AGB,
}

pub struct CPU {
    registers: Registers,
    mmu: Memory,
    halted: bool,
}

impl CPU {
    pub fn new(model: Model, cartridge: Cartridge) -> Result<CPU, Error> {
        Ok(CPU {
            registers: Registers::new(model, &cartridge)?,
            mmu: Memory::new(cartridge),
            halted: false,
        })
    }

    pub fn boot(&mut self) {
        loop {
            self.step();
        }
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

    fn step(&mut self) -> usize {
        self.exec()
    }

    /// Executes the next instruction and returns the number of t-cycles (system clock ticks) it took.
    /// https://gbdev.io/gb-opcodes/optables/
    fn exec(&mut self) -> usize {
        match self.fetch8() {
            // NOP  | ----
            0x00 => 4,
            // LD (BC), (n16) | ----
            0x01 => {
                let value = self.fetch16();
                self.registers.set_bc(value);
                12
            }
            // LD BC, (A) | ----
            0x02 => unimplemented!(),
            // INC (BC) | ----
            0x03 => unimplemented!(),
            // INC (B) | Z0H-
            0x04 => {
                self.registers.b = self.inc8(self.registers.b);
                4
            }
            // DEC (B) | Z1H-
            0x05 => {
                self.registers.b = self.dec8(self.registers.b);
                4
            }
            // LD (B), (n8) | ----
            0x06 => unimplemented!(),
            // RLCA  | 000C
            0x07 => unimplemented!(),
            // LD a16, (SP) | ----
            0x08 => unimplemented!(),
            // ADD (HL), (BC) | -0HC
            0x09 => unimplemented!(),
            // LD (A), BC | ----
            0x0A => unimplemented!(),
            // DEC (BC) | ----
            0x0B => unimplemented!(),
            // INC (C) | Z0H-
            0x0C => {
                self.registers.c = self.inc8(self.registers.c);
                4
            }
            // DEC (C) | Z1H-
            0x0D => {
                self.registers.c = self.dec8(self.registers.c);
                4
            }
            // LD (C), (n8) | ----
            0x0E => unimplemented!(),
            // RRCA  | 000C
            0x0F => unimplemented!(),
            // STOP (n8) | ----
            0x10 => unimplemented!(),
            // LD (DE), (n16) | ----
            0x11 => {
                let value = self.fetch16();
                self.registers.set_de(value);
                12
            }
            // LD DE, (A) | ----
            0x12 => unimplemented!(),
            // INC (DE) | ----
            0x13 => unimplemented!(),
            // INC (D) | Z0H-
            0x14 => {
                self.registers.c = self.inc8(self.registers.d);
                4
            }
            // DEC (D) | Z1H-
            0x15 => {
                self.registers.d = self.dec8(self.registers.d);
                4
            }
            // LD (D), (n8) | ----
            0x16 => unimplemented!(),
            // RLA  | 000C
            0x17 => unimplemented!(),
            // JR (e8) | ----
            0x18 => unimplemented!(),
            // ADD (HL), (DE) | -0HC
            0x19 => unimplemented!(),
            // LD (A), DE | ----
            0x1A => unimplemented!(),
            // DEC (DE) | ----
            0x1B => unimplemented!(),
            // INC (E) | Z0H-
            0x1C => {
                self.registers.e = self.inc8(self.registers.e);
                4
            }
            // DEC (E) | Z1H-
            0x1D => {
                self.registers.e = self.dec8(self.registers.e);
                4
            }
            // LD (E), (n8) | ----
            0x1E => unimplemented!(),
            // RRA  | 000C
            0x1F => unimplemented!(),
            // JR (NZ), (e8) | ----
            0x20 => unimplemented!(),
            // LD (HL), (n16) | ----
            0x21 => unimplemented!(),
            // LD HL, (A) | ----
            0x22 => unimplemented!(),
            // INC (HL) | ----
            0x23 => {
                let value = self.registers.hl().wrapping_add(1);
                self.registers.set_hl(value);
                8
            }
            // INC (H) | Z0H-
            0x24 => {
                self.registers.h = self.inc8(self.registers.h);
                4
            }
            // DEC (H) | Z1H-
            0x25 => {
                self.registers.h = self.dec8(self.registers.h);
                4
            }
            // LD (H), (n8) | ----
            0x26 => unimplemented!(),
            // DAA  | Z-0C
            0x27 => unimplemented!(),
            // JR (Z), (e8) | ----
            0x28 => unimplemented!(),
            // ADD (HL), (HL) | -0HC
            0x29 => unimplemented!(),
            // LD (A), HL | ----
            0x2A => unimplemented!(),
            // DEC (HL) | ----
            0x2B => unimplemented!(),
            // INC (L) | Z0H-
            0x2C => {
                self.registers.l = self.inc8(self.registers.l);
                4
            }
            // DEC (L) | Z1H-
            0x2D => {
                self.registers.l = self.dec8(self.registers.l);
                4
            }
            // LD (L), (n8) | ----
            0x2E => unimplemented!(),
            // CPL  | -11-
            0x2F => unimplemented!(),
            // JR (NC), (e8) | ----
            0x30 => unimplemented!(),
            // LD (SP), (n16) | ----
            0x31 => {
                self.registers.sp = self.fetch16();
                12
            }
            // LD HL, (A) | ----
            0x32 => unimplemented!(),
            // INC (SP) | ----
            0x33 => unimplemented!(),
            // INC HL | Z0H-
            0x34 => unimplemented!(),
            // DEC HL | Z1H-
            0x35 => unimplemented!(),
            // LD HL, (n8) | ----
            0x36 => unimplemented!(),
            // SCF  | -001
            0x37 => unimplemented!(),
            // JR (C), (e8) | ----
            0x38 => unimplemented!(),
            // ADD (HL), (SP) | -0HC
            0x39 => unimplemented!(),
            // LD (A), HL | ----
            0x3A => unimplemented!(),
            // DEC (SP) | ----
            0x3B => unimplemented!(),
            // INC (A) | Z0H-
            0x3C => {
                self.registers.a = self.inc8(self.registers.a);
                4
            }
            // DEC (A) | Z1H-
            0x3D => {
                self.registers.a = self.dec8(self.registers.a);
                4
            }
            // LD (A), (n8) | ----
            0x3E => unimplemented!(),
            // CCF  | -00C
            0x3F => unimplemented!(),
            // LD (B), (B) | ----
            0x40 => unimplemented!(),
            // LD (B), (C) | ----
            0x41 => unimplemented!(),
            // LD (B), (D) | ----
            0x42 => unimplemented!(),
            // LD (B), (E) | ----
            0x43 => unimplemented!(),
            // LD (B), (H) | ----
            0x44 => unimplemented!(),
            // LD (B), (L) | ----
            0x45 => unimplemented!(),
            // LD (B), HL | ----
            0x46 => unimplemented!(),
            // LD (B), (A) | ----
            0x47 => unimplemented!(),
            // LD (C), (B) | ----
            0x48 => unimplemented!(),
            // LD (C), (C) | ----
            0x49 => unimplemented!(),
            // LD (C), (D) | ----
            0x4A => unimplemented!(),
            // LD (C), (E) | ----
            0x4B => unimplemented!(),
            // LD (C), (H) | ----
            0x4C => unimplemented!(),
            // LD (C), (L) | ----
            0x4D => unimplemented!(),
            // LD (C), HL | ----
            0x4E => unimplemented!(),
            // LD (C), (A) | ----
            0x4F => unimplemented!(),
            // LD (D), (B) | ----
            0x50 => unimplemented!(),
            // LD (D), (C) | ----
            0x51 => unimplemented!(),
            // LD (D), (D) | ----
            0x52 => unimplemented!(),
            // LD (D), (E) | ----
            0x53 => unimplemented!(),
            // LD (D), (H) | ----
            0x54 => unimplemented!(),
            // LD (D), (L) | ----
            0x55 => unimplemented!(),
            // LD (D), HL | ----
            0x56 => unimplemented!(),
            // LD (D), (A) | ----
            0x57 => unimplemented!(),
            // LD (E), (B) | ----
            0x58 => unimplemented!(),
            // LD (E), (C) | ----
            0x59 => unimplemented!(),
            // LD (E), (D) | ----
            0x5A => unimplemented!(),
            // LD (E), (E) | ----
            0x5B => unimplemented!(),
            // LD (E), (H) | ----
            0x5C => unimplemented!(),
            // LD (E), (L) | ----
            0x5D => unimplemented!(),
            // LD (E), HL | ----
            0x5E => unimplemented!(),
            // LD (E), (A) | ----
            0x5F => unimplemented!(),
            // LD (H), (B) | ----
            0x60 => unimplemented!(),
            // LD (H), (C) | ----
            0x61 => unimplemented!(),
            // LD (H), (D) | ----
            0x62 => unimplemented!(),
            // LD (H), (E) | ----
            0x63 => unimplemented!(),
            // LD (H), (H) | ----
            0x64 => unimplemented!(),
            // LD (H), (L) | ----
            0x65 => unimplemented!(),
            // LD (H), HL | ----
            0x66 => unimplemented!(),
            // LD (H), (A) | ----
            0x67 => unimplemented!(),
            // LD (L), (B) | ----
            0x68 => unimplemented!(),
            // LD (L), (C) | ----
            0x69 => unimplemented!(),
            // LD (L), (D) | ----
            0x6A => unimplemented!(),
            // LD (L), (E) | ----
            0x6B => unimplemented!(),
            // LD (L), (H) | ----
            0x6C => unimplemented!(),
            // LD (L), (L) | ----
            0x6D => unimplemented!(),
            // LD (L), HL | ----
            0x6E => unimplemented!(),
            // LD (L), (A) | ----
            0x6F => unimplemented!(),
            // LD HL, (B) | ----
            0x70 => unimplemented!(),
            // LD HL, (C) | ----
            0x71 => unimplemented!(),
            // LD HL, (D) | ----
            0x72 => unimplemented!(),
            // LD HL, (E) | ----
            0x73 => unimplemented!(),
            // LD HL, (H) | ----
            0x74 => unimplemented!(),
            // LD HL, (L) | ----
            0x75 => unimplemented!(),
            // HALT  | ----
            0x76 => {
                self.halted = true;
                1
            }
            // LD HL, (A) | ----
            0x77 => unimplemented!(),
            // LD (A), (B) | ----
            0x78 => unimplemented!(),
            // LD (A), (C) | ----
            0x79 => unimplemented!(),
            // LD (A), (D) | ----
            0x7A => unimplemented!(),
            // LD (A), (E) | ----
            0x7B => unimplemented!(),
            // LD (A), (H) | ----
            0x7C => unimplemented!(),
            // LD (A), (L) | ----
            0x7D => unimplemented!(),
            // LD (A), HL | ----
            0x7E => unimplemented!(),
            // LD (A), (A) | ----
            0x7F => unimplemented!(),
            // ADD (A), (B) | Z0HC
            0x80 => {
                self.add8(self.registers.b);
                4
            }
            // ADD (A), (C) | Z0HC
            0x81 => {
                self.add8(self.registers.c);
                4
            }
            // ADD (A), (D) | Z0HC
            0x82 => {
                self.add8(self.registers.d);
                4
            }
            // ADD (A), (E) | Z0HC
            0x83 => {
                self.add8(self.registers.e);
                4
            }
            // ADD (A), (H) | Z0HC
            0x84 => {
                self.add8(self.registers.h);
                4
            }
            // ADD (A), (L) | Z0HC
            0x85 => {
                self.add8(self.registers.l);
                4
            }
            // ADD (A), HL | Z0HC
            0x86 => {
                let value = self.mmu.read8(self.registers.hl());
                self.add8(value);
                8
            }
            // ADD (A), (A) | Z0HC
            0x87 => {
                self.add8(self.registers.a);
                4
            }
            // ADC (A), (B) | Z0HC
            0x88 => {
                self.adc8(self.registers.b);
                4
            }
            // ADC (A), (C) | Z0HC
            0x89 => {
                self.adc8(self.registers.c);
                4
            }
            // ADC (A), (D) | Z0HC
            0x8A => {
                self.adc8(self.registers.d);
                4
            }
            // ADC (A), (E) | Z0HC
            0x8B => {
                self.adc8(self.registers.e);
                4
            }
            // ADC (A), (H) | Z0HC
            0x8C => {
                self.adc8(self.registers.h);
                4
            }
            // ADC (A), (L) | Z0HC
            0x8D => {
                self.adc8(self.registers.l);
                4
            }
            // ADC (A), HL | Z0HC
            0x8E => {
                let value = self.mmu.read8(self.registers.hl());
                self.adc8(value);
                8
            }
            // ADC (A), (A) | Z0HC
            0x8F => {
                self.adc8(self.registers.a);
                4
            }
            // SUB (A), (B) | Z1HC
            0x90 => {
                self.sub8(self.registers.b);
                4
            }
            // SUB (A), (C) | Z1HC
            0x91 => {
                self.sub8(self.registers.c);
                4
            }
            // SUB (A), (D) | Z1HC
            0x92 => {
                self.sub8(self.registers.d);
                4
            }
            // SUB (A), (E) | Z1HC
            0x93 => {
                self.sub8(self.registers.e);
                4
            }
            // SUB (A), (H) | Z1HC
            0x94 => {
                self.sub8(self.registers.h);
                4
            }
            // SUB (A), (L) | Z1HC
            0x95 => {
                self.sub8(self.registers.l);
                4
            }
            // SUB (A), HL | Z1HC
            0x96 => {
                let value = self.mmu.read8(self.registers.hl());
                self.sub8(value);
                8
            }
            // SUB (A), (A) | 1100
            0x97 => {
                self.sub8(self.registers.a);
                4
            }
            // SBC (A), (B) | Z1HC
            0x98 => {
                self.sbc8(self.registers.b);
                4
            }
            // SBC (A), (C) | Z1HC
            0x99 => {
                self.sbc8(self.registers.c);
                4
            }
            // SBC (A), (D) | Z1HC
            0x9A => {
                self.sbc8(self.registers.d);
                4
            }
            // SBC (A), (E) | Z1HC
            0x9B => {
                self.sbc8(self.registers.e);
                4
            }
            // SBC (A), (H) | Z1HC
            0x9C => {
                self.sbc8(self.registers.h);
                4
            }
            // SBC (A), (L) | Z1HC
            0x9D => {
                self.sbc8(self.registers.l);
                4
            }
            // SBC (A), HL | Z1HC
            0x9E => {
                let value = self.mmu.read8(self.registers.hl());
                self.sbc8(value);
                8
            }
            // SBC (A), (A) | Z1H-
            0x9F => {
                self.sbc8(self.registers.a);
                4
            }
            // AND (A), (B) | Z010
            0xA0 => {
                self.and8(self.registers.b);
                4
            }
            // AND (A), (C) | Z010
            0xA1 => {
                self.and8(self.registers.c);
                4
            }
            // AND (A), (D) | Z010
            0xA2 => {
                self.and8(self.registers.d);
                4
            }
            // AND (A), (E) | Z010
            0xA3 => {
                self.and8(self.registers.e);
                4
            }
            // AND (A), (H) | Z010
            0xA4 => {
                self.and8(self.registers.h);
                4
            }
            // AND (A), (L) | Z010
            0xA5 => {
                self.and8(self.registers.l);
                4
            }
            // AND (A), HL | Z010
            0xA6 => {
                let value = self.mmu.read8(self.registers.hl());
                self.and8(value);
                8
            }
            // AND (A), (A) | Z010
            0xA7 => {
                self.and8(self.registers.a);
                4
            }
            // XOR (A), (B) | Z000
            0xA8 => {
                self.xor8(self.registers.b);
                4
            }
            // XOR (A), (C) | Z000
            0xA9 => {
                self.xor8(self.registers.c);
                4
            }
            // XOR (A), (D) | Z000
            0xAA => {
                self.xor8(self.registers.d);
                4
            }
            // XOR (A), (E) | Z000
            0xAB => {
                self.xor8(self.registers.e);
                4
            }
            // XOR (A), (H) | Z000
            0xAC => {
                self.xor8(self.registers.h);
                4
            }
            // XOR (A), (L) | Z000
            0xAD => {
                self.xor8(self.registers.l);
                4
            }
            // XOR (A), HL | Z000
            0xAE => {
                let value = self.mmu.read8(self.registers.hl());
                self.xor8(value);
                8
            }
            // XOR (A), (A) | 1000
            0xAF => {
                self.xor8(self.registers.a);
                4
            }
            // OR (A), (B) | Z000
            0xB0 => {
                self.or8(self.registers.b);
                4
            }
            // OR (A), (C) | Z000
            0xB1 => {
                self.or8(self.registers.c);
                4
            }
            // OR (A), (D) | Z000
            0xB2 => {
                self.or8(self.registers.d);
                4
            }
            // OR (A), (E) | Z000
            0xB3 => {
                self.or8(self.registers.e);
                4
            }
            // OR (A), (H) | Z000
            0xB4 => {
                self.or8(self.registers.h);
                4
            }
            // OR (A), (L) | Z000
            0xB5 => {
                self.or8(self.registers.l);
                4
            }
            // OR (A), HL | Z000
            0xB6 => {
                let value = self.mmu.read8(self.registers.hl());
                self.or8(value);
                8
            }
            // OR (A), (A) | Z000
            0xB7 => {
                self.or8(self.registers.a);
                4
            }
            // CP (A), (B) | Z1HC
            0xB8 => unimplemented!(),
            // CP (A), (C) | Z1HC
            0xB9 => unimplemented!(),
            // CP (A), (D) | Z1HC
            0xBA => unimplemented!(),
            // CP (A), (E) | Z1HC
            0xBB => unimplemented!(),
            // CP (A), (H) | Z1HC
            0xBC => unimplemented!(),
            // CP (A), (L) | Z1HC
            0xBD => unimplemented!(),
            // CP (A), HL | Z1HC
            0xBE => unimplemented!(),
            // CP (A), (A) | 1100
            0xBF => unimplemented!(),
            // RET (NZ) | ----
            0xC0 => unimplemented!(),
            // POP (BC) | ----
            0xC1 => unimplemented!(),
            // JP (NZ), (a16) | ----
            0xC2 => unimplemented!(),
            // JP (a16) | ----
            0xC3 => unimplemented!(),
            // CALL (NZ), (a16) | ----
            0xC4 => unimplemented!(),
            // PUSH (BC) | ----
            0xC5 => unimplemented!(),
            // ADD (A), (n8) | Z0HC
            0xC6 => {
                let value = self.fetch8();
                self.add8(value);
                8
            }
            // RST ($00) | ----
            0xC7 => unimplemented!(),
            // RET (Z) | ----
            0xC8 => unimplemented!(),
            // RET  | ----
            0xC9 => unimplemented!(),
            // JP (Z), (a16) | ----
            0xCA => unimplemented!(),
            // PREFIX  | ----
            0xCB => self.exec_cb(),
            // CALL (Z), (a16) | ----
            0xCC => unimplemented!(),
            // CALL (a16) | ----
            0xCD => unimplemented!(),
            // ADC (A), (n8) | Z0HC
            0xCE => {
                let value = self.fetch8();
                self.adc8(value);
                8
            }
            // RST ($08) | ----
            0xCF => unimplemented!(),
            // RET (NC) | ----
            0xD0 => unimplemented!(),
            // POP (DE) | ----
            0xD1 => unimplemented!(),
            // JP (NC), (a16) | ----
            0xD2 => unimplemented!(),
            // ILLEGAL(0xD3) | ----
            0xD3 => unimplemented!(),
            // CALL (NC), (a16) | ----
            0xD4 => unimplemented!(),
            // PUSH (DE) | ----
            0xD5 => unimplemented!(),
            // SUB (A), (n8) | Z1HC
            0xD6 => {
                let value = self.fetch8();
                self.sub8(value);
                8
            }
            // RST ($10) | ----
            0xD7 => unimplemented!(),
            // RET (C) | ----
            0xD8 => unimplemented!(),
            // RETI  | ----
            0xD9 => unimplemented!(),
            // JP (C), (a16) | ----
            0xDA => unimplemented!(),
            // ILLEGAL(0xDB) | ----
            0xDB => unimplemented!(),
            // CALL (C), (a16) | ----
            0xDC => unimplemented!(),
            // ILLEGAL(0xDD) | ----
            0xDD => unimplemented!(),
            // SBC (A), (n8) | Z1HC
            0xDE => {
                let value = self.fetch8();
                self.sbc8(value);
                8
            }
            // RST ($18) | ----
            0xDF => unimplemented!(),
            // LDH a8, (A) | ----
            0xE0 => unimplemented!(),
            // POP (HL) | ----
            0xE1 => unimplemented!(),
            // LD C, (A) | ----
            0xE2 => unimplemented!(),
            // ILLEGAL(0xE3) | ----
            0xE3 => unimplemented!(),
            // ILLEGAL(0xE4) | ----
            0xE4 => unimplemented!(),
            // PUSH (HL) | ----
            0xE5 => unimplemented!(),
            // AND (A), (n8) | Z010
            0xE6 => {
                let value = self.fetch8();
                self.and8(value);
                8
            }
            // RST ($20) | ----
            0xE7 => unimplemented!(),
            // ADD (SP), (e8) | 00HC
            0xE8 => unimplemented!(),
            // JP (HL) | ----
            0xE9 => unimplemented!(),
            // LD a16, (A) | ----
            0xEA => unimplemented!(),
            // ILLEGAL(0xEB) | ----
            0xEB => unimplemented!(),
            // ILLEGAL(0xEC) | ----
            0xEC => unimplemented!(),
            // ILLEGAL(0xED) | ----
            0xED => unimplemented!(),
            // XOR (A), (n8) | Z000
            0xEE => {
                let value = self.fetch8();
                self.xor8(value);
                8
            }
            // RST ($28) | ----
            0xEF => unimplemented!(),
            // LDH (A), a8 | ----
            0xF0 => unimplemented!(),
            // POP (AF) | ZNHC
            0xF1 => unimplemented!(),
            // LD (A), C | ----
            0xF2 => unimplemented!(),
            // DI  | ----
            0xF3 => unimplemented!(),
            // ILLEGAL(0xF4) | ----
            0xF4 => unimplemented!(),
            // PUSH (AF) | ----
            0xF5 => unimplemented!(),
            // OR (A), (n8) | Z000
            0xF6 => unimplemented!(),
            // RST ($30) | ----
            0xF7 => unimplemented!(),
            // LD (HL), (SP), (e8) | 00HC
            0xF8 => unimplemented!(),
            // LD (SP), (HL) | ----
            0xF9 => unimplemented!(),
            // LD (A), a16 | ----
            0xFA => unimplemented!(),
            // EI  | ----
            0xFB => unimplemented!(),
            // ILLEGAL(0xFC) | ----
            0xFC => unimplemented!(),
            // ILLEGAL(0xFD) | ----
            0xFD => unimplemented!(),
            // CP (A), (n8) | Z1HC
            0xFE => unimplemented!(),
            // RST ($38) | ----
            0xFF => unimplemented!(),
        }
    }

    fn exec_cb(&mut self) -> usize {
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

    fn build_cpu() -> CPU {
        CPU::new(Model::DMG, Cartridge::default()).unwrap()
    }

    #[test]
    fn test_inc8() {
        let mut cpu = build_cpu();
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
        let mut cpu = build_cpu();
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
        let mut cpu = build_cpu();
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
        let mut cpu = build_cpu();
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
        let mut cpu = build_cpu();
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
        let mut cpu = build_cpu();
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
        let mut cpu = build_cpu();
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
        let mut cpu = build_cpu();
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
        let mut cpu = build_cpu();
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
        let mut cpu = build_cpu();
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
        let mut cpu = build_cpu();
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
        let mut cpu = build_cpu();
        cpu.registers.a = 0x01;
        cpu.registers.set_flag(C, true);
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
        let mut cpu = build_cpu();
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
        let mut cpu = build_cpu();
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
        let mut cpu = build_cpu();
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
        let mut cpu = build_cpu();
        cpu.registers.a = 0x02;
        cpu.registers.set_flag(C, true);
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
        let mut cpu = build_cpu();
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
        let mut cpu = build_cpu();
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
        let mut cpu = build_cpu();
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
        let mut cpu = build_cpu();
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
        let mut cpu = build_cpu();
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
        let mut cpu = build_cpu();
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
}
