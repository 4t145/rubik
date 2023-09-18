use crate::{
    prelude::{Cube, CubeFace, CubePermutation, RubikLayerTransform},
    tf, Rubik, RubikLayer,
};

use super::RubikSolver;

pub struct Thistlethwaite {
    pub thread: usize,
}

impl RubikSolver for Thistlethwaite {
    fn solve(&mut self, rubik: &crate::Rubik) -> Vec<&'static RubikLayerTransform> {
        // <U,D,F,B,R,L> -> <U,D,F,B,R2,L2>
        // single thread
        let rubik = rubik.clone();
        // bfs
        fn bfs(
            quene: Vec<(Rubik, Vec<&'static RubikLayerTransform>)>,
            gen_group: &[&'static RubikLayerTransform],
            checker: fn(&Rubik) -> bool,
        ) -> (Rubik, Vec<&'static RubikLayerTransform>) {
            let mut new_quene = vec![];
            for (rubik, ops) in quene.into_iter() {
                if checker(&rubik) {
                    return (rubik, ops);
                }
                for op in gen_group {
                    let mut rubik = rubik.clone();
                    op.apply_on(&mut rubik);
                    let mut new_ops = ops.clone();
                    new_ops.push(*op);
                    new_quene.push((rubik, new_ops));
                }
            }
            bfs(new_quene, gen_group, checker)
        }

        fn ud_aligned(cube: &Cube) -> bool {
            let f = cube.get(CubeFace::U);
            f == CubeFace::U || f == CubeFace::D
        }
        fn rl_aligned(cube: &Cube) -> bool {
            let f = cube.get(CubeFace::L);
            f == CubeFace::L || f == CubeFace::R
        }
        fn g0_aligned(cube: &Cube) -> bool {
            let fu = cube.get(CubeFace::U);
            let fl = cube.get(CubeFace::L);
            fu != CubeFace::L && fu != CubeFace::R && fl != CubeFace::U && fl != CubeFace::D
        }
        // 0. align core
        let (s0, o0) = bfs(
            vec![(rubik, vec![])],
            &[
                &crate::transform::E,
                &crate::transform::S,
                &crate::transform::M,
            ],
            |r| r.core().rotation == CubePermutation::UNIT,
        );
        println!("c solved in {} steps", o0.len());

        // 1. G0: <U,D,F,B,R,L> -> <U,D,F,B,R2,L2>
        fn g0_checker(rubik: &Rubik) -> bool {
            // check e layer
            rubik.iter_by_layer(&RubikLayer::E).all(g0_aligned)
                && rubik.iter_by_layer(&RubikLayer::M).all(g0_aligned)
                && rubik.iter_by_layer(&RubikLayer::S).all(g0_aligned)
        }
        let (s1, o1) = bfs(
            vec![(s0, vec![])],
            &[
                &crate::transform::R,
                &crate::transform::F,
                &crate::transform::B,
                &crate::transform::D,
                &crate::transform::U,
                &crate::transform::L,
            ],
            g0_checker,
        );
        println!("g0 solved in {} steps", o1.len());
        // 2. G1: <U,D,F,B,R2,L2> -> <U,D,F2,B2,R2,L2>
        fn g1_checker(rubik: &Rubik) -> bool {
            // check e layer
            rubik.iter_by_layer(&RubikLayer::U).all(ud_aligned)
                && rubik.iter_by_layer(&RubikLayer::D).all(ud_aligned)
        }
        let (s2, o2) = bfs(
            vec![(s1, vec![])],
            &[
                &crate::transform::F,
                &crate::transform::B,
                &crate::transform::D,
                &crate::transform::U,
                &crate::transform::R2,
                &crate::transform::L2,
            ],
            g1_checker,
        );
        println!("g1 solved in {} steps", o2.len());

        // 3. G2: <U,D,F2,B2,R2,L2> -> <U2,D2,F2,B2,R2,L2>
        fn g2_checker(rubik: &Rubik) -> bool {
            // check e layer
            rubik.iter_by_layer(&RubikLayer::R).all(rl_aligned)
                && rubik.iter_by_layer(&RubikLayer::L).all(rl_aligned)
        }
        let (s3, o3) = bfs(
            vec![(s2, vec![])],
            &[
                &crate::transform::F,
                &crate::transform::B,
                &crate::transform::D2,
                &crate::transform::U2,
                &crate::transform::R2,
                &crate::transform::L2,
            ],
            g2_checker,
        );
        println!("g2 solved in {} steps", o3.len());
        // 4. G3: <U2,D2,F2,B2,R2,L2> -> <>
        fn g3_checker(rubik: &Rubik) -> bool {
            // check e layer
            rubik.is_solved()
        }
        let (s4, o4) = bfs(
            vec![(s3, vec![])],
            &[
                &crate::transform::F2,
                &crate::transform::B2,
                &crate::transform::D2,
                &crate::transform::U2,
                &crate::transform::R2,
                &crate::transform::L2,
            ],
            g3_checker,
        );
        println!("g3 solved in {} steps", o4.len());
        [o0, o1, o2, o3, o4].concat()
    }
}
