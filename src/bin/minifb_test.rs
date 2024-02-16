use std::fs::File;
use std::io::{Read, Write};
use chip8_decode::instructions::Instr;
use chip8_hw::chip8::{Chip8, QUIRKS_NEW, STACK_LIMIT, VRAM_HEIGHT, VRAM_WH, VRAM_WIDTH};
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

struct Scheme {
    fg: u32,
    bg: u32,
}

impl Scheme {
    fn from_env() -> Scheme {
        let default = Scheme {
            fg: 0x00FFAA00,
            bg: 0,
        };

        if let Some(arg) = std::env::args().skip(2).next() {
            return match arg.to_lowercase().as_ref() {
                "light" => Scheme {
                    fg: 0,
                    bg: 0x00FFFFFF,
                },
                _ => default,
            }
        }

        default
    }
}

fn main() {
    let (rom_name, mut c8) = chip8();
    let scheme = Scheme::from_env();
    let (active, halted) = (format!("chip8 - {rom_name}"), format!("<HALTED> - chip8 - {rom_name}"));

    let stdout = std::io::stdout();
    let mut out = stdout.lock();

    let mut display_buf: Vec<u32> = vec![0; VRAM_WH];
    let mut display = Window::new(
        &active,
        VRAM_WIDTH,
        VRAM_HEIGHT,
        WindowOptions {
            resize: false,
            scale: minifb::Scale::X8,
            ..Default::default()
        },
    ).expect("Failed to create c8 display window.");

    //Clear the terminal for the debug console output
    print!("{esc}[2J", esc = 27 as char);
    let mut do_one_step = false;

    while display.is_open() && !display.is_key_down(FBKey::Escape) {
        update_key_states(&mut c8, &display);
        display.set_title(if c8.is_halted() { &halted } else { &active });

        if !c8.is_halted() {
            match c8.step(next_key(&display)) {
                Err(e) => {
                    let _ = writeln!(out, "Execution halted: {e:?}.");
                    c8.set_halted(true);
                }
                Ok(ins) => print_env(&mut out, &c8, ins),
            }

            display_buf.iter_mut()
                .enumerate()
                .for_each(|(idx, pix)| *pix = if c8.vram[idx] { scheme.fg } else { scheme.bg });
        }

        display
            .update_with_buffer(&display_buf, VRAM_WIDTH, VRAM_HEIGHT)
            .expect("Failed to update display buffer on window.");
        
        if do_one_step {
            c8.set_halted(true);
            do_one_step = false;
        }

        if display.is_key_pressed(FBKey::Space, KeyRepeat::No) {
            c8.set_halted(!c8.is_halted());
        } else if display.is_key_pressed(FBKey::N, KeyRepeat::Yes) {
            c8.set_halted(false);
            do_one_step = true;
        }
    }
}

fn chip8() -> (String, Chip8) {
    let path = std::env::args().nth(1).unwrap_or("rom.c8".into());
    let file = File::open(&path).expect(&format!("Failed to open file \"{path}\" (does it exist?)"));
    let bytes: Vec<u8> = file.bytes()
        .filter_map(|byte| byte.ok())
        .collect();
    
    (path, Chip8::load_rom(QUIRKS_NEW, &bytes))
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

#[allow(unused_must_use)]
fn print_env(out: &mut impl Write, c8: &Chip8, ins: Instr) {
    //https://stackoverflow.com/a/34837038
    #[inline(always)]
    fn move_to(out: &mut impl Write, x: usize, y: usize) {
        let _ = write!(out, "{esc}[{y};{x}H", esc = 27 as char);
    }

    let mut buf = String::new();
    buf.push_str("REGS:\n");
    for reg in 0..c8.gpregs.len() {
        if reg > 0 && reg % 4 == 0 {
            buf.push_str("\n");
        }
        buf.push_str(&format!("V{:X} = 0x{:02X}  ", reg, c8.gpregs[reg]));
    }
    buf.push_str("\n");
    buf.push_str(&format!(" I = 0x{:04X}\n\n", *c8.i_reg));

    let kb = &c8.keyboard;
    let st = |b| if b { "*" } else { " " };
    buf.push_str("KEYPAD:\n");
    buf.push_str(&format!("1{} 2{} 3{} C{}\n", st(kb[Key::K1]), st(kb[Key::K2]), st(kb[Key::K3]), st(kb[Key::KC])));
    buf.push_str(&format!("4{} 5{} 6{} D{}\n", st(kb[Key::K4]), st(kb[Key::K5]), st(kb[Key::K6]), st(kb[Key::KD])));
    buf.push_str(&format!("7{} 8{} 9{} E{}\n", st(kb[Key::K7]), st(kb[Key::K8]), st(kb[Key::K9]), st(kb[Key::KE])));
    buf.push_str(&format!("A{} B{} 0{} F{}\n\n", st(kb[Key::KA]), st(kb[Key::K0]), st(kb[Key::KB]), st(kb[Key::KF])));

    buf.push_str(&format!("TIMERS:\nDT = 0x{:02X}\nST = 0x{:02X}\n\n", c8.timers.delay(), c8.timers.sound()));
    //the extra spaces overwrite artifacts from the previous instruction
    //do not remove. Field width on the instruction puts weird spaces in the
    //structure.
    buf.push_str(&format!("PC:\n0x{:04X} -> {ins:?}                                 ", c8.pc));

    const SP_X: usize = 46;
    move_to(out, SP_X, 0);
    write!(out, "STACK:");
    for i in (0..STACK_LIMIT).rev() {
        move_to(out, SP_X, STACK_LIMIT - i + 1);
        if (c8.sp as usize) <= i {
            write!(out, "{:2}: 0x0000", i);
        } else {
            write!(out, "{:2}: 0x{:04X}", i, c8.stack[i]);
        }

        if c8.sp as usize == i {
            write!(out, " <- SP");
        } else {
            write!(out, "      ");
        }
    }
    
    move_to(out, 0, 0);
    writeln!(out, "{buf}");
}