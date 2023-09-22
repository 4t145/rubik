use std::{collections::HashSet, sync::Arc};

use crate::{
    cube::CubeFace,
    prelude::{CubePermutation, RubikLayerTransform},
    Rubik,
};

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
    pub fn g1() -> Self {
        Self {
            dist: dist_g1,
            ops: Arc::new([
                &RubikLayerTransform::R2,
                &RubikLayerTransform::F2,
                &RubikLayerTransform::B2,
                &RubikLayerTransform::L2,
                &RubikLayerTransform::D,
                &RubikLayerTransform::U,
                &RubikLayerTransform::DI,
                &RubikLayerTransform::UI,
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
}

fn dist_g1(rubik: &Rubik) -> usize {
    rubik
        .active_cubes()
        .filter(|c| c.rotation == CubePermutation::UNIT)
        .count()
}

impl RubikSolver for IdaStarSolver {
    fn solve(&self, rubik: Rubik) -> RubikSolveState {
        let mut target_limit = (self.dist)(&rubik);
        let mut stacks = vec![];
        let mut reached_set = HashSet::new();
        while target_limit < self.max_depth {
            dbg!(target_limit);
            dbg!(reached_set.len());
            reached_set.clear();
            stacks.push((0, RubikSolveState::new(rubik.clone(), self.ops.clone())));
            while let Some((depth, state)) = stacks.pop() {
                let dist = (self.dist)(&state.rubik);
                if dist == 0 {
                    return state;
                }
                if target_limit >= depth + dist {
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
