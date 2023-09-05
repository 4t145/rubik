// 实际上是四条对角线的置换群
#[repr(transparent)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct CubePermutation(pub(crate) u8);

impl CubePermutation {
    pub fn new(value: u8) -> Option<Self> {
        Self::check(value).then_some(Self(value))
    }
    
    pub const fn into_inner(self) -> u8 {
        self.0
    }

    pub fn check(value: u8) -> bool {
        let p0 = value & 0b11;
        let p1 = value >> 2 & 0b11;
        let p2 = value >> 4 & 0b11;
        let p3 = value >> 6 & 0b11;
        !(p0 == p1 || p0 == p2 || p0 == p3 || p1 == p2 || p1 == p3 || p2 == p3)
    }

    /// # Safety
    /// if every 2-bit segment is distinct, then value is a valid permutation
    pub const unsafe fn new_unchecked(value: u8) -> Self {
        Self(value)
    }

    // P: replace original S ith element with S[P[i]]
    pub const fn compose(self, p: Self) -> Self {
        Self(unsafe {
            self.get_unchecked(p.get_unchecked(0))
                | (self.get_unchecked(p.get_unchecked(1)) << 2)
                | (self.get_unchecked(p.get_unchecked(2)) << 4)
                | (self.get_unchecked(p.get_unchecked(3)) << 6)
        })
    }

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
    pub const X_2: Self = Self::X_1.compose(Self::X_1);
    pub const X_3: Self = Self::X_1.inverse();
    pub const Y_1: Self = Self(0b_10_00_11_01);
    pub const Y_2: Self = Self::Y_1.compose(Self::Y_1);
    pub const Y_3: Self = Self::Y_1.inverse();
    pub const Z_1: Self = Self(0b_00_01_11_10);
    pub const Z_2: Self = Self::Z_1.compose(Self::Z_1);
    pub const Z_3: Self = Self::Z_1.inverse();

    pub const FRONT: Self = Self::X_1;
    pub const BACK: Self = Self::X_3;
    pub const RIGHT: Self = Self::Y_1;
    pub const LEFT: Self = Self::Y_3;
    pub const UP: Self = Self::Z_1;
    pub const DOWN: Self = Self::Z_3;

    pub const fn unit() -> Self {
        Self::UNIT
    }
}

impl std::ops::Add for CubePermutation {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
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
