use rubik::{
    solver::{ida_star::IdaStarSolver, RubikSolver},
    transform::RubikLayerTransform,
    Rubik,
};
#[test]
fn test_g0_solver() {
    let mut rubik = Rubik::new();
    let g0_solver = IdaStarSolver::g0();
    rubik.shuffle(32);
    let (_r, ops) = g0_solver.solve(rubik).collect();
    dbg!(RubikLayerTransform::sequence_to_string(ops.into_iter()));
}

#[test]
fn test_g1_solver() {
    let mut rubik = Rubik::new();
    let g0_solver = IdaStarSolver::g0();
    let g1_solver = IdaStarSolver::g1();
    rubik.shuffle(32);
    let (r0, ops0) = g0_solver.solve(rubik).collect();
    dbg!(RubikLayerTransform::sequence_to_string(ops0.into_iter()));
    let (r1, ops1) = g1_solver.solve(r0).collect();
    dbg!(RubikLayerTransform::sequence_to_string(ops1.into_iter()));
}
