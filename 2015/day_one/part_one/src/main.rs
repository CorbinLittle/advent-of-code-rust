use std::fs;
use std::time::Instant;
fn main() {
    let start = Instant::now();
    println!("In file {}", "input.txt");

    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let mut floor_counter = 0;
    for i in contents.chars() {
        if i == '(' {
            floor_counter += 1
        }
        else if  i == ')' {
            floor_counter -= 1
        }
    }
    println!("answer is {}", floor_counter);
    println!("it took {} microsecounds to execute", start.elapsed().as_micros())
}