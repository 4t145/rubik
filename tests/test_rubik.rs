use rubik::{prelude::*, tf};
mod print_rubik;
use print_rubik::print_rubik;
#[test]
pub fn test_rubik() {
    let mut rubik = Rubik::new();
    print_rubik(rubik.execute(&tf!((R, U, RI, UI, RI, F, R, FI); 10)));
    print_rubik(rubik.execute(&tf!(F, U, R, UI, RI, FI)));
}
