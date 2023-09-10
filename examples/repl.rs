use rubik::prelude::*;
#[allow(dead_code)]
pub fn print_rubik(rubik: &Rubik) {
    use colored::Colorize;
    let color_map = RubikColor::classic_map();
    fn print_color(c: RubikColor, aligned: bool) {
        let block = if aligned { "[]" } else { "  " };
        let block = match c {
            RubikColor::White => block.on_white(),
            RubikColor::Yellow => block.on_yellow(),
            RubikColor::Red => block.on_red(),
            RubikColor::Orange => block.on_magenta(),
            RubikColor::Blue => block.on_blue(),
            RubikColor::Green => block.on_green(),
        };
        print!("{block}");
    }
    println!("====================");
    let mut counter = 0;
    for cube in rubik.iter_by_layer(&RubikLayer::U) {
        if counter % 3 == 0 {
            print!("{:6}", "");
        }
        let color = color_map[cube.get(CubeFace::U)];
        print_color(color, rubik.is_aligned(cube));
        counter += 1;
        if counter % 3 == 0 {
            println!();
        }
    }
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
            if counter % 3 == 0 && block_cnt != 0 {
                // move to line's end
                print!("\x1B[{}C", block_cnt * 6);
            }
            let color = color_map[cube.get(face)];
            print_color(color, rubik.is_aligned(cube));
            counter += 1;
            if counter % 3 == 0 {
                println!();
            }
        }
        print!("\x1B[3A");
    }
    print!("\x1B[3B");
    counter = 0;
    for cube in rubik.iter_by_layer(&RubikLayer::D) {
        if counter % 3 == 0 {
            print!("{:6}", "");
        }
        let color = color_map[cube.get(CubeFace::D)];
        print_color(color, rubik.is_aligned(cube));
        counter += 1;
        if counter % 3 == 0 {
            println!();
        }
    }
    println!("====================");
}

pub fn main() {
    let mut rubik = Rubik::new();
    let mut input = String::new();
    loop {
        print_rubik(&rubik);
        std::io::stdin().read_line(&mut input).unwrap();
        if input.starts_with("/reset") {
            rubik.reset();
        } else if input.starts_with("/exit") {
            break;
        } else if input.starts_with("/shuffle") {
            rubik.shuffle(64);
        } else {
            match rubik::parser::singmaster::parse(input.trim()) {
                Ok(transform) => {
                    rubik.execute(&transform);
                }
                Err(e) => {
                    println!("Invalid input: {e}");
                }
            }
        }
        input.clear();
    }
}
