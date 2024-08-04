use std::{convert::TryFrom, num::Wrapping};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CartridgeError {
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
    type Error = CartridgeError;

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
            v => Err(CartridgeError::InvalidCartridgeKind(v)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Licensee {
    None,
    Old(&'static str),
    New(&'static str),
}

impl TryFrom<u8> for Licensee {
    type Error = CartridgeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Licensee::None),
            0x01 => Ok(Licensee::Old("Nintendo")),
            0x08 => Ok(Licensee::Old("Capcom")),
            0x09 => Ok(Licensee::Old("HOT-B")),
            0x0A => Ok(Licensee::Old("Jaleco")),
            0x0B => Ok(Licensee::Old("Coconuts Japan")),
            0x0C => Ok(Licensee::Old("Elite Systems")),
            0x13 => Ok(Licensee::Old("EA (Electronic Arts)")),
            0x18 => Ok(Licensee::Old("Hudson Soft")),
            0x19 => Ok(Licensee::Old("ITC Entertainment")),
            0x1A => Ok(Licensee::Old("Yanoman")),
            0x1D => Ok(Licensee::Old("Japan Clary")),
            0x1F => Ok(Licensee::Old("Virgin Games Ltd.3")),
            0x24 => Ok(Licensee::Old("PCM Complete")),
            0x25 => Ok(Licensee::Old("San-X")),
            0x28 => Ok(Licensee::Old("Kemco")),
            0x29 => Ok(Licensee::Old("SETA Corporation")),
            0x30 => Ok(Licensee::Old("Infogrames5")),
            0x31 => Ok(Licensee::Old("Nintendo")),
            0x32 => Ok(Licensee::Old("Bandai")),
            // 0x33 (reserved) indicates to use new licensee
            0x34 => Ok(Licensee::Old("Konami")),
            0x35 => Ok(Licensee::Old("HectorSoft")),
            0x38 => Ok(Licensee::Old("Capcom")),
            0x39 => Ok(Licensee::Old("Banpresto")),
            0x3C => Ok(Licensee::Old(".Entertainment i")),
            0x3E => Ok(Licensee::Old("Gremlin")),
            0x41 => Ok(Licensee::Old("Ubi Soft1")),
            0x42 => Ok(Licensee::Old("Atlus")),
            0x44 => Ok(Licensee::Old("Malibu Interactive")),
            0x46 => Ok(Licensee::Old("Angel")),
            0x47 => Ok(Licensee::Old("Spectrum Holoby")),
            0x49 => Ok(Licensee::Old("Irem")),
            0x4A => Ok(Licensee::Old("Virgin Games Ltd.3")),
            0x4D => Ok(Licensee::Old("Malibu Interactive")),
            0x4F => Ok(Licensee::Old("U.S. Gold")),
            0x50 => Ok(Licensee::Old("Absolute")),
            0x51 => Ok(Licensee::Old("Acclaim Entertainment")),
            0x52 => Ok(Licensee::Old("Activision")),
            0x53 => Ok(Licensee::Old("Sammy USA Corporation")),
            0x54 => Ok(Licensee::Old("GameTek")),
            0x55 => Ok(Licensee::Old("Park Place")),
            0x56 => Ok(Licensee::Old("LJN")),
            0x57 => Ok(Licensee::Old("Matchbox")),
            0x59 => Ok(Licensee::Old("Milton Bradley Company")),
            0x5A => Ok(Licensee::Old("Mindscape")),
            0x5B => Ok(Licensee::Old("Romstar")),
            0x5C => Ok(Licensee::Old("Naxat Soft13")),
            0x5D => Ok(Licensee::Old("Tradewest")),
            0x60 => Ok(Licensee::Old("Titus Interactive")),
            0x61 => Ok(Licensee::Old("Virgin Games Ltd.3")),
            0x67 => Ok(Licensee::Old("Ocean Software")),
            0x69 => Ok(Licensee::Old("EA (Electronic Arts)")),
            0x6E => Ok(Licensee::Old("Elite Systems")),
            0x6F => Ok(Licensee::Old("Electro Brain")),
            0x70 => Ok(Licensee::Old("Infogrames5")),
            0x71 => Ok(Licensee::Old("Interplay Entertainment")),
            0x72 => Ok(Licensee::Old("Broderbund")),
            0x73 => Ok(Licensee::Old("Sculptured Software6")),
            0x75 => Ok(Licensee::Old("The Sales Curve Limited7")),
            0x78 => Ok(Licensee::Old("THQ")),
            0x79 => Ok(Licensee::Old("Accolade")),
            0x7A => Ok(Licensee::Old("Triffix Entertainment")),
            0x7C => Ok(Licensee::Old("Microprose")),
            0x7F => Ok(Licensee::Old("Kemco")),
            0x80 => Ok(Licensee::Old("Misawa Entertainment")),
            0x83 => Ok(Licensee::Old("Lozc")),
            0x86 => Ok(Licensee::Old("Tokuma Shoten")),
            0x8B => Ok(Licensee::Old("Bullet-Proof Software2")),
            0x8C => Ok(Licensee::Old("Vic Tokai")),
            0x8E => Ok(Licensee::Old("Ape")),
            0x8F => Ok(Licensee::Old("I'Max")),
            0x91 => Ok(Licensee::Old("Chunsoft Co.8")),
            0x92 => Ok(Licensee::Old("Video System")),
            0x93 => Ok(Licensee::Old("Tsubaraya Productions")),
            0x95 => Ok(Licensee::Old("Varie")),
            0x96 => Ok(Licensee::Old("Yonezawa/S’Pal")),
            0x97 => Ok(Licensee::Old("Kemco")),
            0x99 => Ok(Licensee::Old("Arc")),
            0x9A => Ok(Licensee::Old("Nihon Bussan")),
            0x9B => Ok(Licensee::Old("Tecmo")),
            0x9C => Ok(Licensee::Old("Imagineer")),
            0x9D => Ok(Licensee::Old("Banpresto")),
            0x9F => Ok(Licensee::Old("Nova")),
            0xA1 => Ok(Licensee::Old("Hori Electric")),
            0xA2 => Ok(Licensee::Old("Bandai")),
            0xA4 => Ok(Licensee::Old("Konami")),
            0xA6 => Ok(Licensee::Old("Kawada")),
            0xA7 => Ok(Licensee::Old("Takara")),
            0xA9 => Ok(Licensee::Old("Technos Japan")),
            0xAA => Ok(Licensee::Old("Broderbund")),
            0xAC => Ok(Licensee::Old("Toei Animation")),
            0xAD => Ok(Licensee::Old("Toho")),
            0xAF => Ok(Licensee::Old("Namco")),
            0xB0 => Ok(Licensee::Old("Acclaim Entertainment")),
            0xB1 => Ok(Licensee::Old("ASCII Corporation or Nexsoft")),
            0xB2 => Ok(Licensee::Old("Bandai")),
            0xB4 => Ok(Licensee::Old("Square Enix")),
            0xB6 => Ok(Licensee::Old("HAL Laboratory")),
            0xB7 => Ok(Licensee::Old("SNK")),
            0xB9 => Ok(Licensee::Old("Pony Canyon")),
            0xBA => Ok(Licensee::Old("Culture Brain")),
            0xBB => Ok(Licensee::Old("Sunsoft")),
            0xBD => Ok(Licensee::Old("Sony Imagesoft")),
            0xBF => Ok(Licensee::Old("Sammy Corporation")),
            0xC0 => Ok(Licensee::Old("Taito")),
            0xC2 => Ok(Licensee::Old("Kemco")),
            0xC3 => Ok(Licensee::Old("Square")),
            0xC4 => Ok(Licensee::Old("Tokuma Shoten")),
            0xC5 => Ok(Licensee::Old("Data East")),
            0xC6 => Ok(Licensee::Old("Tonkinhouse")),
            0xC8 => Ok(Licensee::Old("Koei")),
            0xC9 => Ok(Licensee::Old("UFL")),
            0xCA => Ok(Licensee::Old("Ultra")),
            0xCB => Ok(Licensee::Old("Vap")),
            0xCC => Ok(Licensee::Old("Use Corporation")),
            0xCD => Ok(Licensee::Old("Meldac")),
            0xCE => Ok(Licensee::Old("Pony Canyon")),
            0xCF => Ok(Licensee::Old("Angel")),
            0xD0 => Ok(Licensee::Old("Taito")),
            0xD1 => Ok(Licensee::Old("Sofel")),
            0xD2 => Ok(Licensee::Old("Quest")),
            0xD3 => Ok(Licensee::Old("Sigma Enterprises")),
            0xD4 => Ok(Licensee::Old("ASK Kodansha Co.")),
            0xD6 => Ok(Licensee::Old("Naxat Soft13")),
            0xD7 => Ok(Licensee::Old("Copya System")),
            0xD9 => Ok(Licensee::Old("Banpresto")),
            0xDA => Ok(Licensee::Old("Tomy")),
            0xDB => Ok(Licensee::Old("LJN")),
            0xDD => Ok(Licensee::Old("NCS")),
            0xDE => Ok(Licensee::Old("Human")),
            0xDF => Ok(Licensee::Old("Altron")),
            0xE0 => Ok(Licensee::Old("Jaleco")),
            0xE1 => Ok(Licensee::Old("Towa Chiki")),
            0xE2 => Ok(Licensee::Old("Yutaka")),
            0xE3 => Ok(Licensee::Old("Varie")),
            0xE5 => Ok(Licensee::Old("Epcoh")),
            0xE7 => Ok(Licensee::Old("Athena")),
            0xE8 => Ok(Licensee::Old("Asmik Ace Entertainment")),
            0xE9 => Ok(Licensee::Old("Natsume")),
            0xEA => Ok(Licensee::Old("King Records")),
            0xEB => Ok(Licensee::Old("Atlus")),
            0xEC => Ok(Licensee::Old("Epic/Sony Records")),
            0xEE => Ok(Licensee::Old("IGS")),
            0xF0 => Ok(Licensee::Old("A Wave")),
            0xF3 => Ok(Licensee::Old("Extreme Entertainment")),
            0xFF => Ok(Licensee::Old("LJN")),
            v => Err(CartridgeError::InvalidOldLicenseeCode(v)),
        }
    }
}

impl TryFrom<(char, char)> for Licensee {
    type Error = CartridgeError;

    fn try_from(value: (char, char)) -> Result<Self, Self::Error> {
        match value {
            ('0', '0') => Ok(Licensee::None),
            ('0', '1') => Ok(Licensee::New("Nintendo R&D1")),
            ('0', '8') => Ok(Licensee::New("Capcom")),
            ('1', '3') => Ok(Licensee::New("Electronic Arts")),
            ('1', '8') => Ok(Licensee::New("Hudson Soft")),
            ('1', '9') => Ok(Licensee::New("b-ai")),
            ('2', '0') => Ok(Licensee::New("kss")),
            ('2', '2') => Ok(Licensee::New("pow")),
            ('2', '4') => Ok(Licensee::New("PCM Complete")),
            ('2', '5') => Ok(Licensee::New("san-x")),
            ('2', '8') => Ok(Licensee::New("Kemco Japan")),
            ('2', '9') => Ok(Licensee::New("seta")),
            ('3', '0') => Ok(Licensee::New("Viacom")),
            ('3', '1') => Ok(Licensee::New("Nintendo")),
            ('3', '2') => Ok(Licensee::New("Bandai")),
            ('3', '3') => Ok(Licensee::New("Ocean/Acclaim")),
            ('3', '4') => Ok(Licensee::New("Konami")),
            ('3', '5') => Ok(Licensee::New("Hector")),
            ('3', '7') => Ok(Licensee::New("Taito")),
            ('3', '8') => Ok(Licensee::New("Hudson")),
            ('3', '9') => Ok(Licensee::New("Banpresto")),
            ('4', '1') => Ok(Licensee::New("Ubi Soft")),
            ('4', '2') => Ok(Licensee::New("Atlus")),
            ('4', '4') => Ok(Licensee::New("Malibu")),
            ('4', '6') => Ok(Licensee::New("angel")),
            ('4', '7') => Ok(Licensee::New("Bullet-Proof")),
            ('4', '9') => Ok(Licensee::New("irem")),
            ('5', '0') => Ok(Licensee::New("Absolute")),
            ('5', '1') => Ok(Licensee::New("Acclaim")),
            ('5', '2') => Ok(Licensee::New("Activision")),
            ('5', '3') => Ok(Licensee::New("American sammy")),
            ('5', '4') => Ok(Licensee::New("Konami")),
            ('5', '5') => Ok(Licensee::New("Hi tech entertainment")),
            ('5', '6') => Ok(Licensee::New("LJN")),
            ('5', '7') => Ok(Licensee::New("Matchbox")),
            ('5', '8') => Ok(Licensee::New("Mattel")),
            ('5', '9') => Ok(Licensee::New("Milton Bradley")),
            ('6', '0') => Ok(Licensee::New("Titus")),
            ('6', '1') => Ok(Licensee::New("Virgin")),
            ('6', '4') => Ok(Licensee::New("LucasArts")),
            ('6', '7') => Ok(Licensee::New("Ocean")),
            ('6', '9') => Ok(Licensee::New("Electronic Arts")),
            ('7', '0') => Ok(Licensee::New("Infogrames")),
            ('7', '1') => Ok(Licensee::New("Interplay")),
            ('7', '2') => Ok(Licensee::New("Broderbund")),
            ('7', '3') => Ok(Licensee::New("sculptured")),
            ('7', '5') => Ok(Licensee::New("sci")),
            ('7', '8') => Ok(Licensee::New("THQ")),
            ('7', '9') => Ok(Licensee::New("Accolade")),
            ('8', '0') => Ok(Licensee::New("misawa")),
            ('8', '3') => Ok(Licensee::New("lozc")),
            ('8', '6') => Ok(Licensee::New("Tokuma Shoten Intermedia")),
            ('8', '7') => Ok(Licensee::New("Tsukuda Original")),
            ('9', '1') => Ok(Licensee::New("Chunsoft")),
            ('9', '2') => Ok(Licensee::New("Video system")),
            ('9', '3') => Ok(Licensee::New("Ocean/Acclaim")),
            ('9', '5') => Ok(Licensee::New("Varie")),
            ('9', '6') => Ok(Licensee::New("Yonezawa/s'pal")),
            ('9', '7') => Ok(Licensee::New("Kaneko")),
            ('9', '9') => Ok(Licensee::New("Pack in soft")),
            ('A', '4') => Ok(Licensee::New("Konami (Yu-Gi-Oh!)")),
            (a, b) => Err(CartridgeError::InvalidNewLicenseeCode(a, b)),
        }
    }
}

const NINTENDO_LOGO: [u8; 48] = [
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];

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

    pub fn read(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        panic!(
            "not implemented: write to cartridge: address: {:#06x}, value: {:#04x}",
            address, value
        );
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
    pub fn kind(&self) -> Result<Kind, CartridgeError> {
        Kind::try_from(self.rom[0x147])
    }

    // Indicates the game's publisher
    pub fn licensee(&self) -> Result<Licensee, CartridgeError> {
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

#[cfg(test)]
mod tests {
    use super::*;

    const CPU_INSTRS_ROM: &[u8; 65536] = include_bytes!("../test/fixtures/cpu_instrs.gb");

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
            CartridgeError::InvalidCartridgeKind(0xEE),
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
            CartridgeError::InvalidOldLicenseeCode(0xF4),
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
            CartridgeError::InvalidNewLicenseeCode('Z', 'Z'),
            "invalid new licensee"
        );
    }

    #[test]
    fn test_rom_size() {
        let cart = Cartridge::new(CPU_INSTRS_ROM.to_vec());
        assert_eq!(cart.rom_size(), 64, "cpu_instrs rom size");
    }
}
