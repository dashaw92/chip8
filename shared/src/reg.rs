use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum GPReg {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    VA,
    VB,
    VC,
    VD,
    VE,
    VF,
}

impl GPReg {
    pub fn indexed(reg: u8) -> Option<GPReg> {
        Some(match reg {
            0x0 => Self::V0,
            0x1 => Self::V1,
            0x2 => Self::V2,
            0x3 => Self::V3,
            0x4 => Self::V4,
            0x5 => Self::V5,
            0x6 => Self::V6,
            0x7 => Self::V7,
            0x8 => Self::V8,
            0x9 => Self::V9,
            0xA => Self::VA,
            0xB => Self::VB,
            0xC => Self::VC,
            0xD => Self::VD,
            0xE => Self::VE,
            0xF => Self::VF,
            _ => return None,
        })
    }

    pub fn to_idx(&self) -> usize {
        match self {
            Self::V0 => 0x0,
            Self::V1 => 0x1,
            Self::V2 => 0x2,
            Self::V3 => 0x3,
            Self::V4 => 0x4,
            Self::V5 => 0x5,
            Self::V6 => 0x6,
            Self::V7 => 0x7,
            Self::V8 => 0x8,
            Self::V9 => 0x9,
            Self::VA => 0xA,
            Self::VB => 0xB,
            Self::VC => 0xC,
            Self::VD => 0xD,
            Self::VE => 0xE,
            Self::VF => 0xF,
        }
    }
}

impl<const N: usize> Index<GPReg> for [u8; N] {
    type Output = u8;

    fn index(&self, index: GPReg) -> &Self::Output {
        &self[index.to_idx()]
    }
}

impl<const N: usize> IndexMut<GPReg> for [u8; N] {
    fn index_mut(&mut self, index: GPReg) -> &mut Self::Output {
        &mut self[index.to_idx()]
    }
} 