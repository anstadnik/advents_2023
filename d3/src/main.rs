mod task_1;
use regex::Regex;
use std::fs;
use task_1::task_1;

fn main() {
    let s = fs::read_to_string("input.txt").unwrap();

    println!("The answer to task 1 is {}", task_1(&s));

    println!("The answer to task 2 is {}", task_2(s));
}

fn task_2(s: String) -> usize {
    let s: Vec<String> = s.lines().map(|line| line.to_string()).collect();
    let regex = Regex::new(r"(\d+)").unwrap();
    let regex_ = &regex;
    let s_ = &s;
    s.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '*')
                .map(move |(x, _)| {
                    [y - 1, y, y + 1]
                        .iter()
                        .filter_map(|&y| s_.get(y))
                        .flat_map(|line_| {
                            regex_
                                .find_iter(line_)
                                .filter(|m| x >= m.start().saturating_sub(1) && x <= m.end())
                                .map(|m| m.as_str().parse::<usize>().unwrap())
                        })
                        .collect::<Vec<_>>()
                })
                .filter(|nums| nums.len() == 2)
                .map(|nums| nums[0] * nums[1])
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_2() {
        let s = "1...\n\
                 .*..\n\
                 ..2.\n\
                 ....\n";
        assert_eq!(task_2(s.to_string()), 2);
    }
}
