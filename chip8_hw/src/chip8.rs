use shared::numtypes::u12;

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
    pub stack: [u16; STACK_LIMIT],
    pub vram: [bool; VRAM_WH],
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
            stack: [0x00; STACK_LIMIT],
            vram: [false; VRAM_WH],
        };

        Self::copy_font(&mut c8.ram[0..=0x4F]);

        for idx in 0..rom.len() {
            c8.ram[idx + 0x200] = rom[idx];
        }

        c8
    }
}