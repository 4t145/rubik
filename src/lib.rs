#![allow(clippy::unusual_byte_groupings)]

use std::{hash::Hash, ops::Deref};

use cube::{Cube, CubeFace};
use solver::RubikSolver;
use transform::RubikLayerTransform;

pub mod colored;
pub mod cube;
pub mod parser;
pub mod permutation;
pub mod prelude;
pub mod solver;
pub mod transform;
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
#[repr(transparent)]
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct RubikLayer {
    // cude indexes assuming rotaion is clockwise
    cude_indexes: [u8; 9],
}

impl Deref for RubikLayer {
    type Target = [u8; 9];
    fn deref(&self) -> &Self::Target {
        &self.cude_indexes
    }
}

#[allow(clippy::zero_prefixed_literal)]
impl RubikLayer {
    const fn marker(&self) -> &'static str {
        macro_rules! gen {
            ($($SYMBOL: ident),*) => {
                if false {
                    unreachable!()
                } $(
                    else if self.cude_indexes[0] == Self::$SYMBOL.cude_indexes[0] && self.cude_indexes[8] == Self::$SYMBOL.cude_indexes[8] {
                        stringify!($SYMBOL)
                    }
                )* else {
                    unreachable!()
                }
            };
        }
        gen!(F, B, S, L, M, R, U, D, E)
    }
    const fn bias(self, offset: i8) -> Self {
        Self {
            cude_indexes: [
                (self.cude_indexes[0] as i8 + offset) as u8,
                (self.cude_indexes[1] as i8 + offset) as u8,
                (self.cude_indexes[2] as i8 + offset) as u8,
                (self.cude_indexes[3] as i8 + offset) as u8,
                (self.cude_indexes[4] as i8 + offset) as u8,
                (self.cude_indexes[5] as i8 + offset) as u8,
                (self.cude_indexes[6] as i8 + offset) as u8,
                (self.cude_indexes[7] as i8 + offset) as u8,
                (self.cude_indexes[8] as i8 + offset) as u8,
            ],
        }
    }
    const fn flip_vertical(self) -> Self {
        Self {
            cude_indexes: [
                self.cude_indexes[6],
                self.cude_indexes[7],
                self.cude_indexes[8],
                self.cude_indexes[3],
                self.cude_indexes[4],
                self.cude_indexes[5],
                self.cude_indexes[0],
                self.cude_indexes[1],
                self.cude_indexes[2],
            ],
        }
    }
    const fn flip_horizonal(self) -> Self {
        Self {
            cude_indexes: [
                self.cude_indexes[2],
                self.cude_indexes[1],
                self.cude_indexes[0],
                self.cude_indexes[5],
                self.cude_indexes[4],
                self.cude_indexes[3],
                self.cude_indexes[8],
                self.cude_indexes[7],
                self.cude_indexes[6],
            ],
        }
    }
    pub const fn indexes(&self) -> &[u8; 9] {
        &self.cude_indexes
    }
    pub const F: Self = Self {
        cude_indexes: [00, 01, 02, 03, 04, 05, 06, 07, 08],
    };
    pub const B: Self = Self::F.flip_horizonal().bias(18);
    pub const S: Self = Self::F.flip_horizonal().bias(9);

    pub const L: Self = Self {
        cude_indexes: [18, 09, 00, 21, 12, 03, 24, 15, 06],
    };
    pub const M: Self = Self::L.bias(1);
    pub const R: Self = Self::L.flip_horizonal().bias(2);

    pub const U: Self = Self {
        cude_indexes: [18, 19, 20, 09, 10, 11, 00, 01, 02],
    };
    pub const D: Self = Self::U.flip_vertical().bias(6);
    pub const E: Self = Self::D.bias(-3);
}

pub struct RubikLayerIter<'r> {
    layer: &'static RubikLayer,
    rubik: &'r Rubik,
    index: usize,
}

impl<'r> Iterator for RubikLayerIter<'r> {
    type Item = &'r Cube;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.layer.len() {
            let cude = &self.rubik.cubes[self.layer.cude_indexes[self.index] as usize];
            self.index += 1;
            Some(cude)
        } else {
            None
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Rubik {
    cubes: [Cube; 27],
}

impl Rubik {
    pub const fn new() -> Self {
        Self {
            cubes: [Cube::new(); 27],
        }
    }

    pub fn reset(&mut self) -> &mut Self {
        self.cubes.iter_mut().for_each(|x| *x = Cube::new());
        self
    }

    pub fn is_solved(&self) -> bool {
        [
            (&RubikLayer::F, CubeFace::F),
            (&RubikLayer::B, CubeFace::B),
            (&RubikLayer::R, CubeFace::R),
            (&RubikLayer::L, CubeFace::L),
            (&RubikLayer::U, CubeFace::U),
            (&RubikLayer::D, CubeFace::D),
        ]
        .iter()
        .all(|(layer, face)| {
            let mid_face = self.get_by_layer(layer, 4).get(*face);
            self.iter_by_layer(layer)
                .map(|c| c.get(*face))
                .all(|f| f == mid_face)
        })
    }

    unsafe fn ptr_of(&mut self, idx: u8) -> *mut Cube {
        self.cubes.as_mut_ptr().add(idx as usize)
    }

    pub fn iter_by_layer(&self, layer: &'static RubikLayer) -> RubikLayerIter<'_> {
        RubikLayerIter {
            layer,
            rubik: self,
            index: 0,
        }
    }

    pub fn core(&self) -> &Cube {
        &self.cubes[13]
    }

    pub fn get_by_layer(&self, layer: &'static RubikLayer, index: usize) -> &Cube {
        &self.cubes[layer.cude_indexes[index] as usize]
    }

    pub fn is_aligned(&self, cube: &Cube) -> bool {
        (*self.core()).eq(cube)
    }

    pub fn solve(&mut self, solver: impl RubikSolver) -> Vec<&'static RubikLayerTransform> {
        let s = solver.solve(self.clone());
        let (rubik, ops) = s.collect();
        *self = rubik;
        ops
    }

    pub fn shuffle(&mut self, steps: usize) -> Vec<&'static RubikLayerTransform> {
        self.solve(solver::shuffle::Shuffle::new(steps))
    }

    pub fn cubes_at(&self, iter: impl Iterator<Item = usize>) -> impl Iterator<Item = &Cube> {
        iter.map(move |x| &self.cubes[x])
    }

    pub fn edges(&self) -> impl Iterator<Item = &Cube> {
        self.cubes_at([1, 3, 5, 7, 9, 11, 15, 17, 19, 21, 23, 25].into_iter())
    }

    pub fn edges_e(&self) -> impl Iterator<Item = &Cube> {
        [3, 5, 21, 23].iter().map(move |&x| &self.cubes[x])
    }

    pub fn corners(&self) -> impl Iterator<Item = &Cube> {
        self.cubes_at([0, 2, 6, 8, 18, 20, 24, 26].into_iter())
    }

    pub fn centers(&self) -> impl Iterator<Item = &Cube> {
        [4, 10, 12, 14, 16, 22].iter().map(move |&x| &self.cubes[x])
    }

    pub fn active_cubes(&self) -> impl Iterator<Item = &Cube> {
        self.edges().chain(self.corners()).chain(Some(self.core()))
    }

    pub fn entropy(&self) -> f64 {
        Cube::entropy(self.active_cubes().copied())
    }
}

impl Default for Rubik {
    fn default() -> Self {
        Self {
            cubes: [Cube::new(); 27],
        }
    }
}

impl std::fmt::Debug for Rubik {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn collect_layer(face: CubeFace, iter: RubikLayerIter<'_>, f: &mut std::fmt::DebugStruct) {
            let v: Vec<_> = iter.map(|x| x.get(face)).collect();
            f.field(&format!("{face:?}"), &[&v[0..3], &v[3..6], &v[6..9]]);
        }
        let mut f = f.debug_struct("Rubik");
        collect_layer(CubeFace::F, self.iter_by_layer(&RubikLayer::F), &mut f);
        collect_layer(CubeFace::B, self.iter_by_layer(&RubikLayer::B), &mut f);
        collect_layer(CubeFace::R, self.iter_by_layer(&RubikLayer::R), &mut f);
        collect_layer(CubeFace::L, self.iter_by_layer(&RubikLayer::L), &mut f);
        collect_layer(CubeFace::U, self.iter_by_layer(&RubikLayer::U), &mut f);
        collect_layer(CubeFace::D, self.iter_by_layer(&RubikLayer::D), &mut f);
        f.finish()
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CubePosition {
    FUL = 0,
    FU = 1,
    FUR = 2,
    FL = 3,
    F = 4,
    FR = 5,
    FDL = 6,
    FD = 7,
    FDR = 8,
    UL = 9,
    U = 10,
    UR = 11,
    L = 12,
    C = 13,
    R = 14,
    DLL = 15,
    DL = 16,
    DLR = 17,
    BUL = 18,
    BU = 19,
    BUR = 20,
    BL = 21,
    B = 22,
    BR = 23,
    BDL = 24,
    BD = 25,
    BDR = 26,
}

impl CubePosition {
    unsafe fn from_u8(pos_code: u8) -> Self {
        std::mem::transmute(pos_code)
    }
    pub fn try_from_u8(pos_code: u8) -> Option<Self> {
        if pos_code < 27 {
            Some(unsafe { Self::from_u8(pos_code) })
        } else {
            None
        }
    }
    pub fn horizon_layer(self) -> &'static RubikLayer {
        match (self as u8) % 9 {
            0..=2 => &RubikLayer::U,
            3..=5 => &RubikLayer::E,
            6..=8 => &RubikLayer::D,
            _ => unreachable!(),
        }
    }
    pub fn frontal_layer(self) -> &'static RubikLayer {
        match (self as u8) / 9 {
            0 => &RubikLayer::F,
            1 => &RubikLayer::S,
            2 => &RubikLayer::B,
            _ => unreachable!(),
        }
    }
    pub fn lateral_layer(self) -> &'static RubikLayer {
        match (self as u8) % 3 {
            0 => &RubikLayer::L,
            1 => &RubikLayer::M,
            2 => &RubikLayer::R,
            _ => unreachable!(),
        }
    }
}
