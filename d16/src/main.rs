mod direction;
mod tests;
use anyhow::Result;
use std::fs;

use direction::Direction;
use direction::Direction::*;

type Field = Vec<Vec<char>>;
type Memory = Vec<Vec<Vec<direction::Direction>>>;
type Position = (usize, usize);

fn gen_field(s: &str) -> Field {
    s.lines().map(|l| l.chars().collect()).collect()
}

fn process(field: &Field, m: &mut Memory, (x, y): Position, direction: Direction) -> Option<()> {
    if m.get_mut(y)?.get_mut(x)?.contains(&direction) {
        return None;
    }
    m[y][x].push(direction);
    match (direction, field[y][x]) {
        (_, '.') | (Up | Down, '|') | (Left | Right, '-') => {
            process(field, m, direction.step((x, y)), direction)
        }

        (Up, '/') => process(field, m, Right.step((x, y)), Right),
        (Up, '\\') => process(field, m, Left.step((x, y)), Left),
        (Down, '/') => process(field, m, Left.step((x, y)), Left),
        (Down, '\\') => process(field, m, Right.step((x, y)), Right),
        (Left, '/') => process(field, m, Down.step((x, y)), Down),
        (Left, '\\') => process(field, m, Up.step((x, y)), Up),
        (Right, '/') => process(field, m, Up.step((x, y)), Up),
        (Right, '\\') => process(field, m, Down.step((x, y)), Down),

        (Up | Down, '-') => {
            process(field, m, Left.step((x, y)), Left);
            process(field, m, Right.step((x, y)), Right)
        }
        (Left | Right, '|') => {
            process(field, m, Up.step((x, y)), Up);
            process(field, m, Down.step((x, y)), Down)
        }

        _ => unreachable!(),
    };
    None
}

fn task_1(field: &Field, start: Position, direction: Direction) -> usize {
    let (n, m) = (field.len(), field[0].len());
    let mut memory: Memory = vec![vec![Vec::new(); n]; m];
    process(field, &mut memory, start, direction);
    memory.iter().flatten().filter(|v| !v.is_empty()).count()
}

fn task_2(field: &Field) -> usize {
    let (n, m) = (field.len(), field[0].len());
    let right = (0..n).map(|i| task_1(field, (0, i), Right));
    let left = (0..n).map(|i| task_1(field, (0, i), Left));
    let up = (0..m).map(|i| task_1(field, (i, 0), Up));
    let down = (0..m).map(|i| task_1(field, (i, 0), Down));

    right
        .chain(left)
        .chain(up)
        .chain(down)
        .inspect(|v| println!("{}", v))
        .max()
        .unwrap()
}

fn main() -> Result<()> {
    let s = fs::read_to_string("input.txt")?;
    let field = gen_field(&s);
    println!("Answer to task 1: {}", task_1(&field, (0, 0), Right));

    println!("Answer to task 2: {}", task_2(&field));

    Ok(())
}
