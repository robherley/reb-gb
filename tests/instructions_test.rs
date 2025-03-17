use reb_gb::{
    cartridge::Cartridge,
    cpu::{Model, CPU},
};

const CPU_INSTRS_ROM: &[u8; 65536] = include_bytes!("./fixtures/cpu_instrs.gb");

#[test]
fn test_cpu_instrs() {
    let cart = Cartridge::new(CPU_INSTRS_ROM.to_vec());
    let mut _cpu = CPU::new(Model::DMG, cart);

    // cpu.boot();
}
