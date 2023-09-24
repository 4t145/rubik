use std::{collections::HashSet, sync::Arc};

use crate::{
    cube::{Cube, CubeFace},
    prelude::{CubePermutation, RubikLayerTransform},
    Rubik, RubikLayer,
};

use super::{RubikSolveState, RubikSolver, TransferableState};

pub struct IdaStarSolver {
    dist: fn(&Rubik) -> usize,
    ops: Arc<[&'static RubikLayerTransform]>,
    digest: fn(&Rubik) -> u64,
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
            digest: digest_g0,
        }
    }
}

fn digest_g0(rubik: &Rubik) -> u64 {
    fn encode_edge_e(cube: &Cube) -> u64 {
        let s = cube.rotation.factor().1 .0;
        match s {
            0b_11_10_01_00 => 0,
            0b_10_11_01_00 => 1,
            0b_11_01_10_00 => 2,
            0b_01_11_10_00 => 3,
            0b_01_10_11_00 => 4,
            0b_10_01_11_00 => 5,
            rest => unreachable!("{:08b}", rest),
        }
    }
    // fn encode_edge_ud(cube: &Cube) -> u64 {
    //     let s = cube.rotation.factor().1 .0;
    //     match s {
    //         0b_11_10_01_00 => 0,
    //         0b_10_11_01_00 => 0,
    //         0b_11_01_10_00 => 2,
    //         0b_01_11_10_00 => 2,
    //         0b_01_10_11_00 => 4,
    //         0b_10_01_11_00 => 4,
    //         rest => unreachable!("{:08b}", rest),
    //     }
    // }
    let digest = 0;
    let digest = rubik
        .cubes_at([1, 7, 9, 11, 15, 17, 19, 25].into_iter())
        .fold(digest, |d, cube| (d << 3) | encode_edge_e(cube));
    let digest = rubik
        .cubes_at([3, 5, 21, 23].into_iter())
        .fold(digest, |d, cube| (d << 3) | encode_edge_e(cube));
    let digest = rubik.corners().fold(digest, |d, cube| {
        let up = cube.get(CubeFace::U);
        (d << 1)
            | if up == CubeFace::U || up == CubeFace::D {
                1
            } else {
                0
            }
    });
    digest
}

fn dist_g0(rubik: &Rubik) -> usize {
    (rubik
        .iter_by_layer(&RubikLayer::U)
        .chain(rubik.iter_by_layer(&RubikLayer::D))
        .filter(|c| c.get(CubeFace::U) != CubeFace::U && c.get(CubeFace::U) != CubeFace::D)
        .count()
        + [3, 5, 21, 23]
            .into_iter()
            .map(|idx| rubik.cubes[idx])
            .filter(|c| {
                let r = c.get(CubeFace::R);
                let f = c.get(CubeFace::F);
                (r != CubeFace::R && r != CubeFace::L) || (f != CubeFace::F && f != CubeFace::B)
            })
            .count())
        / 2
}

fn dist_g1(rubik: &Rubik) -> usize {
    rubik
        .active_cubes()
        .filter(|c| {
            let r = c.get(CubeFace::R);
            let f = c.get(CubeFace::F);
            (r == CubeFace::R || r == CubeFace::L) && (f == CubeFace::F || r == CubeFace::B)
        })
        .count()
}

impl RubikSolver for IdaStarSolver {
    fn solve(&self, rubik: Rubik) -> RubikSolveState {
        let mut target_limit = (self.dist)(&rubik);
        let mut stacks = vec![];
        let mut reached_set = HashSet::new();
        while target_limit < self.max_depth {
            reached_set.clear();
            stacks.push((0, RubikSolveState::new(rubik.clone(), self.ops.clone())));
            let mut min_dist = usize::MAX;
            let mut min_op = vec![];
            while let Some((depth, state)) = stacks.pop() {
                let dist = (self.dist)(&state.rubik);
                if dist == 0 {
                    return state;
                }
                if dist < min_dist {
                    min_op = state.clone().collect().1;
                    min_dist = dist;
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
            }
            target_limit += 1;
            println!("min_dist: {}", min_dist);
            dbg!(target_limit);
            dbg!(reached_set.len());
            dbg!(RubikLayerTransform::sequence_to_string(
                min_op.iter().copied()
            ));
        }
        panic!("REACH MAX DEPTH")
    }
}
