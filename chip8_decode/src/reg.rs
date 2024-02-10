#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub(crate) enum GPReg {
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
    pub(crate) fn indexed(reg: u8) -> Option<GPReg> {
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
}