use std::fs;
use std::time::Instant;
fn main() {
    let start = Instant::now();
    let mut total_ribbon_required = 0;
    let contents = fs::read_to_string("puzzle_input.txt").expect("could not find file");
    let contents: Vec<&str> = contents.split("\n").collect();
    for dimensions in contents{
        let dimensions: Vec<&str> = dimensions.split("x").collect();
        let l: i32 = dimensions.get(0).unwrap().parse::<i32>().unwrap();
        let w: i32 = dimensions.get(1).unwrap().parse::<i32>().unwrap();
        let h: i32 = dimensions.get(2).unwrap().parse::<i32>().unwrap();
        let double_l = 2 * l;
        let double_w = 2 * w;
        let double_h = 2 * h;

        total_ribbon_required += double_l;
        total_ribbon_required += double_w;
        total_ribbon_required += double_h;
        total_ribbon_required += l * w * h;
        let dimensions: Vec<i32> = vec![double_l, double_w, double_h];
        total_ribbon_required -= dimensions.iter().max().unwrap()

    }
    println!("{} total ribbon paper", total_ribbon_required);
    println!("{} microsecounds have elapsed", start.elapsed().as_micros())
    }