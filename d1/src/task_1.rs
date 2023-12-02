pub(crate) fn task_1(s: &str) -> u32 {
    s.lines()
        .map(|line| {
            (
                line.chars().find_map(|c| c.to_digit(10)).unwrap(),
                line.chars().rev().find_map(|c| c.to_digit(10)).unwrap(),
            )
        })
        .fold(0, |acc, (a, b)| acc + a * 10 + b)
}
