//! Diagnostic: boot a ROM headlessly and report frame statistics over time.
//!
//! Usage: cargo run --release --example probe_boot -- <rom> [bios]
use oemu_emu::adapter_rustboyadvance::CoreEmulator;
use oemu_emu::Emulator;

fn main() {
    let mut args = std::env::args().skip(1);
    let rom_path = args.next().expect("usage: probe_boot <rom> [bios]");
    let bios_path = args.next();

    let rom = std::fs::read(&rom_path).expect("read rom");
    let mut emu = CoreEmulator::new();
    if let Some(p) = bios_path {
        let bios = std::fs::read(&p).expect("read bios");
        emu = emu.with_bios(bios).expect("valid bios");
    }
    emu.load_rom(&rom).expect("load rom");

    for i in 1..=1800 {
        emu.run_frame();
        if i % 120 == 0 {
            let bytes = emu.frame().as_bytes();
            let distinct = bytes
                .chunks_exact(4)
                .map(|p| (p[0], p[1], p[2]))
                .collect::<std::collections::HashSet<_>>()
                .len();
            println!(
                "frame {i:5}: distinct_colors={distinct:5} first_px=({},{},{})",
                bytes[0], bytes[1], bytes[2]
            );
        }
    }
}
