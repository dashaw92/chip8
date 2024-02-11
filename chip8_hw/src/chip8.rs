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
    pub dt: u8,
    pub st: u8,
    pub pc: u16,
    pub sp: u8,
    pub stack: Vec<u16>,
    pub vram: [bool; VRAM_WH],
    pub keyboard: Keyboard,
}

static FONT: [[u8; 5]; 0x10] = [
    [0xF0, 0x90, 0x90, 0x90, 0xF0], //0
    [0x20, 0x60, 0x20, 0x20, 0x70], //1
    [0xF0, 0x10, 0xF0, 0x80, 0xF0], //2
    [0xF0, 0x10, 0xF0, 0x10, 0xF0], //3
    [0x90, 0x90, 0xF0, 0x10, 0x10], //4
    [0xF0, 0x80, 0xF0, 0x10, 0xF0], //5
    [0xF0, 0x80, 0xF0, 0x90, 0xF0], //6
    [0xF0, 0x10, 0x20, 0x40, 0xF0], //7
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

    pub fn load_rom(rom: &[u8]) -> Self {
        assert!(rom.len() < ROM_MAX_SIZE, "ROM is too large! Must be at most {ROM_MAX_SIZE} bytes!");

        let mut c8 = Self {
            ram: [0x0; RAM_SIZE],
            gpregs: [0x0; 0x10],
            i_reg: u12::of(0x0),
            dt: 0x0,
            st: 0x0,
            pc: 0x200,
            sp: 0x0,
            stack: vec![0x00; STACK_LIMIT],
            vram: [false; VRAM_WH],
            keyboard: Keyboard::default(),
        };

        Self::copy_font(&mut c8.ram[0..=0x4F]);

        for idx in 0..rom.len() {
            c8.ram[idx + 0x200] = rom[idx];
        }

        c8
    }

    pub fn step(&mut self) -> Result<Instr, String> {
        
        let instr = {
            let (b1, b2) = (self.ram[self.pc as usize], self.ram[(self.pc + 1) as usize]);
            let bytes = (b1 << 8) as u16 | b2 as u16;
            Instr::decode(bytes)
        }.map_err(|e| format!("Failed to decode instruction: {e:#?}"))?;
        
        self.pc += 2;
        
        use chip8_decode::instructions::Instr::*;
        match instr {
            SYS(_) => {},
            CLS => self.vram.fill(false),
            RET => {
                if self.sp == 0 {
                    return Err(format!("Stack underflow! pc = 0x{:4X}", self.pc - 2));
                }

                self.sp -= 1;
                self.pc = self.stack.pop().expect("Failed to pop stack when sp != 0; sp out of sync with stack?");
            },
            JP(addr) => self.pc = *addr,
            CALL(addr) => {
                if self.sp as usize == STACK_LIMIT {
                    return Err(format!("Stack overflow! pc = 0x{:4X}", self.pc - 2));
                }

                self.stack.push(self.pc);
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
                self.gpregs[vx] = vx_val + lit;
            },
            LD(vx, vy) => self.gpregs[vx] = self.gpregs[vy],
            OR(vx, vy) => self.gpregs[vx] = self.gpregs[vx] | self.gpregs[vy],
            AND(vx, vy) => self.gpregs[vx] = self.gpregs[vx] & self.gpregs[vy],
            XOR(vx, vy) => self.gpregs[vx] = self.gpregs[vx] ^ self.gpregs[vy],
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
            SHRC(vx, _) => {
                self.gpregs[GPReg::VF] = (self.gpregs[vx] & 0x1 == 1) as u8;
                self.gpregs[vx] >>= 1;
            },
            SUBN(vx, vy) => {
                let (out, carry) = self.gpregs[vy].overflowing_sub(self.gpregs[vx]);
                self.gpregs[vx] = out;
                self.gpregs[GPReg::VF] = !carry as u8;
            },
            SHLC(vx, _) => {
                self.gpregs[GPReg::VF] = (self.gpregs[vx] & 0b10000000 == 1) as u8;
                self.gpregs[vx] <<= 1;
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
            DRW(vx, vy, size) => todo!(),
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
            MOVDT(vx) => self.gpregs[vx] = self.dt,
            LDKB(_) => todo!(),
            LDDT(_) => todo!(),
            LDST(_) => todo!(),
            ADDI(_) => todo!(),
            LDSPR(_) => todo!(),
            LDBCD(_) => todo!(),
            PUSHREG(_) => todo!(),
            POPREG(_) => todo!(),
        }

        Ok(instr)
    }
}