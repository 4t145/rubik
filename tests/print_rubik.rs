use rubik::{prelude::*, tf};
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
    for cube in rubik.iter_by_layer(&RubikLayer::U) {
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
        (&RubikLayer::L, CubeFace::L),
        (&RubikLayer::F, CubeFace::F),
        (&RubikLayer::R, CubeFace::R),
        (&RubikLayer::B, CubeFace::B),
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
    for cube in rubik.iter_by_layer(&RubikLayer::D) {
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
