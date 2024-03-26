use std::collections::HashSet;
use std::time::Instant;
use std::fs;
fn main() {
    let start = Instant::now();
    let mut santa_x_cord = 0;
    let mut santa_y_cord = 0;
    let mut robo_santa_x_cord = 0;
    let mut robo_santa_y_cord = 0;
    let mut is_santas_turn = true;
    let contents = fs::read_to_string("/home/corbin/Desktop/rust_projects/Private/Advent_of_code/2015/day_three/day_three_part_two/src/test_input.txt").expect("was supposed to open puzzle input");
    let mut house_cords: HashSet<(i32, i32)> = HashSet::new();
    house_cords.insert((0, 0));
    for i in contents.chars(){
        if is_santas_turn {
            if i == '<'{
                santa_x_cord -= 1
            }
            else if i == '>'{
                santa_x_cord += 1
            }
            else if i == 'v'{
                santa_y_cord -= 1
            }
            else if i == '^'{
                santa_y_cord += 1
            }
            house_cords.insert((santa_x_cord, santa_y_cord));
        }
        else {
            if i == '<'{
                robo_santa_x_cord -= 1
            }
            else if i == '>'{
                robo_santa_x_cord += 1
            }
            else if i == 'v'{
                robo_santa_y_cord -= 1
            }
            else if i == '^'{
                robo_santa_y_cord += 1
            }
            house_cords.insert((robo_santa_x_cord, robo_santa_y_cord));
        }
        is_santas_turn = !is_santas_turn
    }
    println!("answer is {}", house_cords.len());
    println!("execution time was {} microsecounds", start.elapsed().as_micros())
}