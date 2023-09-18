use crate::prelude::RubikLayerTransform;

use super::RubikSolver;

pub struct Shuffle {
    pub steps: usize,
}

impl Shuffle {
    pub fn new(steps: usize) -> Self {
        Self { steps }
    }
}
impl RubikSolver for Shuffle {
    fn solve(&self, mut rubik: crate::Rubik) -> (crate::Rubik, Vec<&'static RubikLayerTransform>) {
        let op = [
            &RubikLayerTransform::R,
            &RubikLayerTransform::F,
            &RubikLayerTransform::B,
            &RubikLayerTransform::D,
            &RubikLayerTransform::U,
            &RubikLayerTransform::L,
        ];
        let mut shuffle = vec![];
        for _round in 0..self.steps {
            let idx = rand::random::<usize>() % op.len();
            op[idx].apply_on(&mut rubik);
            shuffle.push(op[idx])
        }
        (rubik, shuffle)
    }
}
