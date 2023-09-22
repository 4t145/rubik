use std::collections::HashSet;

use crate::{
    prelude::{Cube, CubeFace, CubePermutation, RubikLayerTransform},
    solver::TransferableState,
    tf, Rubik, RubikLayer,
};

use super::{RubikSolveState, RubikSolver};

pub struct Thistlethwaite {
    pub thread: usize,
}

pub struct BfsSolver {
    gen_group: &'static [&'static RubikLayerTransform],
    checker: fn(&Rubik) -> bool,
}

fn all_aligned(cube: &Cube) -> bool {
    let l = cube.get(CubeFace::L);
    let f = cube.get(CubeFace::F);
    let u = cube.get(CubeFace::U);
    (l == CubeFace::L || l == CubeFace::R)
        && (f == CubeFace::F || f == CubeFace::B)
        && (u == CubeFace::U || u == CubeFace::D)
}

fn checker_c(rubik: &Rubik) -> bool {
    rubik.core().rotation == CubePermutation::UNIT
}
fn checker_g0(rubik: &Rubik) -> bool {
    rubik
        .active_cubes()
        .all(|c| c.get(CubeFace::U) == CubeFace::U || c.get(CubeFace::U) == CubeFace::D)
}
fn checker_g1(rubik: &Rubik) -> bool {
    fn g1_aligned(cube: &Cube) -> bool {
        let f = cube.get(CubeFace::U);
        f == CubeFace::U || f == CubeFace::D
    }
    rubik.cubes.iter().all(g1_aligned)
}
fn checker_g2(rubik: &Rubik) -> bool {
    fn g2_aligned(cube: &Cube) -> bool {
        let f = cube.get(CubeFace::L);
        f == CubeFace::L || f == CubeFace::R
    }
    rubik.cubes.iter().all(g2_aligned)
}

impl BfsSolver {
    fn bfs(&self, quene: Vec<RubikSolveState>, mut reached: HashSet<Rubik>) -> RubikSolveState {
        let mut new_quene = vec![];
        for state in quene.into_iter() {
            if (self.checker)(&state.rubik) {
                return state;
            }
            for next_state in state.neighbors() {
                if !reached.contains(&next_state.rubik) {
                    reached.insert(next_state.rubik.clone());
                    new_quene.push(next_state);
                }
            }
        }
        println!("state size: {}", reached.len());
        self.bfs(new_quene, reached)
    }
    pub const C: Self = BfsSolver {
        gen_group: &[
            &crate::transform::E,
            &crate::transform::S,
            &crate::transform::M,
        ],
        checker: checker_c,
    };
    pub const G0: Self = BfsSolver {
        gen_group: &[
            &crate::transform::F,
            &crate::transform::B,
            &crate::transform::D,
            &crate::transform::U,
            &crate::transform::R,
            &crate::transform::L,
            &crate::transform::FI,
            &crate::transform::BI,
            &crate::transform::DI,
            &crate::transform::UI,
            &crate::transform::RI,
            &crate::transform::LI,
        ],
        checker: checker_g0,
    };
    pub const G1: Self = BfsSolver {
        gen_group: &[
            &crate::transform::F2,
            &crate::transform::B2,
            &crate::transform::R,
            &crate::transform::L,
            &crate::transform::D,
            &crate::transform::U,
            &crate::transform::RI,
            &crate::transform::LI,
            &crate::transform::DI,
            &crate::transform::UI,
        ],
        checker: checker_g1,
    };
    pub const G2: Self = BfsSolver {
        gen_group: &[
            &crate::transform::F2,
            &crate::transform::B2,
            &crate::transform::R2,
            &crate::transform::L2,
            &crate::transform::D,
            &crate::transform::U,
            &crate::transform::DI,
            &crate::transform::UI,
        ],
        checker: checker_g2,
    };
    pub const G3: Self = BfsSolver {
        gen_group: &[
            &crate::transform::F2,
            &crate::transform::B2,
            &crate::transform::R2,
            &crate::transform::L2,
            &crate::transform::D2,
            &crate::transform::U2,
        ],
        checker: Rubik::is_solved,
    };
}

impl RubikSolver for BfsSolver {
    fn solve(&self, rubik: Rubik) -> RubikSolveState {
        let state = RubikSolveState {
            rubik: rubik.clone(),
            op_set: self.gen_group.into(),
            from: None,
        };
        self.bfs(vec![state], HashSet::from([rubik]))
    }
}

impl<A: RubikSolver, B: RubikSolver> RubikSolver for (A, B) {
    fn solve(&self, rubik: Rubik) -> RubikSolveState {
        let s0 = self.0.solve(rubik);
        self.1.solve(s0.rubik)
    }
}

pub struct CSolver;

impl RubikSolver for Thistlethwaite {
    fn solve(&self, rubik: crate::Rubik) -> RubikSolveState {
        (
            BfsSolver::C,
            (
                BfsSolver::G0,
                (BfsSolver::G1, (BfsSolver::G2, BfsSolver::G3)),
            ),
        )
            .solve(rubik)
    }
}
