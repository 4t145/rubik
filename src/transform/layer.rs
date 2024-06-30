use std::fmt::{Display, Formatter};

use crate::permutation::CubePermutation;
use crate::{CubePosition, Rubik, RubikLayer};

use super::RubikTransform;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct RubikLayerTransform {
    // cude indexes assuming rotaion is clockwise
    layer: &'static RubikLayer,
    rotation: CubePermutation,
    ptr_rotate: PtrRotate,
}

impl std::fmt::Debug for RubikLayerTransform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RubikLayerTransform")
            .field("layer", &self.layer)
            .field("rotation", &self.rotation)
            .field("ptr_rotate", &self.ptr_rotate)
            .finish()
    }
}

impl Display for RubikLayerTransform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.layer.marker())?;
        f.write_str(match self.ptr_rotate {
            PtrRotate::Rotate0 => "0",
            PtrRotate::Rotate1 => "",
            PtrRotate::Rotate2 => "2",
            PtrRotate::Rotate3 => "'",
        })
    }
}

#[allow(clippy::zero_prefixed_literal)]
impl RubikLayerTransform {
    pub fn sequence_to_string<'a>(seq: impl Iterator<Item = &'a Self>) -> String {
        let mut s = String::new();
        for tf in seq {
            s.push_str(&tf.to_string());
        }
        s
    }
    pub fn apply_on(&self, rubik: &mut Rubik) {
        for index in self.layer.iter().copied() {
            rubik.cubes[index as usize].rotate(self.rotation);
        }
        unsafe {
            let indicies = &self.layer;
            self.ptr_rotate.call([
                rubik.ptr_of(indicies[0]),
                rubik.ptr_of(indicies[2]),
                rubik.ptr_of(indicies[8]),
                rubik.ptr_of(indicies[6]),
            ]);
            self.ptr_rotate.call([
                rubik.ptr_of(indicies[1]),
                rubik.ptr_of(indicies[5]),
                rubik.ptr_of(indicies[7]),
                rubik.ptr_of(indicies[3]),
            ]);
        }
    }
    pub fn apply_on_position(&self, position: CubePosition) -> Option<CubePosition> {
        let indicies = &self.layer;
        for (idx, pos) in indicies.iter().enumerate() {
            if *pos == position as u8 {
                unsafe {
                    return Some(CubePosition::from_u8(
                        indicies[(self.ptr_rotate.as_permutation())[idx]],
                    ));
                }
            }
        }
        None
    }
    pub const fn inverse(self) -> Self {
        Self {
            layer: self.layer,
            rotation: self.rotation.inverse(),
            ptr_rotate: self.ptr_rotate.inverse(),
        }
    }
    pub const fn square(self) -> Self {
        Self {
            layer: self.layer,
            rotation: self.rotation.compose(self.rotation),
            ptr_rotate: self.ptr_rotate.square(),
        }
    }
    pub const fn rotation(&self) -> CubePermutation {
        self.rotation
    }
    pub const fn layer(&self) -> &'static RubikLayer {
        self.layer
    }
    pub const fn ptr_rotate(&self) -> PtrRotate {
        self.ptr_rotate
    }
    pub const F: Self = Self {
        layer: &RubikLayer::F,
        rotation: CubePermutation::FRONT,
        ptr_rotate: PtrRotate::Rotate1,
    };
    pub const B: Self = Self {
        layer: &RubikLayer::B,
        rotation: CubePermutation::BACK,
        ptr_rotate: PtrRotate::Rotate1,
    };
    pub const L: Self = Self {
        rotation: CubePermutation::LEFT,
        layer: &RubikLayer::L,
        ptr_rotate: PtrRotate::Rotate1,
    };
    pub const R: Self = Self {
        layer: &RubikLayer::R,
        rotation: CubePermutation::RIGHT,
        ptr_rotate: PtrRotate::Rotate1,
    };
    pub const U: Self = Self {
        layer: &RubikLayer::U,
        rotation: CubePermutation::UP,
        ptr_rotate: PtrRotate::Rotate1,
    };
    pub const D: Self = Self {
        layer: &RubikLayer::D,
        rotation: CubePermutation::DOWN,
        ptr_rotate: PtrRotate::Rotate1,
    };
    pub const M: Self = Self {
        layer: &RubikLayer::M,
        rotation: CubePermutation::LEFT,
        ptr_rotate: PtrRotate::Rotate1,
    };
    pub const E: Self = Self {
        layer: &RubikLayer::E,
        rotation: CubePermutation::DOWN,
        ptr_rotate: PtrRotate::Rotate1,
    };
    pub const S: Self = Self {
        layer: &RubikLayer::S,
        rotation: CubePermutation::BACK,
        ptr_rotate: PtrRotate::Rotate1,
    };
    pub const UNIT: Self = Self {
        layer: &RubikLayer::F,
        rotation: CubePermutation::UNIT,
        ptr_rotate: PtrRotate::Rotate0,
    };
    pub const FI: Self = Self::F.inverse();
    pub const BI: Self = Self::B.inverse();
    pub const LI: Self = Self::L.inverse();
    pub const RI: Self = Self::R.inverse();
    pub const UI: Self = Self::U.inverse();
    pub const DI: Self = Self::D.inverse();
    pub const MI: Self = Self::M.inverse();
    pub const EI: Self = Self::E.inverse();
    pub const SI: Self = Self::S.inverse();

    pub const F2: Self = Self::F.square();
    pub const B2: Self = Self::B.square();
    pub const L2: Self = Self::L.square();
    pub const R2: Self = Self::R.square();
    pub const U2: Self = Self::U.square();
    pub const D2: Self = Self::D.square();
    pub const M2: Self = Self::M.square();
    pub const E2: Self = Self::E.square();
    pub const S2: Self = Self::S.square();
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PtrRotate {
    Rotate0,
    Rotate1,
    Rotate2,
    Rotate3,
}

impl PtrRotate {
    pub unsafe fn call<T>(&self, values: [*mut T; 4]) {
        match self {
            PtrRotate::Rotate0 => {}
            PtrRotate::Rotate1 => ptr_rotate_1(values),
            PtrRotate::Rotate2 => ptr_rotate_2(values),
            PtrRotate::Rotate3 => ptr_rotate_3(values),
        }
    }
    pub const fn inverse(self) -> Self {
        match self {
            PtrRotate::Rotate0 => PtrRotate::Rotate0,
            PtrRotate::Rotate1 => PtrRotate::Rotate3,
            PtrRotate::Rotate2 => PtrRotate::Rotate2,
            PtrRotate::Rotate3 => PtrRotate::Rotate1,
        }
    }
    pub const fn square(self) -> Self {
        match self {
            PtrRotate::Rotate0 => PtrRotate::Rotate0,
            PtrRotate::Rotate1 => PtrRotate::Rotate2,
            PtrRotate::Rotate2 => PtrRotate::Rotate0,
            PtrRotate::Rotate3 => PtrRotate::Rotate2,
        }
    }
    pub const fn as_permutation(self) -> &'static [usize; 9] {
        match self {
            PtrRotate::Rotate0 => &[0, 1, 2, 3, 4, 5, 6, 7, 8],
            PtrRotate::Rotate1 => &[6, 3, 0, 7, 4, 1, 8, 5, 2],
            PtrRotate::Rotate2 => &[8, 7, 6, 5, 4, 3, 2, 1, 0],
            PtrRotate::Rotate3 => &[2, 5, 8, 1, 4, 7, 0, 3, 6],
        }
    }
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

impl From<RubikLayerTransform> for RubikTransform {
    fn from(val: RubikLayerTransform) -> Self {
        RubikTransform::Layer(val)
    }
}
