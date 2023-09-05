use rubik::CubePermutation;

#[test]
fn test_compose() {
    let unit = CubePermutation::new(0b11100100).unwrap();
    assert_eq!(unit.compose(unit), unit);
    let a = CubePermutation::new(0b10010011).unwrap();
    assert_eq!(a + unit, a);
    assert_eq!(unit + a, a);
    assert_eq!(
        CubePermutation::BACK + CubePermutation::FRONT,
        CubePermutation::UNIT
    );
    assert_eq!(
        CubePermutation::RIGHT + CubePermutation::LEFT,
        CubePermutation::UNIT
    );
    assert_eq!(
        CubePermutation::DOWN + CubePermutation::UP,
        CubePermutation::UNIT
    );

    assert_eq!(
        CubePermutation::RIGHT
            + CubePermutation::RIGHT
            + CubePermutation::DOWN
            + CubePermutation::DOWN
            + CubePermutation::FRONT
            + CubePermutation::FRONT,
        CubePermutation::UNIT
    );
    assert_eq!(
        CubePermutation::RIGHT + CubePermutation::DOWN,
        CubePermutation::BACK + CubePermutation::RIGHT
    );
}
