use std::fs;
use std::time::Instant;
fn main() {
    let start = Instant::now();
    let mut total_rapping_paper_required = 0;
    let contents = fs::read_to_string("puzzle_input.txt").expect("could not find file");
    let contents: Vec<&str> = contents.split("\n").collect();
    for dimensions in contents{
        let dimensions: Vec<&str> = dimensions.split("x").collect();
        let l: i32 = dimensions.get(0).unwrap().parse::<i32>().unwrap();
        let w: i32 = dimensions.get(1).unwrap().parse::<i32>().unwrap();
        let h: i32 = dimensions.get(2).unwrap().parse::<i32>().unwrap();
        total_rapping_paper_required += 2 * l * w;
        total_rapping_paper_required += 2 * w * h;
        total_rapping_paper_required += 2 * h * l;
        let dimensions: Vec<i32> = vec![l * w, w * h, h * l];
        total_rapping_paper_required +=dimensions.iter().min().unwrap()

    }
    println!("{} total wrapping paper", total_rapping_paper_required);
    println!("{} microsecounds have elapsed", start.elapsed().as_micros())
    }