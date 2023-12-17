mod tests;
use anyhow::Result;
use indicatif::ProgressIterator;
use std::fs;

use ndarray::prelude::*;

fn parse_block(block: &str) -> Result<Array2<char>> {
    let (n, m) = (block.lines().count(), block.lines().next().unwrap().len());
    let v = block.lines().flat_map(|line| line.chars()).collect();
    Ok(Array2::from_shape_vec((n, m), v)?)
}

fn move_rock_north(block: &mut Array2<char>, i: usize, j: usize) {
    let i_ = block
        .column(j)
        .iter()
        .enumerate()
        .take(i)
        .rev()
        .find(|(_, &c)| c == '#' || c == 'O')
        .map_or_else(|| 0, |(i_, _)| i_ + 1);

    block.swap((i, j), (i_, j));
}

fn move_rocks_north(block: &mut Array2<char>) {
    let (n, m) = block.dim();
    for i in 0..n {
        for j in 0..m {
            if block[(i, j)] == 'O' {
                move_rock_north(block, i, j);
            }
        }
    }
}

fn calculate_load(block: ArrayView2<char>) -> usize {
    let n = block.nrows();
    let f = |(i, row): (usize, ArrayView1<char>)| {
        row.into_iter().filter(|&&c| c == 'O').count() * (n - i)
    };
    block.rows().into_iter().enumerate().map(f).sum()
}

fn task1(mut block: Array2<char>) -> usize {
    move_rocks_north(&mut block);
    calculate_load(block.view())
}

fn rot90(arr: &mut Array2<char>) {
    // arr.swap_axes(0, 1);
    // arr.invert_axis(Axis(0));
    arr.invert_axis(Axis(0));
    arr.swap_axes(0, 1);
}

fn cycle(block: &mut Array2<char>) {
    for _ in 0..4 {
        move_rocks_north(block);
        // println!("{}", block);
        rot90(block);
        // println!("{}", block);
    }
}

fn task2(mut block: Array2<char>, n: usize) -> usize {
    // let mut prev = block.clone();
    for _ in (0..n).progress() {
        cycle(&mut block);
        // println!("{}", block);
        // break;
        // if block == prev {
        //     break;
        // }
        // prev = block.clone();
        println!("{}", calculate_load(block.view()));
    }
    calculate_load(block.view())
}

fn main() -> Result<()> {
    let s = fs::read_to_string("input.txt")?;
    let block = parse_block(&s)?;

    // println!("Part 1: {}", task1(block.clone()));
    // println!("Part 2: {}", task2(block.clone()));
    task2(block.clone(), 10000);

    Ok(())
}
