use std::collections::HashSet;
use std::time::Instant;
use std::fs;
fn main() {
    let start = Instant::now();
    let mut x_cord = 0;
    let mut y_cord = 0;
    let contents = fs::read_to_string("/home/corbin/Desktop/rust_projects/Private/Advent_of_code/2015/day_three/day_three_part_one/src/puzzle_input.txt").expect("was supposed to open puzzle input");
    let mut house_cords: HashSet<(i32, i32)> = HashSet::new();
    house_cords.insert((0, 0));
    for i in contents.chars(){
        if i == '<'{
            x_cord -= 1
        }
        else if i == '>'{
            x_cord += 1
        }
        else if i == 'v'{
            y_cord -= 1
        }
        else if i == '^'{
            y_cord += 1
        }
        house_cords.insert((x_cord, y_cord));
    }
    println!("answer is {}", house_cords.len());
    println!("execution time was {} microsecounds", start.elapsed().as_micros())
}
