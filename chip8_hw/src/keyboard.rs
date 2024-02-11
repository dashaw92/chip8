use std::{io::{Read, StdinLock}, ops::{Index, IndexMut}};

#[derive(Debug, Default)]
pub struct Keyboard {
    states: [bool; 0x10],
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

pub trait Input {
    fn wait_key(&mut self) -> Key;
}

impl Input for StdinLock<'_> {
    fn wait_key(&mut self) -> Key {
        let mut buf = [0x0; 1];
        loop {
            self.read_exact(&mut buf).expect("Failed to read input from stdin");
            return match buf[0] {
                b'1' => Key::K1,
                b'2' => Key::K2,
                b'3' => Key::K3,
                b'4' => Key::KC,
                b'q' => Key::K4,
                b'w' => Key::K5,
                b'e' => Key::K6,
                b'r' => Key::KD,
                b'a' => Key::K7,
                b's' => Key::K8,
                b'd' => Key::K9,
                b'f' => Key::KE,
                b'z' => Key::KA,
                b'x' => Key::K0,
                b'c' => Key::KB,
                b'v' => Key::KF,
                _ => continue,
            }
        }
    }
}