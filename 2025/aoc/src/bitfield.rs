use std::ops::{BitAnd, BitAndAssign, Index, Not};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BitField(u64);

impl BitField {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn one_bits(&self) -> u32 {
        let mut value = self.0;
        let mut count = 0;
        while value != 0 {
            count += 1;
            let index = value.trailing_zeros();
            value &= !(1 << index);
        }

        count
    }

    pub fn zero_bits(&self) -> u32 {
        u64::BITS - self.one_bits()
    }

    pub fn set(&self, index: usize, value: bool) -> BitField {
        if value {
            BitField(self.0 | (1 << index))
        } else {
            BitField(self.0 & !(1 << index))
        }
    }

    pub fn get(&self, index: usize) -> bool {
        self.0 & (1 << index) != 0
    }

    pub fn value(&self) -> u64 {
        self.0
    }

    pub fn test_all_bits(&self, bits: BitField) -> bool {
        *self & bits == bits
    }
}

impl PartialEq<u64> for BitField {
    fn eq(&self, other: &u64) -> bool {
        self.0 == *other
    }
}

impl From<u64> for BitField {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<BitField> for u64 {
    fn from(value: BitField) -> Self {
        value.value()
    }
}

impl Index<usize> for BitField {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        match self.get(index) {
            true => &true,
            false => &false,
        }
    }
}

impl BitAnd for BitField {
    type Output = BitField;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for BitField {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitAnd<u64> for BitField {
    type Output = BitField;

    fn bitand(self, rhs: u64) -> Self::Output {
        Self(self.0 & rhs)
    }
}

impl BitAndAssign<u64> for BitField {
    fn bitand_assign(&mut self, rhs: u64) {
        self.0 &= rhs
    }
}

impl Not for BitField {
    type Output = BitField;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::BitField;

    #[test]
    fn test_count() {
        let field = BitField::new(583);
        assert_eq!(5, field.one_bits());
        assert_eq!(59, field.zero_bits());
    }

    #[test]
    fn test_set() {
        let field = BitField::default();
        let field = field.set(0, true);
        assert_eq!(1, field.value());
        let field = field.set(10, true);
        assert_eq!(1025, field.value());
        assert!(field[10]);
        assert!(!field[9]);
        let field = field.set(10, false);
        assert_eq!(1, field.value());
        assert!(!field[10]);
        let field = field.set(10, false);
        assert_eq!(1, field.value());
    }
}
