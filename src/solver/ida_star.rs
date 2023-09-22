use std::{collections::HashSet, sync::Arc};

use crate::{cube::CubeFace, prelude::RubikLayerTransform, Rubik};

use super::{RubikSolveState, RubikSolver, TransferableState};

pub struct IdaStarSolver {
    dist: fn(&Rubik) -> usize,
    ops: Arc<[&'static RubikLayerTransform]>,
    max_depth: usize,
}
impl IdaStarSolver {
    pub fn g0() -> Self {
        Self {
            dist: dist_g0,
            ops: Arc::new([
                &RubikLayerTransform::R,
                &RubikLayerTransform::F,
                &RubikLayerTransform::B,
                &RubikLayerTransform::D,
                &RubikLayerTransform::U,
                &RubikLayerTransform::L,
                &RubikLayerTransform::RI,
                &RubikLayerTransform::FI,
                &RubikLayerTransform::BI,
                &RubikLayerTransform::DI,
                &RubikLayerTransform::UI,
                &RubikLayerTransform::LI,
            ]),
            max_depth: 40,
        }
    }
}
fn dist_g0(rubik: &Rubik) -> usize {
    rubik
        .active_cubes()
        .filter(|c| c.get(CubeFace::U) != CubeFace::U && c.get(CubeFace::U) != CubeFace::D)
        .count()
        / 4
}

impl RubikSolver for IdaStarSolver {
    fn solve(&self, rubik: Rubik) -> RubikSolveState {
        let mut target_limit = (self.dist)(&rubik);
        let mut stacks = vec![];
        let mut reached_set = HashSet::new();
        while target_limit < self.max_depth {
            dbg!(target_limit);
            dbg!(reached_set.len());
            stacks.push((0, RubikSolveState::new(rubik.clone(), self.ops.clone())));
            while let Some((depth, state)) = stacks.pop() {
                let dist = (self.dist)(&state.rubik);
                if dist == 0 {
                    return state;
                }
                if target_limit >= depth + (self.dist)(&state.rubik) {
                    for next_state in state.neighbors() {
                        if !reached_set.contains(&next_state.rubik) {
                            reached_set.insert(next_state.rubik.clone());
                            stacks.push((depth + 1, next_state));
                        }
                    }
                }
            }
            target_limit += 1;
        }
        panic!("REACH MAX DEPTH")
    }
}
