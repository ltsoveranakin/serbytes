use std::ops::{Add, Mul};

/// An estimate to the size of the result after serialization (in bits)

pub struct SizeHint {
    pub min: usize,
    pub max: usize,
}

impl SizeHint {
    pub const fn bytes(bytes: usize) -> Self {
        let bits = bytes * 8;

        Self::bits(bits)
    }

    pub const fn bits(bits: usize) -> Self {
        SizeHint {
            min: bits,
            max: bits,
        }
    }
}

impl Add for SizeHint {
    type Output = SizeHint;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            min: self.min + rhs.min,
            max: self.max + rhs.max,
        }
    }
}

impl Add<SizeHint> for usize {
    type Output = SizeHint;

    fn add(self, rhs: SizeHint) -> Self::Output {
        SizeHint {
            min: self + rhs.min,
            max: self + rhs.max,
        }
    }
}

impl Mul<SizeHint> for usize {
    type Output = SizeHint;

    fn mul(self, rhs: SizeHint) -> Self::Output {
        SizeHint {
            min: self * rhs.min,
            max: self * rhs.max,
        }
    }
}

impl Mul<usize> for SizeHint {
    type Output = SizeHint;

    fn mul(self, rhs: usize) -> Self::Output {
        rhs * self
    }
}
