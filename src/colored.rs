use crate::cube::CubeFace;

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

impl RubikColor {
    pub const fn classic_map() -> CubeFaceMap<Self> {
        CubeFaceMap::new(
            RubikColor::Red,
            RubikColor::Blue,
            RubikColor::White,
            RubikColor::Orange,
            RubikColor::Green,
            RubikColor::Yellow,
        )
    }
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
