mod direction;
mod tests;
use anyhow::Result;
use std::{collections::VecDeque, fs};

use direction::*;
use Direction::*;
type Field = Vec<Vec<u32>>;
type Memory = Vec<Vec<States>>;
type PosState = (Position, State);

fn parse_field(s: &str) -> Option<Field> {
    s.lines()
        .map(|l| l.chars().map(|c| c.to_digit(10)).collect())
        .collect()
}

fn step<'a>(
    field: &'a Field,
    memory: &'a mut Memory,
    (mut pos, mut state): PosState,
    min_m: u32,
    max_m: u32,
) -> Option<impl Iterator<Item = PosState> + 'a> {
    if memory[pos.y][pos.x][state.direction] <= state {
        return None;
    }
    memory[pos.y][pos.x][state.direction] = state;
    let new_directions = match state.direction {
        Up | Down => [Left, Right],
        Left | Right => [Up, Down],
    };
    let update = |mut p: Position, State { direction, cost }| {
        p += direction;
        Some((p, State::new(direction, cost + field.get(p.y)?.get(p.x)?)))
    };
    (pos, state) = (0..min_m).try_fold((pos, state), |(p, state), _| update(p, state))?;
    let f = move |_| {
        (pos, state) = update(pos, state)?;
        Some(new_directions.map(|direction| (pos, State { direction, ..state })))
    };
    Some((0..max_m - min_m).filter_map(f).flatten())
}

fn bfs(field: &Field, memory: &mut Memory, mut q: VecDeque<PosState>, min_m: u32, max_m: u32) {
    while let Some((pos, state)) = q.pop_front() {
        if let Some(iter) = step(field, memory, (pos, state), min_m, max_m) {
            q.extend(iter);
        }
    }
}

fn solve(field: &Field, min_m: u32, max_m: u32) -> u32 {
    let (n, m) = (field.len(), field[0].len());
    let mut mem = vec![vec![States::new(u32::MAX); m]; n];
    let q = [Right, Down].map(|d| (Position::new(0, 0), State::new(d, 0)));
    bfs(field, &mut mem, q.into(), min_m, max_m);
    mem[n - 1][m - 1].min()
}

fn task1(field: &Field) -> u32 {
    solve(field, 0, 3)
}

fn task2(field: &Field) -> u32 {
    solve(field, 3, 10)
}

fn main() -> Result<()> {
    let s = fs::read_to_string("input.txt")?;
    let field = parse_field(&s).ok_or(anyhow::anyhow!("Invalid input"))?;

    println!("Task 1: {}", task1(&field));
    println!("Task 2: {}", task2(&field));

    Ok(())
}
