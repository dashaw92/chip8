use chip8_hw::chip8::{Chip8, QUIRKS_NEW};

fn main() {
    let c8 = Chip8::load_rom(QUIRKS_NEW, &[]);
    println!("{c8:#X?}");
}