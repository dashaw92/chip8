use std::ops::Deref;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct u12(u16);

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct u4(u8);

pub trait Nibbles<const N: usize> {
    fn nibble_at(&self, idx: usize) -> u4;
    fn nibbles(&self) -> [u8; N];
}

mod trait_impls {
    use super::*;

    impl u12 {
        pub fn from_nibbles(hi: u8, mid: u8, lo: u8) -> u12 {
            u12::of((hi as u16) << 8 | (mid as u16) << 4 | lo as u16)
        }

        pub fn of(val: u16) -> Self {
            Self(val & 0xFFF)
        }
    
        pub fn modify(&mut self, f: impl Fn(u16) -> u16) {
            self.0 = f(self.0) & 0xFFF;
        }
    }

    impl u4 {
        pub fn of(val: u8) -> Self {
            Self(val & 0xF)
        }

        pub fn modify(&mut self, f: impl Fn(u8) -> u8) {
            self.0 = f(self.0) & 0xF;
        }
    }

    impl Deref for u4 {
        type Target = u8;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl Deref for u12 {
        type Target = u16;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl Nibbles<2> for u8 {
        fn nibble_at(&self, idx: usize) -> u4 {
            assert!(idx < 2, "nibble idx > 1 for u8");
            u4::of(*self >> (idx * 4))
        }

        fn nibbles(&self) -> [u8; 2] {
            [*self.nibble_at(1), *self.nibble_at(0)]
        }
    }
    
    impl Nibbles<4> for u16 {
        fn nibble_at(&self, idx: usize) -> u4 {
            assert!(idx < 4, "nibble idx > 3 for u16");
            u4::of((*self >> (idx * 4)) as u8)
        }

        fn nibbles(&self) -> [u8; 4] {
            [*self.nibble_at(3), *self.nibble_at(2), *self.nibble_at(1), *self.nibble_at(0)]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::numtypes::Nibbles;

    #[test]
    fn nibblenibble() {
        let x16: u16 = 0xABCD;
        assert_eq!(0xD, *x16.nibble_at(0));
        assert_eq!(0xC, *x16.nibble_at(1));
        assert_eq!(0xB, *x16.nibble_at(2));
        assert_eq!(0xA, *x16.nibble_at(3));

        let x8: u8 = 0xEF;
        assert_eq!(0xF, *x8.nibble_at(0));
        assert_eq!(0xE, *x8.nibble_at(1));
    }
}