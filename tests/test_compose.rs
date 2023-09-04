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
        CubePermutation::UP + CubePermutation::DOWN,
        CubePermutation::UNIT
    );
    assert_eq!(
        CubePermutation::RIGHT + CubePermutation::LEFT,
        CubePermutation::UNIT
    );

    assert_eq!(
        CubePermutation::UP
            + CubePermutation::UP
            + CubePermutation::RIGHT
            + CubePermutation::RIGHT
            + CubePermutation::FRONT
            + CubePermutation::FRONT,
        CubePermutation::UNIT
    );
    assert_eq!(
        CubePermutation::UP + CubePermutation::RIGHT,
        CubePermutation::BACK + CubePermutation::UP
    );
}
