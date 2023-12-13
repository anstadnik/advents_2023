use anyhow::Result;
use ndarray::prelude::*;
use std::fs;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Reflection {
    Row(usize),
    Col(usize),
    None,
}

fn find_reflections(b: ArrayView2<char>, ignore: Reflection) -> Reflection {
    let get_it = |i, len| (i + 1..len).zip((0..=i).rev());
    let (n, m) = (b.nrows(), b.ncols());
    let row = (0..n - 1)
        .filter(|&i| !matches!(ignore, Reflection::Row(j) if i + 1 == j))
        .find(|&i| get_it(i, n).all(|(i, j)| b.row(i) == b.row(j)));
    let col = (0..m - 1)
        .filter(|&i| !matches!(ignore, Reflection::Col(j) if i + 1 == j))
        .find(|&j| get_it(j, m).all(|(i, j)| b.column(i) == b.column(j)));

    match (row, col) {
        (None, None) => Reflection::None,
        (Some(i), None) => Reflection::Row(i + 1),
        (None, Some(i)) => Reflection::Col(i + 1),
        (Some(_), Some(_)) => unreachable!(),
    }
}

fn task2(mut b: ArrayViewMut2<char>) -> Reflection {
    let alt = |c| if c == '#' { '.' } else { '#' };
    let to_ignore = find_reflections(b.view(), Reflection::None);

    for i in 0..b.nrows() {
        for j in 0..b.ncols() {
            b[[i, j]] = alt(b[[i, j]]);
            let ans = find_reflections(b.view(), to_ignore);
            b[[i, j]] = alt(b[[i, j]]);
            if let reflection @ (Reflection::Row(_) | Reflection::Col(_)) = ans {
                return reflection;
            };
        }
    }

    unreachable!()
}

fn parse_line(block: &str) -> Result<Array2<char>> {
    let (n, m) = (block.lines().count(), block.lines().next().unwrap().len());
    let v = block.lines().flat_map(|line| line.chars()).collect();
    Ok(Array2::from_shape_vec((n, m), v)?)
}

fn main() -> Result<()> {
    let s = fs::read_to_string("input.txt")?;
    let mut blocks: Vec<_> = s.split("\n\n").map(parse_line).collect::<Result<_>>()?;

    let f = |reflection| match reflection {
        Reflection::Row(i) => i * 100,
        Reflection::Col(i) => i,
        Reflection::None => unreachable!(),
    };
    let ans1: usize = blocks
        .iter()
        .map(|a| find_reflections(a.view(), Reflection::None))
        .map(f)
        .sum();

    let ans2: usize = blocks.iter_mut().map(|a| task2(a.view_mut())).map(f).sum();

    println!("Part 1: {}", ans1);
    println!("Part 2: {}", ans2);

    Ok(())
}

mod tests;
