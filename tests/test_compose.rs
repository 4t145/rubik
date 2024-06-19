use std::collections::HashSet;

use rubik::prelude::*;

#[test]
fn test_compose() {
    let unit = CubePermutation::new(0b11100100).unwrap();
    assert_eq!(unit.compose(unit), unit);
    let a = CubePermutation::new(0b10010011).unwrap();
    assert_eq!(a * unit, a);
    assert_eq!(unit * a, a);
    assert_eq!(
        CubePermutation::BACK * CubePermutation::FRONT,
        CubePermutation::UNIT
    );
    assert_eq!(
        CubePermutation::RIGHT * CubePermutation::LEFT,
        CubePermutation::UNIT
    );
    assert_eq!(
        CubePermutation::DOWN * CubePermutation::UP,
        CubePermutation::UNIT
    );

    assert_eq!(
        CubePermutation::RIGHT
            * CubePermutation::RIGHT
            * CubePermutation::DOWN
            * CubePermutation::DOWN
            * CubePermutation::FRONT
            * CubePermutation::FRONT,
        CubePermutation::UNIT
    );
    assert_eq!(
        CubePermutation::RIGHT * CubePermutation::DOWN,
        CubePermutation::BACK * CubePermutation::RIGHT
    );
}

#[test]
fn test_sub_group() {
    // group N (normal subgroup)
    let group_n = [
        CubePermutation::X_2,
        CubePermutation::Y_2,
        CubePermutation::Z_2,
        CubePermutation::UNIT,
    ];
    for x in group_n {
        for y in group_n {
            assert_eq!(x * y, y * x);
            assert!(group_n.contains(&(x * y)))
        }
        for s in CubePermutation::enumerate() {
            assert!(group_n.contains(&(s * x * s.inverse())));
        }
    }
    assert_eq!(CubePermutation::X_2.square(), CubePermutation::UNIT);
    assert_eq!(CubePermutation::Y_2.square(), CubePermutation::UNIT);
    assert_eq!(CubePermutation::Z_2.square(), CubePermutation::UNIT);
    assert_eq!(
        CubePermutation::X_2 * CubePermutation::Y_2,
        CubePermutation::Z_2
    );
    assert_eq!(
        CubePermutation::X_2 * CubePermutation::Y_2,
        CubePermutation::Z_2
    );
    assert_eq!(
        CubePermutation::Y_2 * CubePermutation::Z_2,
        CubePermutation::X_2
    );
    assert_eq!(
        CubePermutation::Z_2 * CubePermutation::X_2,
        CubePermutation::Y_2
    );

    let coset = |s: CubePermutation| {
        group_n
            .iter()
            .map(move |n| (s).compose(*n))
            .collect::<HashSet<_>>()
    };

    let quotient_group =
        CubePermutation::enumerate()
            .map(coset)
            .fold(Vec::new(), |mut set, coset| {
                if !set.contains(&coset) {
                    set.push(coset)
                }
                set
            });
    assert_eq!(quotient_group.len(), 6);
    let quotient_group = quotient_group
        .iter()
        .filter_map(|x| x.iter().find(|s| s.into_inner() & 0b11 == 0))
        .collect::<Vec<_>>();
    dbg!(&quotient_group);
    println!(
        "{:?}{:?}",
        &CubePermutation::Z_1.factor_2().0,
        &CubePermutation::Z_1.factor_2().1
    );
    println!(
        "{:?}{:?}",
        &CubePermutation::Z_3.factor_2().0,
        &CubePermutation::Z_3.factor_2().1
    );
}

#[test]
fn test_factor_3() {
    let f0_set = [
        CubePermutation::UNIT,
        CubePermutation::X_2,
        CubePermutation::Y_2,
        CubePermutation::Z_2,
    ];
    let f1_set = [
        CubePermutation::UNIT,
        CubePermutation::C1,
        CubePermutation::C2,
    ];
    let f2_set = [CubePermutation::UNIT, CubePermutation::I];

    for p in CubePermutation::enumerate() {
        let (f0, f1, f2) = p.factor_3();
        println!("{:?} => {:?} x {:?} x {:?}", p, f0, f1, f2);
        assert!(f0_set.contains(&f0));
        assert!(f1_set.contains(&f1));
        assert!(f2_set.contains(&f2));
    }
}
