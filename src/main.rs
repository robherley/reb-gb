use std::sync::LazyLock;

use reb_gb::{
    cartridge::Cartridge,
    cpu::{Model, CPU},
};

// TODO(robherley): tmp load rom at runtime so build in CI (where this rom might not exist) passes
static ROM_DATA: LazyLock<Vec<u8>> = LazyLock::new(|| {
    std::fs::read("tmp/gb-test-roms/cpu_instrs/individual/04-op r,imm.gb")
        .expect("Failed to read ROM file")
});

fn main() {
    // 09 - 88 89 8A 8B 8C 8D 8F 98 99 9A 9B 9C 9D 9F | Failed
    // 11 - 8E 9E | Failed
    let rom = ROM_DATA.clone();
    let cartridge = Cartridge::new(rom);
    pretty_print(&cartridge);

    let mut cpu = CPU::new(Model::DEBUG, cartridge);
    cpu.debug_mode(true);
    if let Err(err) = cpu.boot() {
        panic!("Encounted fatal error: {}", err);
    }
}

fn pretty_print(cart: &Cartridge) {
    eprintln!("size: {:?}", cart.rom.len());
    eprintln!("nintendo logo matches: {:?}", cart.is_logo_match());
    eprintln!("title: {:?}", cart.title());
    eprintln!("licensee code: {:?}", cart.licensee());
    eprintln!("rom size: {:?}", cart.rom_size());
    eprintln!("ram size: {:?}", cart.ram_size());
    eprintln!(
        "header checksum: {:?} | valid? {:?}",
        cart.header_checksum(),
        cart.is_header_checksum_valid()
    );
    eprintln!(
        "global checksum: {:#04x} | valid? {:?}",
        cart.global_checksum(),
        cart.is_global_checksum_valid()
    );
}
