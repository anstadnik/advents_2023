mod task_1;
mod task_2;
use std::fs;
use task_1::task_1;
use task_2::task_2;

fn main() {
    let s = fs::read_to_string("input.txt").unwrap();

    println!("The answer to task 1 is {}", task_1(&s));
    println!("The answer to task 2 is {}", task_2(&s));
}
