use num::{one, Integer};
use std::ops;
use Direction::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position<T: Integer> {
    pub x: T,
    pub y: T,
}

impl<T: Integer> Position<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Integer + Copy> ops::Add<Direction> for Position<T> {
    type Output = Self;

    fn add(mut self, rhs: Direction) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T: Integer + Copy> ops::AddAssign<Direction> for Position<T> {
    fn add_assign(&mut self, rhs: Direction) {
        match rhs {
            Up => self.y = self.y - one(),
            Down => self.y = self.y + one(),
            Left => self.x = self.x - one(),
            Right => self.x = self.x + one(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
