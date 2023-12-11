#[cfg(test)]
mod tests;
use anyhow::Result;
use std::fs;

fn follow_the_loop(field: &[Vec<char>], start: [(i32, i32); 2]) -> Option<Vec<(i32, i32)>> {
    let mut cur = start;
    let mut path = vec![start[0]];
    while cur[1] != start[0] {
        path.push(cur[1]);
        let [(x1, y1), (x2, y2)] = cur;

        let (x2_, y2_) = match (x2 - x1, y2 - y1, field.get(y2 as usize)?.get(x2 as usize)?) {
            // Up
            (0, -1, '|') => (x2, y2 - 1),
            // Down
            (0, 1, '|') => (x2, y2 + 1),

            // Vertical turn
            (0, 1, 'J') | (0, -1, '7') => (x2 - 1, y2),
            (0, 1, 'L') | (0, -1, 'F') => (x2 + 1, y2),

            // Right
            (1, 0, '-') => (x2 + 1, y2),
            // Left
            (-1, 0, '-') => (x2 - 1, y2),

            // Horizontal turn
            (1, 0, '7') | (-1, 0, 'F') => (x2, y2 + 1),
            (1, 0, 'J') | (-1, 0, 'L') => (x2, y2 - 1),

            _ => return None,
        };
        cur = [(x2, y2), (x2_, y2_)]
    }
    Some(path)
}

fn count_intersections(coord: (i32, i32), path: &[(i32, i32)]) -> i32 {
    let (x, y) = coord;
    path.windows(2)
        .filter(|w| {
            w[0].1 == w[1].1 && w[0].1 < y && w[0].0.min(w[1].0) < x && x <= w[0].0.max(w[1].0)
        })
        .count() as i32
}

fn count_tiles_inside(field: &[Vec<char>], path: &[(i32, i32)]) -> i32 {
    (0..field.len())
        .flat_map(|y| (0..field[0].len()).map(move |x| (x as i32, y as i32)))
        .filter(|&(x, y)| !path.contains(&(x, y)) && count_intersections((x, y), path) % 2 == 1)
        .count() as i32
}

fn find_start(field: &[Vec<char>]) -> Result<(i32, i32)> {
    field
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|c| *c == 'S').map(|x| (x as _, y as _)))
        .ok_or_else(|| anyhow::anyhow!("No S found"))
}

fn get_path(field: &[Vec<char>], (x, y): (i32, i32)) -> Result<Vec<(i32, i32)>> {
    [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)]
        .into_iter()
        .find_map(|adj| follow_the_loop(field, [(x, y), adj]))
        .ok_or_else(|| anyhow::anyhow!("No loop found"))
}

fn main() -> Result<()> {
    let s = fs::read_to_string("input.txt")?;
    let field: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();

    let (x, y) = find_start(&field)?;
    let path = get_path(&field, (x, y))?;

    let answer1 = path.len() / 2;

    println!("Answer to part 1: {}", answer1);
    println!("Answer to part 2: {}", count_tiles_inside(&field, &path));

    Ok(())
}
