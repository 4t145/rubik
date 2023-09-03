#![allow(clippy::unusual_byte_groupings)]

use std::{default, fmt::Debug, ops::Index};

fn main() {
    println!("Hello, world!");
}

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
        let output = unsafe {
            self.get_unchecked(p.get_unchecked(0))
                | (self.get_unchecked(p.get_unchecked(1)) << 2)
                | (self.get_unchecked(p.get_unchecked(2)) << 4)
                | (self.get_unchecked(p.get_unchecked(3)) << 6)
        };
        Self(output)
    }

    pub const fn unit() -> Self {
        Self::UNIT
    }

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
    pub const UP: Self = Self(0b_10_00_11_01);
    pub const DOWN: Self = Self(0b_01_11_00_10);
    pub const LEFT: Self = Self(0b_00_01_11_10);
    pub const RIGHT: Self = Self(0b_01_00_10_11);
    pub const FRONT: Self = Self(0b_10_01_00_11);
    pub const BACK: Self = Self(0b_00_11_10_01);
    pub const UNIT: Self = Self(0b_11_10_01_00);
}

impl std::ops::Add for CubePermutation {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.compose(rhs)
    }
}

impl Debug for CubePermutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("CudeRotation")
            .field(&self.segments())
            .finish()
    }
}

impl Default for CubePermutation {
    fn default() -> Self {
        Self::unit()
    }
}

#[test]
fn test_compose() {
    let unit = CubePermutation::new(0b11100100).unwrap();
    assert_eq!(unit.compose(unit), unit);
    let a = CubePermutation::new(0b10010011).unwrap();
    assert_eq!(a + unit, a);
    assert_eq!(unit + a, a);
    assert_eq!(
        CubePermutation::BACK + CubePermutation::FRONT,
        CubePermutation::UNIT
    );
    assert_eq!(
        CubePermutation::UP + CubePermutation::DOWN,
        CubePermutation::UNIT
    );
    assert_eq!(
        CubePermutation::RIGHT + CubePermutation::LEFT,
        CubePermutation::UNIT
    );

    assert_eq!(
        CubePermutation::UP
            + CubePermutation::UP
            + CubePermutation::RIGHT
            + CubePermutation::RIGHT
            + CubePermutation::FRONT
            + CubePermutation::FRONT,
        CubePermutation::UNIT
    );
    assert_eq!(
        CubePermutation::UP + CubePermutation::RIGHT,
        CubePermutation::BACK + CubePermutation::UP
    );
}

#[derive(Clone, Copy)]
pub struct CubeFaceMap<T> {
    pub f: T,
    pub r: T,
    pub u: T,
    pub b: T,
    pub l: T,
    pub d: T,
}

impl<T> CubeFaceMap<T> {
    pub const fn new(f: T, r: T, u: T, b: T, l: T, d: T) -> Self {
        Self { f, r, u, b, l, d }
    }
    pub const fn get(&self, face: CubeFace) -> &T {
        match face {
            CubeFace::F => &self.f,
            CubeFace::B => &self.b,
            CubeFace::R => &self.r,
            CubeFace::L => &self.l,
            CubeFace::U => &self.u,
            CubeFace::D => &self.d,
        }
    }
}

impl<T> std::ops::Index<CubeFace> for CubeFaceMap<T> {
    type Output = T;
    fn index(&self, index: CubeFace) -> &Self::Output {
        self.get(index)
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Cube {
    pub rotation: CubePermutation,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum CubeFace {
    F = 0b_11_10_01_00,
    B = 0b_01_10_11_00,
    R = 0b_01_11_10_00,
    L = 0b_10_11_01_00,
    U = 0b_10_01_11_00,
    D = 0b_11_01_10_00,
}

impl CubeFace {
    pub fn from_cude_permutation(value: CubePermutation) -> CubeFace {
        let mut value = value.0;
        while value & 0b11 != 0b00 {
            value = value.rotate_left(2);
        }
        unsafe { std::mem::transmute(value) }
    }
}

impl Cube {
    pub const fn new() -> Self {
        Self {
            rotation: CubePermutation::unit(),
        }
    }
    pub fn rotate(&mut self, rotation: CubePermutation) -> &mut Self {
        self.rotation = self.rotation.compose(rotation);
        self
    }
    fn get_front(&self) -> CubeFace {
        CubeFace::from_cude_permutation(self.rotation)
    }
    fn get_top(&self) -> CubeFace {
        CubeFace::from_cude_permutation(self.rotation + CubePermutation::DOWN)
    }
    fn get_down(&self) -> CubeFace {
        CubeFace::from_cude_permutation(self.rotation + CubePermutation::UP)
    }
    fn get_back(&self) -> CubeFace {
        CubeFace::from_cude_permutation(
            self.rotation + CubePermutation::UP.compose(CubePermutation::UP),
        )
    }
    fn get_left(&self) -> CubeFace {
        CubeFace::from_cude_permutation(self.rotation + CubePermutation::RIGHT)
    }
    fn get_right(&self) -> CubeFace {
        CubeFace::from_cude_permutation(self.rotation + CubePermutation::LEFT)
    }
    fn get(&self, face: CubeFace) -> CubeFace {
        match face {
            CubeFace::F => self.get_front(),
            CubeFace::B => self.get_back(),
            CubeFace::R => self.get_right(),
            CubeFace::L => self.get_left(),
            CubeFace::U => self.get_top(),
            CubeFace::D => self.get_down(),
        }
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

#[test]
pub fn test_cube() {
    let mut cube = Cube::new();
    dbg!(&cube);
    cube.rotate(CubePermutation::UP);
    dbg!(&cube);
    cube.rotate(CubePermutation::UP);
    dbg!(&cube);
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RubikColor {
    White,
    Yellow,
    Red,
    Orange,
    Blue,
    Green,
}

pub struct RubikLayer {
    cude_indecies: [u8; 9],
    rotation: CubePermutation,
}

unsafe fn ptr_rotate_n<T, const N: usize>(values: [*mut T; N]) {
    for idx in 1..N {
        std::ptr::swap(values[0], values[idx])
    }
}

pub struct RubikLayerIter<'r> {
    layer: &'static RubikLayer,
    rubik: &'r Rubik,
    index: usize,
}

impl<'r> Iterator for RubikLayerIter<'r> {
    type Item = &'r Cube;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.layer.cude_indecies.len() {
            let cude = &self.rubik.cubes[self.layer.cude_indecies[self.index] as usize];
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
impl RubikLayer {
    pub fn rotate(&self, rubik: &mut Rubik) {
        for index in self.cude_indecies {
            rubik.cubes[index as usize].rotate(self.rotation);
        }
        unsafe {
            let indicies = &self.cude_indecies;
            ptr_rotate_n([
                rubik.ptr_of(indicies[0]),
                rubik.ptr_of(indicies[2]),
                rubik.ptr_of(indicies[8]),
                rubik.ptr_of(indicies[6]),
            ]);
            ptr_rotate_n([
                rubik.ptr_of(indicies[1]),
                rubik.ptr_of(indicies[5]),
                rubik.ptr_of(indicies[8]),
                rubik.ptr_of(indicies[3]),
            ])
        }
    }

    pub const F: Self = Self {
        cude_indecies: [00, 01, 02, 03, 04, 05, 06, 07, 08],
        rotation: CubePermutation::FRONT,
    };
    pub const B: Self = Self {
        cude_indecies: [20, 19, 18, 23, 22, 21, 26, 25, 24],
        rotation: CubePermutation::BACK,
    };
    pub const L: Self = Self {
        cude_indecies: [18, 09, 00, 21, 12, 03, 24, 15, 06],
        rotation: CubePermutation::DOWN,
    };
    pub const R: Self = Self {
        cude_indecies: [02, 11, 20, 05, 14, 23, 08, 17, 26],
        rotation: CubePermutation::UP,
    };
    pub const U: Self = Self {
        cude_indecies: [18, 19, 20, 9, 10, 11, 00, 01, 02],
        rotation: CubePermutation::LEFT,
    };
    pub const D: Self = Self {
        cude_indecies: [06, 07, 08, 15, 16, 17, 24, 25, 26],
        rotation: CubePermutation::RIGHT,
    };
}

pub struct Rubik {
    cubes: [Cube; 27],
    color_map: CubeFaceMap<RubikColor>,
}

impl Rubik {
    pub const fn new() -> Self {
        Self {
            cubes: [Cube::new(); 27],
            color_map: CubeFaceMap::new(
                RubikColor::Red,
                RubikColor::Blue,
                RubikColor::White,
                RubikColor::Orange,
                RubikColor::Green,
                RubikColor::Yellow,
            ),
        }
    }
    unsafe fn ptr_of(&mut self, idx: u8) -> *mut Cube {
        self.cubes.as_mut_ptr().add(idx as usize)
    }
    pub fn get_color(&self, face: CubeFace) -> RubikColor {
        self.color_map[face]
    }
    pub fn r(&mut self) -> &mut Self {
        RubikLayer::R.rotate(self);
        self
    }
    pub fn l(&mut self) -> &mut Self {
        RubikLayer::L.rotate(self);
        self
    }
    pub fn u(&mut self) -> &mut Self {
        RubikLayer::U.rotate(self);
        self
    }
    pub fn d(&mut self) -> &mut Self {
        RubikLayer::D.rotate(self);
        self
    }
    pub fn f(&mut self) -> &mut Self {
        RubikLayer::F.rotate(self);
        self
    }
    pub fn b(&mut self) -> &mut Self {
        RubikLayer::B.rotate(self);
        self
    }
    pub fn iter_by_layer(&self, layer: &'static RubikLayer) -> RubikLayerIter<'_> {
        RubikLayerIter {
            layer,
            rubik: self,
            index: 0,
        }
    }
}

pub fn print_rubik(rubik: &Rubik) {
    use colored::Colorize;

    fn print_color(c: RubikColor) {
        match c {
            RubikColor::White => print!("{} ", "■".white()),
            RubikColor::Yellow => print!("{} ", "■".yellow()),
            RubikColor::Red => print!("{} ", "■".red()),
            RubikColor::Orange => print!("{} ", "■".magenta()),
            RubikColor::Blue => print!("{} ", "■".blue()),
            RubikColor::Green => print!("{} ", "■".green()),
        }
    }
    println!("====================");
    let mut counter = 0;
    for cube in rubik.iter_by_layer(&RubikLayer::U) {
        if counter % 3 == 0 {
            print!("\t");
        }
        let color = rubik.get_color(cube.get_top());
        print_color(color);
        counter += 1;
        if counter % 3 == 0 {
            println!();
        }
    }
    println!();
    for (block_cnt, (layer, face)) in [
        (&RubikLayer::L, CubeFace::L),
        (&RubikLayer::F, CubeFace::F),
        (&RubikLayer::R, CubeFace::R),
        (&RubikLayer::B, CubeFace::B),
    ].into_iter().enumerate() {
        counter = 0;
        for cube in rubik.iter_by_layer(layer) {
            if counter % 3 == 0 {
                // move to line's end
                print!("\x1B[{}C", block_cnt*8);
            }
            let color = rubik.get_color(cube.get(face));
            print_color(color);
            counter += 1;
            if counter % 3 == 0 {
                println!();
            }
        }
        print!("\x1B[3A");
    }
    print!("\x1B[3B");
    println!();
    counter = 0;
    for cube in rubik.iter_by_layer(&RubikLayer::D) {
        if counter % 3 == 0 {
            print!("\t");
        }
        let color = rubik.get_color(cube.get_down());
        print_color(color);
        counter += 1;
        if counter % 3 == 0 {
            println!();
        }
    }
    println!("====================");
}

#[test]
pub fn test_rubik() {
    let mut rubik = Rubik::new();
    print_rubik(&rubik);
    rubik.r();
    print_rubik(&rubik);
    rubik.l();
    print_rubik(&rubik);
    rubik.u();
    print_rubik(&rubik);
    rubik.d();
    print_rubik(&rubik);
    rubik.f();
    print_rubik(&rubik);
    rubik.b();
    print_rubik(&rubik);
}
