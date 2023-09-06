# A CAN'T EVEN BETTER IMPLEMENTATION FOR RUBIK'S CUBE
```RUST
use rubik::parser::singmaster::parse;
let transform = parse("BLE2(RR'F'R2F2'F'2)3'RRB").unwrap();
Rubik::new().execute(transform);
```