#![allow(clippy::unusual_byte_groupings)]

use std::ops::Deref;

use cube::Cube;

pub mod colored;
pub mod cube;
pub mod operation;
pub mod permutation;
pub mod prelude;
pub mod transform;
pub mod parser;

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
    pub const F: Self = Self {
        cude_indexes: [00, 01, 02, 03, 04, 05, 06, 07, 08],
    };
    pub const B: Self = Self::F.flip_horizonal().bias(18);
    pub const S: Self = Self::F.bias(9);

    pub const L: Self = Self {
        cude_indexes: [18, 09, 00, 21, 12, 03, 24, 15, 06],
    };
    pub const M: Self = Self::L.bias(1);
    pub const R: Self = Self::L.flip_horizonal().bias(2);
    
    pub const U: Self = Self {
        cude_indexes: [18, 19, 20, 09, 10, 11, 00, 01, 02],
    };
    pub const D: Self = Self::U.flip_vertical().bias(4);
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

pub struct Rubik {
    cubes: [Cube; 27],
}

impl Rubik {
    pub const fn new() -> Self {
        Self {
            cubes: [Cube::new(); 27],
        }
    }

    pub fn is_solved(&self) -> bool {
        let first_cube = &self.cubes[0];
        self.cubes.iter().all(|c| c.eq(first_cube))
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
}
