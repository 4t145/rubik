use crate::permutation::CubePermutation;
use std::{collections::HashMap, fmt::Debug, ops::AddAssign};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

impl Default for Cube {
    fn default() -> Self {
        Self::new()
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

    pub fn entropy(iter: impl Iterator<Item = Self>) -> f64 {
        let (map, n) = iter.fold((HashMap::new(), 0usize), |(mut map, count), cube| {
            map.entry(cube).or_insert(0_usize).add_assign(1);
            (map, count + 1)
        });
        let n = n as f64;
        map.values().map(|x| (n / (*x as f64)).log2()).sum()
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
