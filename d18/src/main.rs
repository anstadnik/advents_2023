mod parse;
mod structs;
mod tests;

use anyhow::Result;
use indicatif::ParallelProgressIterator;
use parse::Step;
use rayon::prelude::IntoParallelIterator;
use rayon::prelude::ParallelIterator;
use std::fs;
use structs::Direction::*;
use structs::Position;
type PosStep<T> = (Position<T>, Step);

fn intersection((x1, x2): (u64, u64), walls: &[(u64, u64)]) -> u64 {
    walls
        .iter()
        .filter_map(|&(x1_, x2_)| Some(x2_.min(x2 - 1).checked_sub(x1_.max(x1 + 1))? + 1))
        .sum()
}

fn get_n_tiles_inside(h: u64, path: &[PosStep<u64>]) -> u64 {
    let mut horizontal_walls = vec![Vec::new(); h as usize];
    for &(Position { x, y }, (direction, l)) in path {
        match direction {
            Right => horizontal_walls[y as usize].push((x, x + l)),
            Left => horizontal_walls[y as usize].push((x - l, x)),
            _ => {}
        }
    }

    (0..h)
        .into_par_iter()
        .progress_count(h)
        .map(|y| {
            let mut walls: Vec<_> = path
                .iter()
                .filter_map(
                    |&(Position { x: x_, y: y_ }, (direction, l))| match direction {
                        Up if y_ >= y && y > y_ - l => Some(x_),
                        Down if y_ + l >= y && y > y_ => Some(x_),
                        _ => None,
                    },
                )
                .collect();
            walls.sort_unstable();
            walls
                .chunks(2)
                .map(|chunk| (chunk[0], chunk[1]))
                .map(|(a, b)| b - a - 1 - intersection((a, b), &horizontal_walls[y as usize]))
                .sum::<u64>()
        })
        .sum()
}

fn gen_positions(steps: &[Step]) -> Vec<PosStep<i64>> {
    steps
        .iter()
        .scan(Position::new(0, 0), |pos, &(d, l)| {
            let pos_ = *pos;
            (0..l).for_each(|_| *pos += d);
            Some((pos_, (d, l)))
        })
        .collect()
}

fn flatten_positions(positions: &[PosStep<u64>]) -> impl Iterator<Item = PosStep<u64>> + '_ {
    positions.iter().flat_map(|&(pos, (d, l))| {
        (0..l).scan(pos, move |pos, _| {
            *pos += d;
            Some((*pos, (d, 1)))
        })
    })
}

fn offset_positions(positions: &[PosStep<i64>]) -> Result<Vec<PosStep<u64>>> {
    let (min_x, min_y) = positions
        .iter()
        .fold((0, 0), |(mx, my), (Position { x, y }, _)| {
            (mx.min(*x), my.min(*y))
        });
    positions
        .iter()
        .map(|&(Position { x, y }, step)| {
            Ok((
                Position::new((x - min_x).try_into()?, (y - min_y).try_into()?),
                step,
            ))
        })
        .collect()
}

fn gen_field_size(positions: &[PosStep<u64>]) -> (u64, u64) {
    positions
        .iter()
        .fold((0, 0), |(mx, my), &(p, _)| (mx.max(p.x), my.max(p.y)))
}

fn task(steps: &[Step]) -> Result<u64> {
    let positions = gen_positions(steps);
    let positions = offset_positions(&positions)?;
    let (_, h) = gen_field_size(&positions);
    Ok(flatten_positions(&positions).count() as u64 + get_n_tiles_inside(h + 1, &positions))
}

fn main() -> Result<()> {
    let s = fs::read_to_string("input.txt")?;

    println!("Task 1: {}", task(&parse::parse_input1(&s)?)?);
    println!("Task 2: {}", task(&parse::parse_input2(&s)?)?);

    Ok(())
}
