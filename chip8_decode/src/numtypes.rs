use std::ops::Deref;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub(crate) struct u12(u16);

impl u12 {
    pub fn of(val: u16) -> Self {
        Self(val & 0xFFF)
    }

    pub fn modify(&mut self, f: impl Fn(u16) -> u16) {
        self.0 = f(self.0) & 0xFFF;
    }
}

impl Deref for u12 {
    type Target = u16;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub(crate) struct u4(u8);

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

trait Nibbles {
    fn nibble_at(&self, idx: usize) -> u4;
}

impl Nibbles for u8 {
    fn nibble_at(&self, idx: usize) -> u4 {
        assert!(idx < 2, "nibble idx > 1 for u8");
        u4::of(*self >> (idx * 4))
    }
}

impl Nibbles for u16 {
    fn nibble_at(&self, idx: usize) -> u4 {
        assert!(idx < 4, "nibble idx > 3 for u16");
        u4::of((*self >> (idx * 4)) as u8)
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