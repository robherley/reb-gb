use reb_gb::{
    cartridge::Cartridge,
    cpu::{Model, CPU},
};

fn main() {
    let rom = include_bytes!("../tests/fixtures/cpu_instrs.gb").to_vec();
    let cartridge = Cartridge::new(rom);
    pretty_print(&cartridge);

    let mut cpu = CPU::new(Model::DMG, cartridge);
    cpu.boot();
}

fn pretty_print(cart: &Cartridge) {
    println!("size: {:?}", cart.rom.len());
    println!("nintendo logo matches: {:?}", cart.is_logo_match());
    println!("title: {:?}", cart.title());
    println!("licensee code: {:?}", cart.licensee());
    println!("rom size: {:?}", cart.rom_size());
    println!("ram size: {:?}", cart.ram_size());
    println!(
        "header checksum: {:?} | valid? {:?}",
        cart.header_checksum(),
        cart.is_header_checksum_valid()
    );
    println!(
        "global checksum: {:#04x} | valid? {:?}",
        cart.global_checksum(),
        cart.is_global_checksum_valid()
    );
}
