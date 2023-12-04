use crate::days::day1::puzzle1;

#[allow(dead_code)]
pub mod days;

fn main() {
    let val1 = puzzle1().map_or_else(|err_msg| err_msg.to_string(), |val| val.to_string());

    println!("Puzzle 1 answer: {}", val1);
}
