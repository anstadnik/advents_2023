use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let s = fs::read_to_string("input.txt")?;
    let field: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();
    let (_, m) = (field.len(), field[0].len());

    let rs: Vec<_> = field
        .iter()
        .enumerate()
        .filter(|&(_, row)| !row.contains(&'#'))
        .map(|(i, _)| i)
        .collect();

    let cs: Vec<_> = (0..m)
        .filter(|&j| field.iter().all(|row| row[j] == '.'))
        .collect();

    let galaxies: Vec<_> = field
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, &c)| c == '#')
                .map(move |(x, _)| (x, y))
        })
        .collect();

    let galaxies_pairs = galaxies.iter().enumerate().flat_map(|(i, &(x1, y1))| {
        galaxies
            .iter()
            .skip(i + 1)
            .map(move |&(x2, y2)| (x1.min(x2), y1.min(y2), x1.max(x2), y1.max(y2)))
    });
    let ans1 = galaxies_pairs
        .clone()
        .map(|(x1, y1, x2, y2)| {
            let dist = x2 - x1 + y2 - y1;
            let expanded_dist = rs.iter().filter(|&y| y1 <= *y && *y <= y2).count()
                + cs.iter().filter(|&x| x1 <= *x && *x <= x2).count();
            dist + expanded_dist
        })
        .sum::<usize>();

    let ans2 = galaxies_pairs
        .map(|(x1, y1, x2, y2)| {
            let dist = x2 - x1 + y2 - y1;
            let expanded_dist = rs.iter().filter(|&y| y1 <= *y && *y <= y2).count()
                + cs.iter().filter(|&x| x1 <= *x && *x <= x2).count();
            dist + expanded_dist * 999999
        })
        .sum::<usize>();

    println!("Answer to task 1: {}", ans1);
    println!("Answer to task 2: {}", ans2);

    Ok(())
}
