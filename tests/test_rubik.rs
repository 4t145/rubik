use rubik::*;

pub fn print_rubik(rubik: &Rubik) {
    use colored::Colorize;
    let color_map = RubikColor::classic_map();
    fn print_color(c: RubikColor) {
        match c {
            RubikColor::White => print!("{} ", "■".white()),
            RubikColor::Yellow => print!("{} ", "■".yellow()),
            RubikColor::Red => print!("{} ", "■".red()),
            RubikColor::Orange => print!("{} ", "■".magenta()),
            RubikColor::Blue => print!("{} ", "■".blue()),
            RubikColor::Green => print!("{} ", "■".green()),
        }
    }
    println!("====================");
    let mut counter = 0;
    for cube in rubik.iter_by_layer(&RubikLayerTransform::U) {
        if counter % 3 == 0 {
            print!("\t");
        }
        let color = color_map[cube.get(CubeFace::U)];
        print_color(color);
        counter += 1;
        if counter % 3 == 0 {
            println!();
        }
    }
    println!();
    for (block_cnt, (layer, face)) in [
        (&RubikLayerTransform::L, CubeFace::L),
        (&RubikLayerTransform::F, CubeFace::F),
        (&RubikLayerTransform::R, CubeFace::R),
        (&RubikLayerTransform::B, CubeFace::B),
    ]
    .into_iter()
    .enumerate()
    {
        counter = 0;
        for cube in rubik.iter_by_layer(layer) {
            if counter % 3 == 0 {
                // move to line's end
                print!("\x1B[{}C", block_cnt * 8);
            }
            let color = color_map[cube.get(face)];
            print_color(color);
            counter += 1;
            if counter % 3 == 0 {
                println!();
            }
        }
        print!("\x1B[3A");
    }
    print!("\x1B[3B");
    println!();
    counter = 0;
    for cube in rubik.iter_by_layer(&RubikLayerTransform::D) {
        if counter % 3 == 0 {
            print!("\t");
        }
        let color = color_map[cube.get(CubeFace::D)];
        print_color(color);
        counter += 1;
        if counter % 3 == 0 {
            println!();
        }
    }
    println!("====================");
}

#[test]
pub fn test_rubik() {
    let mut rubik = Rubik::new();
    use rubik::operation::*;
    let op = [F, R, U, R_, U_, F_];
    print_rubik(rubik.execute(op));
    print_rubik(rubik.execute(op));
    print_rubik(rubik.execute(op));
}