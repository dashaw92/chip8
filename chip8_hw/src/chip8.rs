use std::time::Instant;

use chip8_decode::instructions::Instr;
use shared::{numtypes::u12, reg::GPReg};

use crate::keyboard::{Key, Keyboard};

pub const RAM_SIZE: usize = 0x1000;
pub const ROM_MAX_SIZE: usize = 0xE00;
pub const STACK_LIMIT: usize = 0x10;

pub const VRAM_WIDTH: usize = 64;
pub const VRAM_HEIGHT: usize = 32;
pub const VRAM_WH: usize = 64 * 32;

#[derive(Debug)]
pub struct Chip8 {
    pub ram: [u8; RAM_SIZE],
    pub gpregs: [u8; 0x10],
    pub i_reg: u12,
    pub pc: u16,
    pub sp: usize,
    pub stack: [u16; STACK_LIMIT],
    pub vram: [bool; VRAM_WH],
    pub keyboard: Keyboard,
    halted: bool,
    quirks: Quirks,
    pub timers: Timers,
}

#[derive(Debug)]
pub struct Timers {
    dt: u8,
    st: u8,
    last_tick: Instant,
}

impl Timers {
    const HZ_60: u128 = 1_000_000_000 / 60;

    fn tick(&mut self) {
        let elapsed = self.last_tick.elapsed();
        if elapsed.as_nanos() >= Timers::HZ_60 {
            if self.dt > 0 {
                self.dt -= 1;
            }

            if self.st > 0 {
                self.st -= 1;
            }

            self.last_tick = Instant::now();
        }
    }

    pub fn delay(&self) -> u8 {
        self.dt
    }

    pub fn sound(&self) -> u8 {
        self.st
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Quirks {
    /// VF is reset to 0 for AND, OR, and XOR opcodes
    pub vf_reset: bool,
    /// PUSHREG and POPREG modify the value of I
    pub memory: bool,
    pub shifting: bool,
}

#[allow(private_interfaces)]
pub static QUIRKS_OLD: Quirks = Quirks {
    vf_reset: true,
    memory: false,
    shifting: false,
};

#[allow(private_interfaces)]
pub static QUIRKS_NEW: Quirks = Quirks {
    vf_reset: true,
    memory: true,
    shifting: false,
};

static FONT: [[u8; 5]; 0x10] = [
    [0xF0, 0x90, 0x90, 0x90, 0xF0], //0
    [0x20, 0x60, 0x20, 0x20, 0x70], //1
    [0xF0, 0x10, 0xF0, 0x80, 0xF0], //2
    [0xF0, 0x10, 0xF0, 0x10, 0xF0], //3
    [0x90, 0x90, 0xF0, 0x10, 0x10], //4
    [0xF0, 0x80, 0xF0, 0x10, 0xF0], //5
    [0xF0, 0x80, 0xF0, 0x90, 0xF0], //6
    [0xF0, 0x10, 0x20, 0x40, 0x40], //7
    [0xF0, 0x90, 0xF0, 0x90, 0xF0], //8
    [0xF0, 0x90, 0xF0, 0x10, 0xF0], //9
    [0xF0, 0x90, 0xF0, 0x90, 0x90], //A
    [0xE0, 0x90, 0xE0, 0x90, 0xE0], //B
    [0xF0, 0x80, 0x80, 0x80, 0xF0], //C
    [0xE0, 0x90, 0x90, 0x90, 0xE0], //D
    [0xF0, 0x80, 0xF0, 0x80, 0xF0], //E
    [0xF0, 0x80, 0xF0, 0x80, 0x80], //F
];

impl Chip8 {
    fn copy_font(ram: &mut [u8]) {
        assert!(ram.len() <= 5 * 0x10);

        FONT.iter()
            .flat_map(IntoIterator::into_iter)
            .enumerate()
            .for_each(|(idx, &byte)| ram[idx] = byte);
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }

    pub fn set_halted(&mut self, halt: bool) {
        self.halted = halt;
    }

    pub fn load_rom(quirks: Quirks, rom: &[u8]) -> Self {
        assert!(rom.len() < ROM_MAX_SIZE, "ROM is too large! Must be at most {ROM_MAX_SIZE} bytes!");

        let mut c8 = Self {
            ram: [0x0; RAM_SIZE],
            gpregs: [0x0; 0x10],
            i_reg: u12::of(0x0),
            pc: 0x200,
            sp: 0x0,
            stack: [0x00; STACK_LIMIT],
            vram: [false; VRAM_WH],
            keyboard: Keyboard::default(),
            halted: false,
            quirks,
            timers: Timers {
                dt: 0,
                st: 0,
                last_tick: Instant::now(),
            }
        };

        Self::copy_font(&mut c8.ram[0..=0x4F]);

        for idx in 0..rom.len() {
            c8.ram[idx + 0x200] = rom[idx];
        }

        c8
    }

    pub fn step(&mut self, next_key: Option<Key>) -> Result<Instr, String> {
        let instr = {
            let (b1, b2) = (self.ram[self.pc as usize], self.ram[(self.pc + 1) as usize]);
            let bytes = (b1 as u16) << 8 | b2 as u16;
            Instr::decode(bytes)
        }.map_err(|e| format!("Failed to decode instruction: {e:#?}"))?;
        
        self.pc += 2;
        self.timers.tick();
        
        use chip8_decode::instructions::Instr::*;
        match instr {
            SYS(_) => {},
            CLS => self.vram.fill(false),
            RET => {
                if self.sp == 0 {
                    return Err(format!("Stack underflow! pc = 0x{:04X}", self.pc - 2));
                }

                self.stack[self.sp] = 0;
                self.sp -= 1;
                self.pc = self.stack[self.sp];
            },
            JP(addr) => {
                if self.pc - 2 == *addr {
                    self.halted = true;
                }
                self.pc = *addr;
            },
            CALL(addr) => {
                if self.sp as usize == STACK_LIMIT {
                    return Err(format!("Stack overflow! pc = 0x{:04X}", self.pc - 2));
                }

                self.stack[self.sp] = self.pc;
                self.sp += 1;
                self.pc = *addr;
            },
            SEQ(vx, lit) => {
                if self.gpregs[vx] == lit {
                    self.pc += 2;
                }
            },
            SNELIT(vx, lit) => {
                if self.gpregs[vx] != lit {
                    self.pc += 2;
                }
            },
            SE(vx, vy) => {
                if self.gpregs[vx] == self.gpregs[vy] {
                    self.pc += 2;
                }
            },
            LDL(vx, lit) => self.gpregs[vx] = lit,
            ADDL(vx, lit) => {
                let vx_val = self.gpregs[vx];
                self.gpregs[vx] = vx_val.overflowing_add(lit).0;
            },
            LD(vx, vy) => self.gpregs[vx] = self.gpregs[vy],
            OR(vx, vy) => {
                self.gpregs[vx] = self.gpregs[vx] | self.gpregs[vy];
                if self.quirks.vf_reset {
                    self.gpregs[GPReg::VF] = 0;
                }
            },
            AND(vx, vy) => {
                self.gpregs[vx] = self.gpregs[vx] & self.gpregs[vy];
                if self.quirks.vf_reset {
                    self.gpregs[GPReg::VF] = 0;
                }
            },
            XOR(vx, vy) => {
                self.gpregs[vx] = self.gpregs[vx] ^ self.gpregs[vy];
                if self.quirks.vf_reset {
                    self.gpregs[GPReg::VF] = 0;
                }
            },
            ADDC(vx, vy) => {
                let (out, carry) = self.gpregs[vx].overflowing_add(self.gpregs[vy]);
                self.gpregs[vx] = out;
                self.gpregs[GPReg::VF] = carry as u8;
            },
            SUBC(vx, vy) => {
                let (out, carry) = self.gpregs[vx].overflowing_sub(self.gpregs[vy]);
                self.gpregs[vx] = out;
                self.gpregs[GPReg::VF] = !carry as u8;
            },
            SHRC(vx, vy) => {
                let vy_val = self.gpregs[vy];
                let vx_val = self.gpregs[vx];

                let mut val = vy_val;
                if self.quirks.shifting {
                    val = vx_val;
                }

                self.gpregs[vx] = val >> 1;
                self.gpregs[GPReg::VF] = (vx_val & 0x1 == 1) as u8;
            },
            SUBN(vx, vy) => {
                let (out, carry) = self.gpregs[vy].overflowing_sub(self.gpregs[vx]);
                self.gpregs[vx] = out;
                self.gpregs[GPReg::VF] = !carry as u8;
            },
            SHLC(vx, vy) => {
                let vy_val = self.gpregs[vy];
                let vx_val = self.gpregs[vx];

                let mut val = vy_val;
                if self.quirks.shifting {
                    val = vx_val;
                }

                self.gpregs[vx] = val << 1;
                self.gpregs[GPReg::VF] = (vx_val & 0b10000000 == 0b10000000) as u8;
            },
            SNE(vx, vy) => {
                if self.gpregs[vx] != self.gpregs[vy] {
                    self.pc += 2;
                }
            },
            LDI(addr) => self.i_reg = addr,
            JPL(addr) => self.pc = self.gpregs[GPReg::V0] as u16 + *addr,
            RND(vx, byte) => {
                let rng = rand::random::<u8>() & byte;
                self.gpregs[vx] = rng;
            },
            DRW(vx, vy, size) => {
                self.gpregs[GPReg::VF] = 0;

                let x_start = self.gpregs[vx] as usize & 63;
                let y_start = self.gpregs[vy] as usize & 31;

                let spr = &self.ram[*self.i_reg as usize ..= (*self.i_reg + *size as u16) as usize];
                for y in 0 .. *size {
                    let byte = spr[y as usize];
                    for x in 0 ..= 7 {
                        let mask = 0b10000000 >> x;
                        let bit = byte & mask == mask;

                        let y_coord = y_start + y as usize;
                        let x_coord = x_start + x as usize;

                        if y_coord >= VRAM_HEIGHT || x_coord >= VRAM_WIDTH {
                            continue;
                        }

                        let idx = y_coord * VRAM_WIDTH + x_coord;
                        if self.vram[idx] {
                            self.gpregs[GPReg::VF] = 1;
                        }

                        self.vram[idx] ^= bit;
                    }
                }
            },
            SKP(vx) => {
                let key = Key::try_from(self.gpregs[vx]).map_err(|_| format!("Invalid key idx {}. pc = 0x{:4X}", self.gpregs[vx], self.pc))?;
                if self.keyboard[key] {
                    self.pc += 2;
                }
            },
            SKNP(vx) => {
                let key = Key::try_from(self.gpregs[vx]).map_err(|_| format!("Invalid key idx {}. pc = 0x{:4X}", self.gpregs[vx], self.pc))?;
                if !self.keyboard[key] {
                    self.pc += 2;
                }
            },
            MOVDT(vx) => self.gpregs[vx] = self.timers.dt,
            LDKB(vx) => {
                let Some(key) = next_key else {
                    self.pc -= 2;
                    return Ok(instr);
                };
                self.gpregs[vx] = key as u8;
            },
            LDDT(vx) => self.timers.dt = self.gpregs[vx],
            LDST(vx) => self.timers.st = self.gpregs[vx],
            ADDI(vx) => self.i_reg.modify(|i| i + self.gpregs[vx] as u16),
            LDSPR(vx) => self.i_reg.modify(|_| self.gpregs[vx] as u16 * 5),
            LDBCD(vx) => {
                let mut vx_val = self.gpregs[vx];
                let ones = vx_val % 10;
                vx_val /= 10;
                let tens = vx_val % 10;
                vx_val /= 10;
                let hund = vx_val % 10;

                self.ram[*self.i_reg as usize + 0] = hund;
                self.ram[*self.i_reg as usize + 1] = tens;
                self.ram[*self.i_reg as usize + 2] = ones;
            },
            PUSHREG(vx) => {
                for (i, addr) in (*self.i_reg ..= *self.i_reg + vx.to_idx() as u16).enumerate() {
                    let reg = GPReg::indexed(i as u8).ok_or(format!("Invalid GPReg {}", i))?;
                    self.ram[addr as usize] = self.gpregs[reg];
                }

                if self.quirks.memory {
                    self.i_reg.modify(|i| i + vx.to_idx() as u16 + 1);
                }
            },
            POPREG(vx) => {
                for (i, addr) in (*self.i_reg ..= *self.i_reg + vx.to_idx() as u16).enumerate() {
                    let reg = GPReg::indexed(i as u8).ok_or(format!("Invalid GPReg {}", i))?;
                    self.gpregs[reg] = self.ram[addr as usize];
                }

                if self.quirks.memory {
                    self.i_reg.modify(|i| i + vx.to_idx() as u16 + 1);
                }
            },
        }

        Ok(instr)
    }
}