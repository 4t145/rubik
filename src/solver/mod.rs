use crate::{prelude::RubikLayerTransform, Rubik};

pub trait RubikSolver {
    fn solve(&self, rubik: Rubik) -> (Rubik, Vec<&'static RubikLayerTransform>);
}

pub mod thistlethwaite;
pub mod shuffle;