use anyhow::Result;
use std::fs;

// (time - hold) * hold > distance
// time * hold - hold * hold > distance
//
// Lol who needs math if rust is so fast

fn parse_line1(line: &str) -> Vec<i64> {
    line.split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn task_1(s: &str) -> usize {
    let mut it = s.lines();
    let times = parse_line1(it.next().unwrap());
    let distances = parse_line1(it.next().unwrap());

    calculate(&times, &distances)
}

fn parse_line2(line: &str) -> i64 {
    line.split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .collect::<String>()
        .parse()
        .unwrap()
}

fn task_2(s: &str) -> usize {
    let mut it = s.lines();
    let times = vec![parse_line2(it.next().unwrap())];
    let distances = vec![parse_line2(it.next().unwrap())];

    calculate(&times, &distances)
}

fn calculate(times: &[i64], distances: &[i64]) -> usize {
    times
        .iter()
        .zip(distances)
        .map(|(&t, &d)| (0..=t).filter(|hold| hold * (t - hold) > d).count())
        .product::<usize>()
}

fn main() -> Result<()> {
    let s = fs::read_to_string("input.txt")?;

    println!("Part 1: {}", task_1(&s));
    println!("Part 2: {}", task_2(&s));

    Ok(())
}
