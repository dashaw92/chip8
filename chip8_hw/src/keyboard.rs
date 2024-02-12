use std::ops::{Index, IndexMut};

#[derive(Debug, Default)]
pub struct Keyboard {
    states: [bool; 0x10],
}
impl Keyboard {
    pub(crate) fn key_pressed(&self) -> Option<Key> {
        (0..self.states.len())
            .find(|&pressed| self.states[pressed])
            .map(|key| Key::try_from(key as u8).unwrap())
    }
}

#[derive(PartialOrd, Ord, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Key {
    K1,
    K2,
    K3,
    KC,
    K4,
    K5,
    K6,
    KD,
    K7,
    K8,
    K9,
    KE,
    KA,
    K0,
    KB,
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
            0x0 => Key::K1,
            0x1 => Key::K2,
            0x2 => Key::K3,
            0x3 => Key::KC,
            0x4 => Key::K4,
            0x5 => Key::K5,
            0x6 => Key::K6,
            0x7 => Key::KD,
            0x8 => Key::K7,
            0x9 => Key::K8,
            0xA => Key::K9,
            0xB => Key::KE,
            0xC => Key::KA,
            0xD => Key::K0,
            0xE => Key::KB,
            0xF => Key::KF,
            _ => return Err(())
        })
    }
}