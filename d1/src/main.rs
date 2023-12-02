mod task_1;
mod task_2;
use crate::{task_1::task_1, task_2::task_2};
use std::fs;

fn main() {
    let s = fs::read_to_string("input.txt").expect("Error reading input.txt");
    println!("The answer to task 1 is {}", task_1(&s));
    println!("The answer to task 2 is {}", task_2(&s));
}
