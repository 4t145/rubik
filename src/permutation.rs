// 实际上是四条对角线的置换群
#[repr(transparent)]
#[derive(PartialEq, Eq, Clone, Copy, Hash)]

/// an element of S4 group
pub struct CubePermutation(pub(crate) u8);

impl CubePermutation {
    /// new with check
    pub fn new(value: u8) -> Option<Self> {
        Self::check(value).then_some(Self(value))
    }

    pub const fn into_inner(self) -> u8 {
        self.0
    }

    /// check if the value is valid permutation
    pub const fn check(value: u8) -> bool {
        let p0 = value & 0b11;
        let p1 = value >> 2 & 0b11;
        let p2 = value >> 4 & 0b11;
        let p3 = value >> 6 & 0b11;
        !(p0 == p1 || p0 == p2 || p0 == p3 || p1 == p2 || p1 == p3 || p2 == p3)
    }

    /// new without valid check
    /// # Safety
    /// if every 2-bit segment is distinct, then value is a valid permutation
    pub const unsafe fn new_unchecked(value: u8) -> Self {
        Self(value)
    }

    /// the group binary operation, `a.compose(b)` means a+b
    pub const fn compose(self, p: Self) -> Self {
        Self(unsafe {
            self.get_unchecked(p.get_unchecked(0))
                | (self.get_unchecked(p.get_unchecked(1)) << 2)
                | (self.get_unchecked(p.get_unchecked(2)) << 4)
                | (self.get_unchecked(p.get_unchecked(3)) << 6)
        })
    }

    /// the inverse element of it
    pub const fn inverse(self) -> Self {
        Self(unsafe {
            0 << (self.get_unchecked(0) * 2)
                | 1 << (self.get_unchecked(1) * 2)
                | 2 << (self.get_unchecked(2) * 2)
                | 3 << (self.get_unchecked(3) * 2)
        })
    }

    /// # Safety
    /// index should less than 4
    pub const unsafe fn get_unchecked(&self, index: u8) -> u8 {
        (self.0 >> (index * 2)) & 0b11
    }

    pub fn get(&self, index: u8) -> u8 {
        if index < 4 {
            unsafe { self.get_unchecked(index) }
        } else {
            panic!("index out of bound")
        }
    }

    pub const fn square(self) -> Self {
        self.compose(self)
    }

    pub fn enumerate() -> impl Iterator<Item = Self> {
        [
            0b_11_10_01_00_u8,
            0b_10_11_01_00_u8,
            0b_11_01_10_00_u8,
            0b_01_11_10_00_u8,
            0b_01_10_11_00_u8,
            0b_10_01_11_00_u8,
        ]
        .iter()
        .flat_map(|i| {
            [
                (*i),
                (*i).rotate_left(2),
                (*i).rotate_left(4),
                (*i).rotate_left(6),
            ]
        })
        .map(Self)
    }

    const fn align(self) -> Self {
        let mut value = self.0;
        if value & 0b11 != 0b00 {
            value = value.rotate_left(2);
        }
        if value & 0b11 != 0b00 {
            value = value.rotate_left(2);
        }
        if value & 0b11 != 0b00 {
            value = value.rotate_left(2);
        }
        Self(value)
    }

    pub const fn factor_2(self) -> (Self, Self) {
        if self.0 & 0b11 == 0 {
            (Self::UNIT, self)
        } else if Self::X_2.compose(self).0 & 0b11 == 0 {
            (Self::X_2, Self::X_2.compose(self))
        } else if Self::Y_2.compose(self).0 & 0b11 == 0 {
            (Self::Y_2, Self::Y_2.compose(self))
        } else if Self::Z_2.compose(self).0 & 0b11 == 0 {
            (Self::Z_2, Self::Z_2.compose(self))
        } else {
            unreachable!()
        }
    }

    /// factor 0 group: {UNIT, X_2, Y_2, Z_2}
    /// 
    /// factor 1 group: {UNIT, C1, C2,}
    /// 
    /// factor 2 group: {UNIT, I}
    pub const fn factor_3(self) -> (Self, Self, Self) {
        let (f0, f1) = self.factor_2();
        const Q: [u8; 6] = [
            CubePermutation::UNIT.0,
            CubePermutation::I.0,
            CubePermutation::C1.compose(CubePermutation::I).0,
            CubePermutation::C2.compose(CubePermutation::I).0,
            CubePermutation::C1.0,
            CubePermutation::C2.0,
        ];
        if f1.0 == Q[0] {
            (f0, Self::UNIT, Self::UNIT)
        } else if f1.0 == Q[1] {
            (f0, Self::UNIT, Self::I)
        } else if f1.0 == Q[2] {
            (f0, Self::C1, Self::I)
        } else if f1.0 == Q[3] {
            (f0, Self::C2, Self::I)
        } else if f1.0 == Q[4] {
            (f0, Self::C1, Self::UNIT)
        } else if f1.0 == Q[5] {
            (f0, Self::C2, Self::UNIT)
        } else {
            unreachable!()
        }
    }
    //
    //   01-----10
    //  /   U   /|
    // 11-----00 |
    // |       |R|
    // |   F   | 11
    // |       |/
    // 10-----01
    //
    //

    pub const UNIT: Self = Self(0b_11_10_01_00);

    pub const X_1: Self = Self(0b_10_01_00_11);
    pub const X_2: Self = Self::X_1.square();
    pub const X_3: Self = Self::X_1.inverse();
    pub const Y_1: Self = Self(0b_10_00_11_01);
    pub const Y_2: Self = Self::Y_1.square();
    pub const Y_3: Self = Self::Y_1.inverse();
    pub const Z_1: Self = Self(0b_00_01_11_10);
    pub const Z_2: Self = Self::Z_1.square();
    pub const Z_3: Self = Self::Z_1.inverse();

    pub const FRONT: Self = Self::X_1;
    pub const BACK: Self = Self::X_3;
    pub const RIGHT: Self = Self::Y_1;
    pub const LEFT: Self = Self::Y_3;
    pub const UP: Self = Self::Z_1;
    pub const DOWN: Self = Self::Z_3;
    // factor 0 group: {UNIT, X_2, Y_2, Z_2}
    // factor 1 group: {UNIT, Y_1, Z_1, X_3, Y_3, UNIT}

    pub const C1: Self = Self(0b_01_11_10_00);
    pub const C2: Self = Self::C1.inverse();

    pub const I: Self = Self(0b_01_10_11_00);

    pub const fn unit() -> Self {
        Self::UNIT
    }
}

impl std::ops::Mul for CubePermutation {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        self.compose(rhs)
    }
}

impl std::fmt::Debug for CubePermutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("CubePermutation")
            .field(&[self.get(0), self.get(1), self.get(2), self.get(3)])
            .finish()
    }
}

impl Default for CubePermutation {
    fn default() -> Self {
        Self::UNIT
    }
}

pub struct Flip(CubePermutation);

impl Flip {
    pub const fn check(&self) -> bool {
        self.0 .0 == CubePermutation::X_2.0
            || self.0 .0 == CubePermutation::Y_2.0
            || self.0 .0 == CubePermutation::Z_2.0
            || self.0 .0 == CubePermutation::UNIT.0
    }
}
