use crate::{prelude::RubikLayerTransform, Rubik};

pub trait RubikSolver {
    fn solve(&mut self, rubik: &Rubik) -> Vec<&'static RubikLayerTransform>;
}

pub mod thistlethwaite;
