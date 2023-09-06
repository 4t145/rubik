use rubik::{prelude::*, tf};
mod print_rubik;
use print_rubik::print_rubik;

#[test]
fn test_singmaster_compiler() {
    use rubik::parser::singmaster::parse;
    print_rubik(Rubik::new().execute(parse("BLE2(RR'F'R2F2'F'2)3'RRB").unwrap()));
}
