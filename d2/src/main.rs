use std::{fs, str::FromStr};

mod game;
use game::{Balls, Game};

fn task_1(games: &[Game]) -> u32 {
    let available_balls = &Balls {
        r: 12,
        g: 13,
        b: 14,
    };
    let is_possible = |b: &&Game| {
        b.balls.iter().all(|b| {
            b.r <= available_balls.r && b.g <= available_balls.g && b.b <= available_balls.b
        })
    };
    games.iter().filter(is_possible).map(|g| g.id).sum()
}

fn task_2(games: &[Game]) -> u32 {
    games
        .iter()
        .map(|g| {
            g.balls.iter().fold(Balls::default(), |acc, b| Balls {
                r: acc.r.max(b.r),
                g: acc.g.max(b.g),
                b: acc.b.max(b.b),
            })
        })
        .map(|b| b.r * b.g * b.b)
        .sum()
}

fn main() -> anyhow::Result<()> {
    let s = fs::read_to_string("input.txt").unwrap();
    let games = s
        .lines()
        .map(FromStr::from_str)
        .collect::<anyhow::Result<Vec<game::Game>>>()?;
    println!("Answer 1: {}", task_1(&games));
    println!("Answer 2: {}", task_2(&games));
    Ok(())
}
