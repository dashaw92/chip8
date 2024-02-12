use std::fs::File;
use std::io::{Read, Write};
use chip8_hw::chip8::{Chip8, VRAM_HEIGHT, VRAM_WH, VRAM_WIDTH};
use minifb::{Key as FBKey, KeyRepeat, Window, WindowOptions};

pub const SCALE: u32 = 8;

fn main() {
    let (rom_name, mut c8) = chip8();
    let (active, halted) = (format!("chip8 - {rom_name}"), format!("<HALTED> - chip8 - {rom_name}"));
    let stdin = std::io::stdin();
    let mut lock = stdin.lock();

    let stdout = std::io::stdout();
    let mut out = stdout.lock();

    let mut buffer: Vec<u32> = vec![0; VRAM_WH];
    let mut window = Window::new(
        &active,
        VRAM_WIDTH,
        VRAM_HEIGHT,
        WindowOptions {
            resize: false,
            scale: minifb::Scale::X8,
            ..Default::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(FBKey::Escape) {
        window.set_title(if c8.is_halted() {
            &halted
        } else {
            &active
        });

        if !c8.is_halted() {
            match c8.step(&mut lock) {
                Ok(instr) => {
                    let _ = writeln!(out, "{instr:?}");
                },
                Err(e) => {
                    let _ = writeln!(out, "Execution halted: {e:?}.");
                    c8.set_halted(true);
                }
            }

            for y in 0..VRAM_HEIGHT {
                for x in 0..VRAM_WIDTH {
                    let idx = y * VRAM_WIDTH + x;
                    buffer[idx] = match c8.vram[idx] {
                        true => 0x00FFAA00,
                        false => 0,
                    };
                }
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, VRAM_WIDTH, VRAM_HEIGHT)
            .unwrap();

        if window.is_key_pressed(FBKey::Space, KeyRepeat::No) {
            c8.set_halted(!c8.is_halted());
        }
    }
}

fn chip8() -> (String, Chip8) {
    let path = std::env::args().nth(1).unwrap_or("rom.c8".into());
    let file = File::open(&path).expect(&format!("Failed to open file \"{path}\" (does it exist?)"));
    let bytes: Vec<u8> = file.bytes()
        .filter_map(|byte| byte.ok())
        .collect();
    
    (path, Chip8::load_rom(&bytes))
}