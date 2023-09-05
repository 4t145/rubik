#![allow(clippy::unusual_byte_groupings)]

use std::fmt::Debug;

mod colored;
pub use colored::*;
pub mod operation;
// 实际上是四条对角线的置换群
#[repr(transparent)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct CubePermutation(u8);

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

    const fn segments(self) -> [u8; 4] {
        let value = self.0;
        let p0 = value & 0b11;
        let p1 = value >> 2 & 0b11;
        let p2 = value >> 4 & 0b11;
        let p3 = value >> 6 & 0b11;
        [p0, p1, p2, p3]
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

impl Debug for CubePermutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("CubeRotation")
            .field(&self.segments())
            .finish()
    }
}

impl Default for CubePermutation {
    fn default() -> Self {
        Self::UNIT
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Cube {
    pub rotation: CubePermutation,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CubeFace {
    F = unsafe { Self::align(CubePermutation::UNIT.0) },
    B = unsafe { Self::align(CubePermutation::Y_2.0) },
    R = unsafe { Self::align(CubePermutation::Z_1.0) },
    L = unsafe { Self::align(CubePermutation::Z_3.0) },
    U = unsafe { Self::align(CubePermutation::Y_3.0) },
    D = unsafe { Self::align(CubePermutation::Y_1.0) },
}

impl CubeFace {
    const unsafe fn align(value: u8) -> u8 {
        let mut value = value;
        if value & 0b11 != 0b00 {
            value = value.rotate_left(2);
        }
        if value & 0b11 != 0b00 {
            value = value.rotate_left(2);
        }
        if value & 0b11 != 0b00 {
            value = value.rotate_left(2);
        }
        value
    }
    const fn from_cube_permutation(value: CubePermutation) -> CubeFace {
        unsafe { std::mem::transmute(Self::align(value.0)) }
    }
    const fn as_cube_permutation(self) -> CubePermutation {
        unsafe { CubePermutation::new_unchecked(self as u8) }
    }
}

impl Cube {
    pub const fn new() -> Self {
        Self {
            rotation: CubePermutation::UNIT,
        }
    }
    pub const fn new_with_rotation(rotation: CubePermutation) -> Self {
        Self { rotation }
    }
    pub fn rotate(&mut self, rotation: CubePermutation) -> &mut Self {
        self.rotation = self.rotation.compose(rotation);
        self
    }
    pub const fn get(&self, face: CubeFace) -> CubeFace {
        CubeFace::from_cube_permutation(self.rotation.compose(face.as_cube_permutation()))
    }
}

impl Debug for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cube")
            .field("rotation", &self.rotation)
            .field("front", &self.get(CubeFace::F))
            .field("top", &self.get(CubeFace::U))
            .field("down", &self.get(CubeFace::D))
            .field("back", &self.get(CubeFace::B))
            .field("left", &self.get(CubeFace::L))
            .field("right", &self.get(CubeFace::R))
            .finish()
    }
}

pub struct RubikLayerTransform {
    // cude indexes assuming rotaion is clockwise
    cude_indexes: [u8; 9],
    rotation: CubePermutation,
}

unsafe fn ptr_rotate_1<T>(values: [*mut T; 4]) {
    std::ptr::swap(values[0], values[1]);
    std::ptr::swap(values[0], values[2]);
    std::ptr::swap(values[0], values[3]);
}

unsafe fn ptr_rotate_2<T>(values: [*mut T; 4]) {
    std::ptr::swap(values[0], values[2]);
    std::ptr::swap(values[1], values[3]);
}

unsafe fn ptr_rotate_3<T>(values: [*mut T; 4]) {
    std::ptr::swap(values[3], values[2]);
    std::ptr::swap(values[3], values[1]);
    std::ptr::swap(values[3], values[0]);
}

pub struct RubikLayerIter<'r> {
    layer: &'static RubikLayerTransform,
    rubik: &'r Rubik,
    index: usize,
}

impl<'r> Iterator for RubikLayerIter<'r> {
    type Item = &'r Cube;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.layer.cude_indexes.len() {
            let cude = &self.rubik.cubes[self.layer.cude_indexes[self.index] as usize];
            self.index += 1;
            Some(cude)
        } else {
            None
        }
    }
}
/*
            UU_UU_UU
            UU_UU_UU
            UU_UU_UU

LL_LL_LL    FF_FF_FF    RR_RR_RR    BB_BB_BB
LL_LL_LL    FF_FF_FF    RR_RR_RR    BB_BB_BB
LL_LL_LL    FF_FF_FF    RR_RR_RR    BB_BB_BB

            DD_DD_DD
            DD_DD_DD
            DD_DD_DD


            18_19_20
            09_10_11
            00_01_02

18_09_00    00_01_02    02_11_20    20_19_18
21_12_03    03_04_05    05_14_23    23_22_21
24_15_06    06_07_08    08_17_26    26_25_24

            06_07_08
            15_16_17
            24_25_26
*/
#[allow(clippy::zero_prefixed_literal)]
impl RubikLayerTransform {
    pub fn rotate1(&self, rubik: &mut Rubik) {
        for index in self.cude_indexes {
            rubik.cubes[index as usize].rotate(self.rotation);
        }
        unsafe {
            let indicies = &self.cude_indexes;
            ptr_rotate_1([
                rubik.ptr_of(indicies[0]),
                rubik.ptr_of(indicies[2]),
                rubik.ptr_of(indicies[8]),
                rubik.ptr_of(indicies[6]),
            ]);
            ptr_rotate_1([
                rubik.ptr_of(indicies[1]),
                rubik.ptr_of(indicies[5]),
                rubik.ptr_of(indicies[7]),
                rubik.ptr_of(indicies[3]),
            ])
        }
    }
    pub const fn inverse(self) -> Self {
        Self {
            cude_indexes: [
                self.cude_indexes[0],
                self.cude_indexes[3],
                self.cude_indexes[6],
                self.cude_indexes[1],
                self.cude_indexes[4],
                self.cude_indexes[7],
                self.cude_indexes[2],
                self.cude_indexes[5],
                self.cude_indexes[8],
            ],
            rotation: self.rotation.inverse(),
        }
    }
    pub const F: Self = Self {
        cude_indexes: [00, 01, 02, 03, 04, 05, 06, 07, 08],
        rotation: CubePermutation::FRONT,
    };
    pub const B: Self = Self {
        cude_indexes: [20, 19, 18, 23, 22, 21, 26, 25, 24],
        rotation: CubePermutation::BACK,
    };
    pub const L: Self = Self {
        cude_indexes: [18, 09, 00, 21, 12, 03, 24, 15, 06],
        rotation: CubePermutation::LEFT,
    };
    pub const R: Self = Self {
        cude_indexes: [02, 11, 20, 05, 14, 23, 08, 17, 26],
        rotation: CubePermutation::RIGHT,
    };
    pub const U: Self = Self {
        cude_indexes: [18, 19, 20, 09, 10, 11, 00, 01, 02],
        rotation: CubePermutation::UP,
    };
    pub const D: Self = Self {
        cude_indexes: [06, 07, 08, 15, 16, 17, 24, 25, 26],
        rotation: CubePermutation::DOWN,
    };

    pub const F_: Self = Self::F.inverse();
    pub const B_: Self = Self::B.inverse();
    pub const L_: Self = Self::L.inverse();
    pub const R_: Self = Self::R.inverse();
    pub const U_: Self = Self::U.inverse();
    pub const D_: Self = Self::D.inverse();
}

pub struct Rubik {
    cubes: [Cube; 27],
}
pub trait RubikTransform {
    fn apply_on(&self, rubik: &mut Rubik);
}


impl RubikTransform for RubikLayerTransform {
    fn apply_on(&self, rubik: &mut Rubik) {
        self.rotate1(rubik);
    }
}

pub struct RubikTransformGroup<'a> {
    inner: Vec<&'a dyn RubikTransform>,
}

impl RubikTransform for RubikTransformGroup<'_> {
    fn apply_on(&self, rubik: &mut Rubik) {
        rubik.execute(self.inner.iter().copied());
    }
}
impl Rubik {
    pub const fn new() -> Self {
        Self {
            cubes: [Cube::new(); 27],
        }
    }
    unsafe fn ptr_of(&mut self, idx: u8) -> *mut Cube {
        self.cubes.as_mut_ptr().add(idx as usize)
    }
    pub fn execute<'a>(
        &mut self,
        operations: impl IntoIterator<Item = &'a dyn RubikTransform>,
    ) -> &mut Self {
        for op in operations {
            op.apply_on(self);
        }
        self
    }
    pub fn execute_one(&mut self, operation: &impl RubikTransform) -> &mut Self {
        operation.apply_on(self);
        self
    }

    pub fn iter_by_layer(&self, layer: &'static RubikLayerTransform) -> RubikLayerIter<'_> {
        RubikLayerIter {
            layer,
            rubik: self,
            index: 0,
        }
    }
}
