use std::fs::File;
use std::io::Read;

use chip8_decode::instructions::Instr;

fn main() {
    let path = std::env::args().nth(1).unwrap_or("rom.c8".into());
    let file = File::open(&path).expect(&format!("Failed to open file \"{path}\" (does it exist?)"));
    let bytes: Vec<u8> = file.bytes()
        .filter_map(|byte| byte.ok())
        .collect();
    
    dbg!(&bytes);

    bytes.chunks(2)
        .map(|bytes: &[u8]| (bytes[0] as u16) << 8 | bytes[1] as u16)
        .map(Instr::decode)
        .for_each(|instr| { println!("{instr:?}"); });
}