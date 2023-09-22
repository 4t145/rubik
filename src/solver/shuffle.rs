use std::sync::Arc;

use crate::prelude::RubikLayerTransform;

use super::{RubikSolveState, RubikSolver, TransferableState};

pub struct Shuffle {
    pub steps: usize,
}

impl Shuffle {
    pub fn new(steps: usize) -> Self {
        Self { steps }
    }
}
impl RubikSolver for Shuffle {
    fn solve(&self, mut rubik: crate::Rubik) -> RubikSolveState {
        let mut state = RubikSolveState {
            rubik,
            from: None,
            op_set: Arc::new([
                &RubikLayerTransform::R,
                &RubikLayerTransform::F,
                &RubikLayerTransform::B,
                &RubikLayerTransform::D,
                &RubikLayerTransform::U,
                &RubikLayerTransform::L,
            ]),
        };
        for _round in 0..self.steps {
            state = state.random_transfer()
        }
        state
    }
}
