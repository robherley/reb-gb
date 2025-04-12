use reb_gb::{
    cartridge::Cartridge,
    cpu::{Model, CPU},
};

fn main() {
    let rom = include_bytes!("../tmp/gb-test-roms/cpu_instrs/individual/03-op sp,hl.gb").to_vec();
    let cartridge = Cartridge::new(rom);
    pretty_print(&cartridge);

    let mut cpu = CPU::new(Model::DMG, cartridge);
    cpu.debug_mode(true);
    cpu.boot();
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
