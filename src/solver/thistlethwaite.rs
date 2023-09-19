use std::collections::HashSet;

use crate::{
    prelude::{Cube, CubeFace, CubePermutation, RubikLayerTransform},
    tf, Rubik, RubikLayer,
};

use super::RubikSolver;

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
    rubik.iter_by_layer(&RubikLayer::E).all(|cube| {
        all_aligned(cube.clone().rotate(CubePermutation::Y_1))
            || all_aligned(
                cube.clone()
                    .rotate(CubePermutation::Y_1.compose(CubePermutation::Z_1)),
            )
    }) && rubik.iter_by_layer(&RubikLayer::M).all(|cube| {
        all_aligned(cube.clone().rotate(CubePermutation::Z_1))
            || all_aligned(
                cube.clone()
                    .rotate(CubePermutation::Z_1.compose(CubePermutation::Y_1)),
            )
    }) && rubik.iter_by_layer(&RubikLayer::S).all(|cube| {
        all_aligned(cube.clone().rotate(CubePermutation::Z_1))
            || all_aligned(cube.clone().rotate(CubePermutation::Y_1))
    })
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
    fn bfs(
        &self,
        quene: Vec<(Rubik, Vec<&'static RubikLayerTransform>)>,
        mut reached: HashSet<Rubik>,
    ) -> (Rubik, Vec<&'static RubikLayerTransform>) {
        let mut new_quene = vec![];
        for (rubik, ops) in quene.into_iter() {
            if (self.checker)(&rubik) {
                return (rubik, ops);
            }
            for op in self.gen_group {
                let mut rubik = rubik.clone();
                op.apply_on(&mut rubik);
                if !reached.contains(&rubik) {
                    reached.insert(rubik.clone());
                    let mut new_ops = ops.clone();
                    new_ops.push(*op);
                    new_quene.push((rubik, new_ops));
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
    fn solve(&self, rubik: Rubik) -> (Rubik, Vec<&'static RubikLayerTransform>) {
        self.bfs(vec![(rubik.clone(), vec![])], HashSet::from([rubik]))
    }
}

impl<A: RubikSolver, B: RubikSolver> RubikSolver for (A, B) {
    fn solve(&self, rubik: Rubik) -> (Rubik, Vec<&'static RubikLayerTransform>) {
        let (s0, o0) = self.0.solve(rubik);
        let (s1, o1) = self.1.solve(s0);
        (s1, [o0, o1].concat())
    }
}

pub struct CSolver;

impl RubikSolver for Thistlethwaite {
    fn solve(&self, rubik: crate::Rubik) -> (Rubik, Vec<&'static RubikLayerTransform>) {
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
