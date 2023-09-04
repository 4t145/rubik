use rubik::*;

#[test]
pub fn test_cube() {
    let mut cube = Cube::new();
    dbg!(&cube);
    cube.rotate(CubePermutation::UP);
    dbg!(&cube);
    cube.rotate(CubePermutation::UP);
    dbg!(&cube);
}
