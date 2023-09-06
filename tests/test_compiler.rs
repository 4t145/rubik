use rubik::{prelude::*, tf};
mod print_rubik;
use print_rubik::print_rubik;

#[test]
fn test_singmaster_compiler() {
    use rubik::parser::singmaster::*;
    let output = parse("RUR'U'R'FRF'").unwrap();
    let tfg = RubikTransformGroup::from(output.as_slice());
    print_rubik(Rubik::new().execute(tfg));
}