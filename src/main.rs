use reb_gb::cartridge::Cartridge;

fn main() {
    // let rom = include_bytes!("../tmp/roms/tetris.gb").to_vec();
    let rom = include_bytes!("../test/fixtures/cpu_instrs.gb").to_vec();

    let cartridge = Cartridge::new(rom);
    pretty_print(cartridge);
}

fn pretty_print(cart: Cartridge) {
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
