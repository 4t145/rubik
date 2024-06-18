use std::{collections::HashSet, sync::Arc};

use crate::{
    cube::{Cube, CubeFace},
    prelude::{CubePermutation, RubikLayerTransform},
    Rubik,
};

use super::{RubikSolveState, RubikSolver, TransferableState};

pub struct IdaStarSolver {
    dist: fn(&Rubik) -> usize,
    digest: fn(&Rubik) -> u64,
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
            digest: digest_g0,
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
            digest: digest_g1,
        }
    }
}
fn dist_g0(rubik: &Rubik) -> usize {
    rubik
        .iter_by_layer(&crate::RubikLayer::U)
        .filter(|c| c.get(CubeFace::U) != CubeFace::U && c.get(CubeFace::U) != CubeFace::D)
        .count()
        + rubik
            .iter_by_layer(&crate::RubikLayer::D)
            .filter(|c| c.get(CubeFace::U) != CubeFace::U && c.get(CubeFace::U) != CubeFace::D)
            .count()
        + rubik
            .edges_e()
            .filter(|e| e.rotation == CubePermutation::UNIT)
            .count()
}

fn dist_g1(rubik: &Rubik) -> usize {
    rubik
        .corners()
        .filter(|c| c.rotation == CubePermutation::UNIT)
        .count()
}
fn g0_align(c: Cube) -> u64 {
    let u = c.get(CubeFace::U);
    if u == CubeFace::U || u == CubeFace::D {
        0
    } else if u == CubeFace::F || u == CubeFace::B {
        1
    } else {
        2
    }
}
fn g1_align(c: Cube) -> u64 {
    match c.rotation.factor().0 {
        CubePermutation::UNIT => 0,
        CubePermutation::X_2 => 1,
        CubePermutation::Y_2 => 2,
        _ => 3,
    }
}
fn digest_g0(rubik: &Rubik) -> u64 {
    rubik
        .active_cubes()
        .copied()
        .map(g0_align)
        .fold(0, |d, align| (d << 1) | align)
}
fn digest_g1(rubik: &Rubik) -> u64 {
    rubik
        .active_cubes()
        .copied()
        .map(g1_align)
        .fold(0, |d, align| (d << 1) | align)
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
                        let digest = (self.digest)(&next_state.rubik);
                        if !reached_set.contains(&digest) {
                            reached_set.insert(digest);
                            stacks.push((depth + 1, next_state));
                        }   
                    }
                }
                dbg!(stacks.len());
            }
            target_limit += 1;
        }
        panic!("REACH MAX DEPTH")
    }
}
