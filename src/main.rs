#![feature(test)]
extern crate test;

use crate::days::day1::{puzzle1, puzzle2};
use crate::days::day2::puzzle3;

#[allow(dead_code)]
pub mod days;

fn main() {
    let val1 = puzzle1().map_or_else(|err_msg| err_msg.to_string(), |val| val.to_string());
    println!("Puzzle 1 answer: {}", val1);

    let val2 = puzzle2().map_or_else(|err_msg| err_msg.to_string(), |val| val.to_string());
    println!("Puzzle 2 answer: {}", val2);

    let val3 = puzzle3().map_or_else(|err_msg| err_msg.to_string(), |val| val.to_string());
    println!("Puzzle 3 answer: {}", val3);
}
