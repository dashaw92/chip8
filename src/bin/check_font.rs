use chip8_hw::chip8::Chip8;

fn main() {
    let c8 = Chip8::load_rom(&[]);
    println!("{c8:#X?}");
}