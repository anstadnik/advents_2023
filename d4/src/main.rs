use std::fs;

fn calculate_n_matches(mut card: &str) -> i32 {
    card = card.split_once(": ").unwrap().1;
    let (winning, available) = card.split_once(" | ").unwrap();
    let winning: Vec<i32> = winning
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let available: Vec<i32> = available
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    available
        .into_iter()
        .filter(|x| winning.contains(x))
        .count() as i32
}

fn main() {
    let s = fs::read_to_string("input.txt").expect("Error reading file");

    let cards_scores: Vec<i32> = s.lines().map(calculate_n_matches).collect();

    let task_1 = cards_scores
        .iter()
        .filter(|x| **x > 0)
        .map(|x| 2_i32.pow(*x as u32 - 1))
        .sum::<i32>();

    println!("The answer for task 1 is {}", task_1);

    let mut dp = vec![0; cards_scores.len()];
    for i in (0..cards_scores.len()).rev() {
        let n = cards_scores[i] as usize;
        dp[i] = (i + 1..i + n + 1).map(|x| dp[x]).sum::<i32>() + 1;
    }

    println!("The answer for task 2 is {}", dp.into_iter().sum::<i32>());
}
