use crate::mmu;
use std::{convert::TryFrom, num::Wrapping};

use super::metadata::Licensee;

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("invalid cartridge kind: {0:#04x}")]
    InvalidCartridgeKind(u8),
    #[error("invalid old licensee code: {0:#04x}")]
    InvalidOldLicenseeCode(u8),
    #[error("invalid new licensee code: {0}{1}")]
    InvalidNewLicenseeCode(char, char),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Kind {
    RomOnly,
    Mbc1,
    Mb1Ram,
    Mbc1RamBattery,
    Mbc2,
    Mbc2Battery,
    RomRam,
    RomRamBattery,
    Mmm01,
    Mmm01Ram,
    Mmm01RamBattery,
    Mbc3TimerBattery,
    Mbc3TimerRamBattery,
    Mbc3,
    Mbc3Ram,
    Mbc3RamBattery,
    Mbc5,
    Mbc5Ram,
    Mbc5RamBattery,
    Mbc5Rumble,
    Mbc5RumbleRam,
    Mbc5RumbleRamBattery,
    Mbc6,
    Mbc7SensorRumbleRamBattery,
    PocketCamera,
    BandaiTama5,
    Huc3,
    Huc1RamBattery,
}

impl TryFrom<u8> for Kind {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Kind::RomOnly),
            0x01 => Ok(Kind::Mbc1),
            0x02 => Ok(Kind::Mb1Ram),
            0x03 => Ok(Kind::Mbc1RamBattery),
            0x05 => Ok(Kind::Mbc2),
            0x06 => Ok(Kind::Mbc2Battery),
            0x08 => Ok(Kind::RomRam),
            0x09 => Ok(Kind::RomRamBattery),
            0x0B => Ok(Kind::Mmm01),
            0x0C => Ok(Kind::Mmm01Ram),
            0x0D => Ok(Kind::Mmm01RamBattery),
            0x0F => Ok(Kind::Mbc3TimerBattery),
            0x10 => Ok(Kind::Mbc3TimerRamBattery),
            0x11 => Ok(Kind::Mbc3),
            0x12 => Ok(Kind::Mbc3Ram),
            0x13 => Ok(Kind::Mbc3RamBattery),
            0x19 => Ok(Kind::Mbc5),
            0x1A => Ok(Kind::Mbc5Ram),
            0x1B => Ok(Kind::Mbc5RamBattery),
            0x1C => Ok(Kind::Mbc5Rumble),
            0x1D => Ok(Kind::Mbc5RumbleRam),
            0x1E => Ok(Kind::Mbc5RumbleRamBattery),
            0x20 => Ok(Kind::Mbc6),
            0x22 => Ok(Kind::Mbc7SensorRumbleRamBattery),
            0xFC => Ok(Kind::PocketCamera),
            0xFD => Ok(Kind::BandaiTama5),
            0xFE => Ok(Kind::Huc3),
            0xFF => Ok(Kind::Huc1RamBattery),
            v => Err(Error::InvalidCartridgeKind(v)),
        }
    }
}

const NINTENDO_LOGO: [u8; 48] = [
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];

#[derive(Debug, PartialEq, Eq)]
pub enum ColorMode {
    None,
    Supports,
    Required,
}

// https://gbdev.io/pandocs/The_Cartridge_Header.html
pub struct Cartridge {
    pub rom: Vec<u8>,
}

impl Cartridge {
    pub fn new(rom: Vec<u8>) -> Self {
        Cartridge { rom }
    }

    // First address the boot rom jumps to after checking nintendo logo. Usually a NOP then JP $0150
    pub fn entry_point(&self) -> &[u8] {
        &self.rom[0x100..0x104]
    }

    // Area that is expected to contain a bitmap of the nintendo logo
    pub fn nintendo_logo(&self) -> &[u8] {
        &self.rom[0x104..0x134]
    }

    // Cartridge logo area must match specific hexidecimal values to be "valid" or else the boot rom locks up.
    // Interestingly, gameboy colors and later only checks the top half of the logo.
    // https://gbdev.io/pandocs/Power_Up_Sequence.html?highlight=half#behavior
    pub fn is_logo_match(&self) -> bool {
        self.nintendo_logo() == &NINTENDO_LOGO
    }

    // Title of the game in uppercase ASCII. 16 bytes (chars) max, padded with 0x00
    // Later cartridges trim the title and use the bytes for other information
    // Example:
    //  color_mode: title is only 15 bytes ($0134–$0142)
    //  manufacturer_code: title is only 11 bytes ($0134–$013E)
    pub fn title(&self) -> String {
        let title = &self.rom[0x134..0x144];
        title
            .iter()
            .take_while(|&&c| c != 0x00)
            .map(|&c| c as char)
            .collect()
    }

    // In newer cartridges, this is a 4-character manufacturer code (in uppercase ASCII)
    pub fn manufacturer_code(&self) -> &[u8] {
        &self.rom[0x13F..0x143]
    }

    // The Color and later models use this byte to determine if the game supports color features.
    pub fn color_mode(&self) -> ColorMode {
        match self.rom[0x143] {
            0x80 => ColorMode::Supports,
            0xC0 => ColorMode::Required,
            _ => ColorMode::None,
        }
    }

    // This byte specifies whether the cartridge is a Super GameBoy cartridge.
    // Technically old licensee code should be nintendo (0x01) too
    pub fn is_super_gameboy(&self) -> bool {
        self.rom[0x146] == 0x03
    }

    // This byte specifies the type of cartridge, can be used to determine memory bank controller.
    pub fn kind(&self) -> Result<Kind, Error> {
        Kind::try_from(self.rom[0x147])
    }

    // Indicates the game's publisher
    pub fn licensee(&self) -> Result<Licensee, Error> {
        match self.rom[0x14B] {
            0x33 => Licensee::try_from((self.rom[0x144] as char, self.rom[0x145] as char)),
            v => Licensee::try_from(v),
        }
    }

    // This byte specifies the cartridge's ROM size.
    // 32 KiB × (1 << <value>)
    pub fn rom_size(&self) -> u8 {
        32 * (1 << self.rom[0x148])
    }

    // This byte specifies the cartridge's RAM size.
    pub fn ram_size(&self) -> u8 {
        // todo(robherley): should be 0 if no ram (check kind)
        // maybe derive number of banks from value or just from the kind
        self.rom[0x149]
    }

    // This byte specifies whether this version of the game is intended to be sold in Japan or elsewhere.
    pub fn is_sold_overseas(&self) -> bool {
        self.rom[0x14A] == 0x01
    }

    // This byte specifies the version number of the game. It is usually $00.
    pub fn mask_rom_version(&self) -> u8 {
        self.rom[0x14C]
    }

    // An 8-bit checksum computed from the cartridge header bytes $0134–014C. The boot ROM verifies this checksum.
    pub fn header_checksum(&self) -> u8 {
        self.rom[0x14D]
    }

    // Computes the header checksum to see if it's valid. The boot ROM verifies this checksum.
    pub fn is_header_checksum_valid(&self) -> bool {
        let mut checksum: u8 = 0;

        for addr in 0x134..=0x14C {
            checksum = checksum.wrapping_sub(self.rom[addr]).wrapping_sub(1);
        }

        checksum == self.header_checksum()
    }

    // A 16-bit checksum computed from the summing the entire cartridge ROM. This is _NOT_ verified by the boot ROM.
    pub fn global_checksum(&self) -> u16 {
        u16::from_be_bytes([self.rom[0x14E], self.rom[0x14F]])
    }

    // Computes the header checksum to see if it's valid. This is _NOT_ verified by the boot ROM.
    pub fn is_global_checksum_valid(&self) -> bool {
        let checksum = self
            .rom
            .iter()
            .enumerate()
            .map(|(i, &b)| {
                match i as u16 {
                    // skip global checksum bytes
                    0x14E | 0x14F => Wrapping(0),
                    _ => Wrapping(b as u16),
                }
            })
            .sum::<Wrapping<u16>>()
            .0;

        checksum == self.global_checksum()
    }
}

impl mmu::RW for Cartridge {
    fn read(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn write(&mut self, address: u16, value: u8) {
        unreachable!(
            "write to cartridge: address: {:#06x}, value: {:#04x}",
            address, value
        );
    }
}

impl Default for Cartridge {
    fn default() -> Self {
        Cartridge {
            rom: vec![0; 65536],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CPU_INSTRS_ROM: &[u8; 65536] = include_bytes!("../../tests/fixtures/cpu_instrs.gb");

    #[test]
    fn test_attributes() {
        let cart = Cartridge::new(CPU_INSTRS_ROM.to_vec());
        assert_eq!(cart.rom.len(), 65536, "rom size");
        assert_eq!(cart.entry_point(), &[0x00, 0xc3, 0x37, 0x06], "entry point");
        assert_eq!(cart.nintendo_logo(), &NINTENDO_LOGO, "nintendo logo");
        assert_eq!(cart.title(), "CPU_INSTRS", "title");
        assert!(cart.is_logo_match(), "matches nintendo logo");
    }

    #[test]
    fn test_kind() {
        let cart = Cartridge::new(CPU_INSTRS_ROM.to_vec());
        assert_eq!(
            cart.kind().unwrap(),
            Kind::Mbc1,
            "cpu_instrs should be mbc1"
        );

        let cart = Cartridge::new(vec![0x00; 65536]);
        assert_eq!(cart.kind().unwrap(), Kind::RomOnly, "rom only",);

        let cart = Cartridge::new(vec![0xEE; 65536]);
        assert_eq!(
            cart.kind().unwrap_err(),
            Error::InvalidCartridgeKind(0xEE),
            "invalid type"
        );
    }

    #[test]
    fn test_licensee() {
        let cart = Cartridge::new(CPU_INSTRS_ROM.to_vec());
        assert_eq!(
            cart.licensee().unwrap(),
            Licensee::None,
            "cpu_instrs licensee"
        );

        let cart = Cartridge::new(vec![0x00; 65536]);
        assert_eq!(cart.licensee().unwrap(), Licensee::None, "no licensee");

        let mut rom = vec![0x00; 65536];
        rom[0x14B] = 0xAF;
        assert_eq!(
            Cartridge::new(rom).licensee().unwrap(),
            Licensee::Old("Namco"),
            "old licensee"
        );

        let mut rom = vec![0x00; 65536];
        rom[0x14B] = 0xF4;
        assert_eq!(
            Cartridge::new(rom).licensee().unwrap_err(),
            Error::InvalidOldLicenseeCode(0xF4),
            "invalid old licensee"
        );

        let mut rom = vec![0x00; 65536];
        rom[0x14B] = 0x33;
        rom[0x144] = '6' as u8;
        rom[0x145] = '9' as u8;
        assert_eq!(
            Cartridge::new(rom).licensee().unwrap(),
            Licensee::New("Electronic Arts"),
            "new licensee"
        );

        let mut rom = vec![0x00; 65536];
        rom[0x14B] = 0x33;
        rom[0x144] = 'Z' as u8;
        rom[0x145] = 'Z' as u8;
        assert_eq!(
            Cartridge::new(rom).licensee().unwrap_err(),
            Error::InvalidNewLicenseeCode('Z', 'Z'),
            "invalid new licensee"
        );
    }

    #[test]
    fn test_color_mode() {
        let cart = Cartridge::new(CPU_INSTRS_ROM.to_vec());
        assert_eq!(cart.color_mode(), ColorMode::Supports);
    }

    #[test]
    fn test_rom_size() {
        let cart = Cartridge::new(CPU_INSTRS_ROM.to_vec());
        assert_eq!(cart.rom_size(), 64);
    }

    #[test]
    fn test_ram_size() {
        let cart = Cartridge::new(CPU_INSTRS_ROM.to_vec());
        assert_eq!(cart.ram_size(), 0);
    }

    #[test]
    fn test_is_sold_overseas() {
        let cart = Cartridge::new(CPU_INSTRS_ROM.to_vec());
        assert!(!cart.is_sold_overseas());
    }

    #[test]
    fn test_mask_rom_version() {
        let cart = Cartridge::new(CPU_INSTRS_ROM.to_vec());
        assert_eq!(cart.mask_rom_version(), 0);
    }

    #[test]
    fn test_header_checksum() {
        let cart = Cartridge::new(CPU_INSTRS_ROM.to_vec());
        assert_eq!(cart.header_checksum(), 0x3B);
        assert!(cart.is_header_checksum_valid());
    }

    #[test]
    fn test_global_checksum() {
        let mut rom = vec![0x01; 65536];
        rom[0x14E] = 0xFF;
        rom[0x14F] = 0xFE;
        let cart = Cartridge::new(rom);
        assert_eq!(cart.global_checksum(), 0xFFFE);
        assert!(cart.is_global_checksum_valid());
    }
}
