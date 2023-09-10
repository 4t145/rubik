mod print_rubik;



// #[cfg(singmaster)]
#[test]
fn test_singmaster_compiler() {
    use rubik::prelude::*;
    use print_rubik::print_rubik;
    use rubik::parser::singmaster::parse;
    print_rubik(Rubik::new().execute(&parse("BL E2(RR'F'R2 F2' F'2 )3'RR B").unwrap()));
}