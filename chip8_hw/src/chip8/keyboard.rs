use std::ops::{Index, IndexMut};

#[derive(Debug, Default)]
pub struct Keyboard {
    states: [bool; 0x10],
}

#[derive(PartialOrd, Ord, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Key {
    K0,
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
    K8,
    K9,
    KA,
    KB,
    KC,
    KD,
    KE,
    KF,
}

impl Index<Key> for Keyboard {
    type Output = bool;

    fn index(&self, index: Key) -> &Self::Output {
        &self.states[index as usize]
    }
}

impl IndexMut<Key> for Keyboard {
    fn index_mut(&mut self, index: Key) -> &mut Self::Output {
        &mut self.states[index as usize]
    }
}

impl TryFrom<u8> for Key {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0x0 => Key::K0,
            0x1 => Key::K1,
            0x2 => Key::K2,
            0x3 => Key::K3,
            0x4 => Key::K4,
            0x5 => Key::K5,
            0x6 => Key::K6,
            0x7 => Key::K7,
            0x8 => Key::K8,
            0x9 => Key::K9,
            0xA => Key::KA,
            0xB => Key::KB,
            0xC => Key::KC,
            0xD => Key::KD,
            0xE => Key::KE,
            0xF => Key::KF,
            _ => return Err(())
        })
    }
}