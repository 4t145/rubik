#[test]
fn test_singmaster_compiler() {
    use rubik::parser::singmaster::*;
    let output = parse("BLE2(RR'F'R2F2'F'2)3'RRB").unwrap();
    dbg!(output);
}