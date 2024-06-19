use rubik::prelude::*;

#[test]
pub fn test_cube() {
    let mut cube = Cube::new();
    cube.rotate(CubePermutation::Z_1);
    assert_eq!(cube.get(CubeFace::L), CubeFace::F);
    assert_eq!(cube.get(CubeFace::F), CubeFace::R);
    assert_eq!(cube.get(CubeFace::R), CubeFace::B);
    assert_eq!(cube.get(CubeFace::B), CubeFace::L);
    assert_eq!(cube.get(CubeFace::U), CubeFace::U);
    assert_eq!(cube.get(CubeFace::D), CubeFace::D);
    cube.rotate(CubePermutation::Z_1);
    assert_eq!(cube.get(CubeFace::L), CubeFace::R);
    assert_eq!(cube.get(CubeFace::F), CubeFace::B);
    assert_eq!(cube.get(CubeFace::R), CubeFace::L);
    assert_eq!(cube.get(CubeFace::B), CubeFace::F);
    assert_eq!(cube.get(CubeFace::U), CubeFace::U);
    assert_eq!(cube.get(CubeFace::D), CubeFace::D);
    cube.rotate(CubePermutation::Y_2);
    assert_eq!(cube.get(CubeFace::L), CubeFace::R);
    assert_eq!(cube.get(CubeFace::R), CubeFace::L);
    assert_eq!(cube.get(CubeFace::F), CubeFace::F);
    assert_eq!(cube.get(CubeFace::B), CubeFace::B);
    assert_eq!(cube.get(CubeFace::U), CubeFace::D);
    assert_eq!(cube.get(CubeFace::D), CubeFace::U);
    cube.rotate(CubePermutation::X_2);
    assert_eq!(cube.get(CubeFace::L), CubeFace::L);
    assert_eq!(cube.get(CubeFace::R), CubeFace::R);
    assert_eq!(cube.get(CubeFace::F), CubeFace::F);
    assert_eq!(cube.get(CubeFace::B), CubeFace::B);
    assert_eq!(cube.get(CubeFace::U), CubeFace::U);
    assert_eq!(cube.get(CubeFace::D), CubeFace::D);
}

#[test]
pub fn test_cube_xyz_rotation() {
    let mut cube = Cube::new();
    cube.rotate(
        CubePermutation::X_1
            .compose(CubePermutation::Y_1)
            .compose(CubePermutation::Z_1),
    );
    dbg!(cube);
    let mut cube = Cube::new();
    cube.rotate(
        CubePermutation::I
    );
    dbg!(cube);
    // assert_eq!(cube.get(CubeFace::L), CubeFace::F);
    // assert_eq!(cube.get(CubeFace::F), CubeFace::R);
    // assert_eq!(cube.get(CubeFace::R), CubeFace::B);
    // assert_eq!(cube.get(CubeFace::B), CubeFace::L);
    // assert_eq!(cube.get(CubeFace::U), CubeFace::U);
    // assert_eq!(cube.get(CubeFace::D), CubeFace::D);
}
