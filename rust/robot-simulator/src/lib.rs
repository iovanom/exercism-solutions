// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
use Direction::*;

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    position: (i32, i32),
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {
            position: (x, y),
            direction: d,
        }
    }

    pub fn turn_right(mut self) -> Self {
        self.direction = match self.direction {
            North => East,
            East => South,
            South => West,
            West => North,
        };
        self
    }

    pub fn turn_left(mut self) -> Self {
        self.direction = match self.direction {
            North => West,
            West => South,
            South => East,
            East => North,
        };
        self
    }

    pub fn advance(mut self) -> Self {
        match self.direction {
            North => self.position.1 += 1,
            East => self.position.0 += 1,
            South => self.position.1 -= 1,
            West => self.position.0 -= 1,
        }
        self
    }

    pub fn instructions(mut self, instructions: &str) -> Self {
        for instruction in instructions.chars() {
            self = match instruction {
                'L' => self.turn_left(),
                'R' => self.turn_right(),
                'A' => self.advance(),
                _ => self,
            }
        }
        self
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
