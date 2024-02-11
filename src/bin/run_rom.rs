use std::fs::File;
use std::io::{Read, Write};
use std::time::Duration;

use chip8_hw::chip8::{Chip8, VRAM_HEIGHT, VRAM_WIDTH};

fn main() {
    // test();
    let path = std::env::args().nth(1).unwrap_or("rom.c8".into());
    let file = File::open(&path).expect(&format!("Failed to open file \"{path}\" (does it exist?)"));
    let bytes: Vec<u8> = file.bytes()
        .filter_map(|byte| byte.ok())
        .collect();
    
    let mut c8 = Chip8::load_rom(&bytes);

    let stdin = std::io::stdin();
    let mut lock = stdin.lock();

    let stdout = std::io::stdout();
    let mut out = stdout.lock();

    loop {
        let result = c8.step(&mut lock);

        match result {
            // Ok(instr) => println!("{instr:?}"),
            Err(e) => {
                dbg!(e);
                break;
            }
            _ => {},
        }

        // println!("regs = {:04X?}, pc = {:4X}, sp = {:04X}, stack = {:04X?}", c8.gpregs, c8.pc, c8.sp, c8.stack);
        cls(&mut out);
        display(&mut out, &c8.vram);
        std::thread::sleep(Duration::from_millis(10));
    }
    println!("Execution halted.");
}

fn cls(out: &mut impl Write) {
    let _ = write!(out, "{0}[2J{0}[1;1H", 27 as char);
}

fn display(out: &mut impl Write, vram: &[bool]) {
    for y in 0 .. VRAM_HEIGHT {
        for x in 0 .. VRAM_WIDTH {
            let _ = if vram[y * VRAM_WIDTH + x] {
                write!(out, "#")
            } else {
                write!(out, " ")
            };
        }
        let _ = writeln!(out, "");
    }

    let _ = out.flush();
}

fn test() {
    let x = [
        [0xFF,0x0, 0xFF, 0x0, 0x3C, 0x0, 0x3C, 0x0, 0x3C, 0x0, 0x3C, 0x0, 0xFF, 0x0, 0xFF, 0xFF],
        [0xFF,0x0, 0xFF, 0x0, 0x38, 0x0, 0x3F, 0x0, 0x3F, 0x0, 0x38, 0x0, 0xFF, 0x0, 0xFF, 0x80],
        [0x80,0x0, 0xE0, 0x0, 0xE0, 0x0, 0x80, 0x0, 0x80, 0x0, 0xE0, 0x0, 0xE0, 0x0, 0x80, 0xF8],
        [0xF8,0x0, 0xFC, 0x0, 0x3E, 0x0, 0x3F, 0x0, 0x3B, 0x0, 0x39, 0x0, 0xF8, 0x0, 0xF8, 0x3],
        [0x3, 0x0, 0x7,  0x0, 0x0F, 0x0, 0xBF, 0x0, 0xFB, 0x0, 0xF3, 0x0, 0xE3, 0x0, 0x43, 0xE0],
        [0xE0,0x0, 0xE0, 0x0, 0x80, 0x0, 0x80, 0x0, 0x80, 0x0, 0x80, 0x0, 0xE0, 0x0, 0xE0, 0x0]
    ];

    for sprite in x {
        for byte in sprite {
            for x in 0 ..= 7 {
                let mask = 0b10000000 >> x;
                let bit = byte & mask == mask;
                if bit {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
        }
        println!("");
    }
}