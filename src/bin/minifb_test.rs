use std::fs::File;
use std::io::{Read, Write};
use chip8_hw::chip8::{Chip8, VRAM_HEIGHT, VRAM_WH, VRAM_WIDTH};
use chip8_hw::keyboard::Key;
use minifb::{Key as FBKey, KeyRepeat, Window, WindowOptions};

static KEY_MAP: &[(FBKey, Key)] = &[
    (FBKey::Key1, Key::K1),
    (FBKey::Key2, Key::K2),
    (FBKey::Key3, Key::K3),
    (FBKey::Key4, Key::KC),
    (FBKey::Q   , Key::K4),
    (FBKey::W   , Key::K5),
    (FBKey::E   , Key::K6),
    (FBKey::R   , Key::KD),
    (FBKey::A   , Key::K7),
    (FBKey::S   , Key::K8),
    (FBKey::D   , Key::K9),
    (FBKey::F   , Key::KE),
    (FBKey::Z   , Key::KA),
    (FBKey::X   , Key::K0),
    (FBKey::C   , Key::KB),
    (FBKey::V   , Key::KF),
];

fn main() {
    let (rom_name, mut c8) = chip8();
    let (active, halted) = (format!("chip8 - {rom_name}"), format!("<HALTED> - chip8 - {rom_name}"));

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
    .unwrap();

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(FBKey::Escape) {
        update_key_states(&mut c8, &window);
        window.set_title(if c8.is_halted() {
            &halted
        } else {
            &active
        });

        if !c8.is_halted() {
            match c8.step(next_key(&window)) {
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

//If any key was released, return it (LDKB)
//Otherwise, none. When LDKB checks this,
//if it's none, the emulator will loop back to
//the instruction. ST and DT are not updated,
//faking "halting" the emulator until a key is ready.
fn next_key(window: &Window) -> Option<Key> {
    KEY_MAP.iter()
        .find(|(fbkey, _)| window.is_key_released(*fbkey))
        .map(|(_, key)| *key)
}

fn update_key_states(c8: &mut Chip8, window: &Window) {
    let pressed = window.get_keys();
    let released = window.get_keys_released();

    for (fbkey, key) in KEY_MAP {
        if pressed.contains(fbkey) {
            c8.keyboard[*key] = true;
        } else if released.contains(fbkey) {
            c8.keyboard[*key] = false;
        }
    }
}