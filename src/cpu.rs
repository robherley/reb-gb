use crate::cartridge::Cartridge;
use crate::memory::Memory;
use crate::registers::Registers;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CPUError {
    #[error("cpu not supported: {0:?}")]
    CPUNotSupported(Model),
}

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
    registers: Registers,
    mmu: Memory,
    halted: bool,
}

impl CPU {
    pub fn new(model: Model, cartridge: Cartridge) -> Result<CPU, CPUError> {
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

    #[allow(dead_code)]
    fn fetch16(&mut self) -> u16 {
        let pc = self.registers.pc;
        self.registers.pc += 2;
        self.mmu.read16(pc)
    }

    pub fn step(&mut self) -> usize {
        self.exec()
    }

    // Executes the next instruction and returns the number of m-cycles it took.
    pub fn exec(&mut self) -> usize {
        let op = self.fetch8();
        match op {
            // NOP  | ----
            0x00 => 1,
            // LD (BC), (n16) | ----f
            0x01 => unimplemented!(),
            // LD BC, (A) | ----
            0x02 => unimplemented!(),
            // INC (BC) | ----
            0x03 => unimplemented!(),
            // INC (B) | Z0H-
            0x04 => unimplemented!(),
            // DEC (B) | Z1H-
            0x05 => unimplemented!(),
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
            0x0C => unimplemented!(),
            // DEC (C) | Z1H-
            0x0D => unimplemented!(),
            // LD (C), (n8) | ----
            0x0E => unimplemented!(),
            // RRCA  | 000C
            0x0F => unimplemented!(),
            // STOP (n8) | ----
            0x10 => unimplemented!(),
            // LD (DE), (n16) | ----
            0x11 => unimplemented!(),
            // LD DE, (A) | ----
            0x12 => unimplemented!(),
            // INC (DE) | ----
            0x13 => unimplemented!(),
            // INC (D) | Z0H-
            0x14 => unimplemented!(),
            // DEC (D) | Z1H-
            0x15 => unimplemented!(),
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
            0x1C => unimplemented!(),
            // DEC (E) | Z1H-
            0x1D => unimplemented!(),
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
            0x23 => unimplemented!(),
            // INC (H) | Z0H-
            0x24 => unimplemented!(),
            // DEC (H) | Z1H-
            0x25 => unimplemented!(),
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
            0x2C => unimplemented!(),
            // DEC (L) | Z1H-
            0x2D => unimplemented!(),
            // LD (L), (n8) | ----
            0x2E => unimplemented!(),
            // CPL  | -11-
            0x2F => unimplemented!(),
            // JR (NC), (e8) | ----
            0x30 => unimplemented!(),
            // LD (SP), (n16) | ----
            0x31 => unimplemented!(),
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
            0x3C => unimplemented!(),
            // DEC (A) | Z1H-
            0x3D => unimplemented!(),
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
            0x80 => unimplemented!(),
            // ADD (A), (C) | Z0HC
            0x81 => unimplemented!(),
            // ADD (A), (D) | Z0HC
            0x82 => unimplemented!(),
            // ADD (A), (E) | Z0HC
            0x83 => unimplemented!(),
            // ADD (A), (H) | Z0HC
            0x84 => unimplemented!(),
            // ADD (A), (L) | Z0HC
            0x85 => unimplemented!(),
            // ADD (A), HL | Z0HC
            0x86 => unimplemented!(),
            // ADD (A), (A) | Z0HC
            0x87 => unimplemented!(),
            // ADC (A), (B) | Z0HC
            0x88 => unimplemented!(),
            // ADC (A), (C) | Z0HC
            0x89 => unimplemented!(),
            // ADC (A), (D) | Z0HC
            0x8A => unimplemented!(),
            // ADC (A), (E) | Z0HC
            0x8B => unimplemented!(),
            // ADC (A), (H) | Z0HC
            0x8C => unimplemented!(),
            // ADC (A), (L) | Z0HC
            0x8D => unimplemented!(),
            // ADC (A), HL | Z0HC
            0x8E => unimplemented!(),
            // ADC (A), (A) | Z0HC
            0x8F => unimplemented!(),
            // SUB (A), (B) | Z1HC
            0x90 => unimplemented!(),
            // SUB (A), (C) | Z1HC
            0x91 => unimplemented!(),
            // SUB (A), (D) | Z1HC
            0x92 => unimplemented!(),
            // SUB (A), (E) | Z1HC
            0x93 => unimplemented!(),
            // SUB (A), (H) | Z1HC
            0x94 => unimplemented!(),
            // SUB (A), (L) | Z1HC
            0x95 => unimplemented!(),
            // SUB (A), HL | Z1HC
            0x96 => unimplemented!(),
            // SUB (A), (A) | 1100
            0x97 => unimplemented!(),
            // SBC (A), (B) | Z1HC
            0x98 => unimplemented!(),
            // SBC (A), (C) | Z1HC
            0x99 => unimplemented!(),
            // SBC (A), (D) | Z1HC
            0x9A => unimplemented!(),
            // SBC (A), (E) | Z1HC
            0x9B => unimplemented!(),
            // SBC (A), (H) | Z1HC
            0x9C => unimplemented!(),
            // SBC (A), (L) | Z1HC
            0x9D => unimplemented!(),
            // SBC (A), HL | Z1HC
            0x9E => unimplemented!(),
            // SBC (A), (A) | Z1H-
            0x9F => unimplemented!(),
            // AND (A), (B) | Z010
            0xA0 => unimplemented!(),
            // AND (A), (C) | Z010
            0xA1 => unimplemented!(),
            // AND (A), (D) | Z010
            0xA2 => unimplemented!(),
            // AND (A), (E) | Z010
            0xA3 => unimplemented!(),
            // AND (A), (H) | Z010
            0xA4 => unimplemented!(),
            // AND (A), (L) | Z010
            0xA5 => unimplemented!(),
            // AND (A), HL | Z010
            0xA6 => unimplemented!(),
            // AND (A), (A) | Z010
            0xA7 => unimplemented!(),
            // XOR (A), (B) | Z000
            0xA8 => unimplemented!(),
            // XOR (A), (C) | Z000
            0xA9 => unimplemented!(),
            // XOR (A), (D) | Z000
            0xAA => unimplemented!(),
            // XOR (A), (E) | Z000
            0xAB => unimplemented!(),
            // XOR (A), (H) | Z000
            0xAC => unimplemented!(),
            // XOR (A), (L) | Z000
            0xAD => unimplemented!(),
            // XOR (A), HL | Z000
            0xAE => unimplemented!(),
            // XOR (A), (A) | 1000
            0xAF => unimplemented!(),
            // OR (A), (B) | Z000
            0xB0 => unimplemented!(),
            // OR (A), (C) | Z000
            0xB1 => unimplemented!(),
            // OR (A), (D) | Z000
            0xB2 => unimplemented!(),
            // OR (A), (E) | Z000
            0xB3 => unimplemented!(),
            // OR (A), (H) | Z000
            0xB4 => unimplemented!(),
            // OR (A), (L) | Z000
            0xB5 => unimplemented!(),
            // OR (A), HL | Z000
            0xB6 => unimplemented!(),
            // OR (A), (A) | Z000
            0xB7 => unimplemented!(),
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
            0xC6 => unimplemented!(),
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
            0xCE => unimplemented!(),
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
            0xD6 => unimplemented!(),
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
            0xDE => unimplemented!(),
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
            0xE6 => unimplemented!(),
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
            0xEE => unimplemented!(),
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

    pub fn exec_cb(&mut self) -> usize {
        let op = self.fetch8();
        match op {
            // NOP  | ----
            0x00 => unimplemented!(),
            // LD (BC), (n16) | ----
            0x01 => unimplemented!(),
            // LD BC, (A) | ----
            0x02 => unimplemented!(),
            // INC (BC) | ----
            0x03 => unimplemented!(),
            // INC (B) | Z0H-
            0x04 => unimplemented!(),
            // DEC (B) | Z1H-
            0x05 => unimplemented!(),
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
            0x0C => unimplemented!(),
            // DEC (C) | Z1H-
            0x0D => unimplemented!(),
            // LD (C), (n8) | ----
            0x0E => unimplemented!(),
            // RRCA  | 000C
            0x0F => unimplemented!(),
            // STOP (n8) | ----
            0x10 => unimplemented!(),
            // LD (DE), (n16) | ----
            0x11 => unimplemented!(),
            // LD DE, (A) | ----
            0x12 => unimplemented!(),
            // INC (DE) | ----
            0x13 => unimplemented!(),
            // INC (D) | Z0H-
            0x14 => unimplemented!(),
            // DEC (D) | Z1H-
            0x15 => unimplemented!(),
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
            0x1C => unimplemented!(),
            // DEC (E) | Z1H-
            0x1D => unimplemented!(),
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
            0x23 => unimplemented!(),
            // INC (H) | Z0H-
            0x24 => unimplemented!(),
            // DEC (H) | Z1H-
            0x25 => unimplemented!(),
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
            0x2C => unimplemented!(),
            // DEC (L) | Z1H-
            0x2D => unimplemented!(),
            // LD (L), (n8) | ----
            0x2E => unimplemented!(),
            // CPL  | -11-
            0x2F => unimplemented!(),
            // JR (NC), (e8) | ----
            0x30 => unimplemented!(),
            // LD (SP), (n16) | ----
            0x31 => unimplemented!(),
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
            0x3C => unimplemented!(),
            // DEC (A) | Z1H-
            0x3D => unimplemented!(),
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
            0x76 => unimplemented!(),
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
            0x80 => unimplemented!(),
            // ADD (A), (C) | Z0HC
            0x81 => unimplemented!(),
            // ADD (A), (D) | Z0HC
            0x82 => unimplemented!(),
            // ADD (A), (E) | Z0HC
            0x83 => unimplemented!(),
            // ADD (A), (H) | Z0HC
            0x84 => unimplemented!(),
            // ADD (A), (L) | Z0HC
            0x85 => unimplemented!(),
            // ADD (A), HL | Z0HC
            0x86 => unimplemented!(),
            // ADD (A), (A) | Z0HC
            0x87 => unimplemented!(),
            // ADC (A), (B) | Z0HC
            0x88 => unimplemented!(),
            // ADC (A), (C) | Z0HC
            0x89 => unimplemented!(),
            // ADC (A), (D) | Z0HC
            0x8A => unimplemented!(),
            // ADC (A), (E) | Z0HC
            0x8B => unimplemented!(),
            // ADC (A), (H) | Z0HC
            0x8C => unimplemented!(),
            // ADC (A), (L) | Z0HC
            0x8D => unimplemented!(),
            // ADC (A), HL | Z0HC
            0x8E => unimplemented!(),
            // ADC (A), (A) | Z0HC
            0x8F => unimplemented!(),
            // SUB (A), (B) | Z1HC
            0x90 => unimplemented!(),
            // SUB (A), (C) | Z1HC
            0x91 => unimplemented!(),
            // SUB (A), (D) | Z1HC
            0x92 => unimplemented!(),
            // SUB (A), (E) | Z1HC
            0x93 => unimplemented!(),
            // SUB (A), (H) | Z1HC
            0x94 => unimplemented!(),
            // SUB (A), (L) | Z1HC
            0x95 => unimplemented!(),
            // SUB (A), HL | Z1HC
            0x96 => unimplemented!(),
            // SUB (A), (A) | 1100
            0x97 => unimplemented!(),
            // SBC (A), (B) | Z1HC
            0x98 => unimplemented!(),
            // SBC (A), (C) | Z1HC
            0x99 => unimplemented!(),
            // SBC (A), (D) | Z1HC
            0x9A => unimplemented!(),
            // SBC (A), (E) | Z1HC
            0x9B => unimplemented!(),
            // SBC (A), (H) | Z1HC
            0x9C => unimplemented!(),
            // SBC (A), (L) | Z1HC
            0x9D => unimplemented!(),
            // SBC (A), HL | Z1HC
            0x9E => unimplemented!(),
            // SBC (A), (A) | Z1H-
            0x9F => unimplemented!(),
            // AND (A), (B) | Z010
            0xA0 => unimplemented!(),
            // AND (A), (C) | Z010
            0xA1 => unimplemented!(),
            // AND (A), (D) | Z010
            0xA2 => unimplemented!(),
            // AND (A), (E) | Z010
            0xA3 => unimplemented!(),
            // AND (A), (H) | Z010
            0xA4 => unimplemented!(),
            // AND (A), (L) | Z010
            0xA5 => unimplemented!(),
            // AND (A), HL | Z010
            0xA6 => unimplemented!(),
            // AND (A), (A) | Z010
            0xA7 => unimplemented!(),
            // XOR (A), (B) | Z000
            0xA8 => unimplemented!(),
            // XOR (A), (C) | Z000
            0xA9 => unimplemented!(),
            // XOR (A), (D) | Z000
            0xAA => unimplemented!(),
            // XOR (A), (E) | Z000
            0xAB => unimplemented!(),
            // XOR (A), (H) | Z000
            0xAC => unimplemented!(),
            // XOR (A), (L) | Z000
            0xAD => unimplemented!(),
            // XOR (A), HL | Z000
            0xAE => unimplemented!(),
            // XOR (A), (A) | 1000
            0xAF => unimplemented!(),
            // OR (A), (B) | Z000
            0xB0 => unimplemented!(),
            // OR (A), (C) | Z000
            0xB1 => unimplemented!(),
            // OR (A), (D) | Z000
            0xB2 => unimplemented!(),
            // OR (A), (E) | Z000
            0xB3 => unimplemented!(),
            // OR (A), (H) | Z000
            0xB4 => unimplemented!(),
            // OR (A), (L) | Z000
            0xB5 => unimplemented!(),
            // OR (A), HL | Z000
            0xB6 => unimplemented!(),
            // OR (A), (A) | Z000
            0xB7 => unimplemented!(),
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
            0xC6 => unimplemented!(),
            // RST ($00) | ----
            0xC7 => unimplemented!(),
            // RET (Z) | ----
            0xC8 => unimplemented!(),
            // RET  | ----
            0xC9 => unimplemented!(),
            // JP (Z), (a16) | ----
            0xCA => unimplemented!(),
            // PREFIX  | ----
            0xCB => unimplemented!(),
            // CALL (Z), (a16) | ----
            0xCC => unimplemented!(),
            // CALL (a16) | ----
            0xCD => unimplemented!(),
            // ADC (A), (n8) | Z0HC
            0xCE => unimplemented!(),
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
            0xD6 => unimplemented!(),
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
            0xDE => unimplemented!(),
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
            0xE6 => unimplemented!(),
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
            0xEE => unimplemented!(),
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
}
