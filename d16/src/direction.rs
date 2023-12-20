use super::Position;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub(crate) fn step(&self, p: Position) -> Position {
        match self {
            Direction::Up => (p.0, p.1.wrapping_sub(1)),
            Direction::Down => (p.0, p.1 + 1),
            Direction::Left => (p.0.wrapping_sub(1), p.1),
            Direction::Right => (p.0 + 1, p.1),
        }
    }
}

