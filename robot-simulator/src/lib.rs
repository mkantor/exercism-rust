use Direction::*;

#[derive(Debug)]
enum Error {
    UnknownInstruction,
}

#[derive(Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    fn right(self) -> Self {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    fn left(self) -> Self {
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Robot {
    location: Point,
    orientation: Direction,
}
impl Robot {
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot {
            location: Point { x: x, y: y },
            orientation: d,
        }
    }

    pub fn turn_right(self) -> Self {
        Robot {
            orientation: self.orientation.right(),
            ..self
        }
    }

    pub fn turn_left(self) -> Self {
        Robot {
            orientation: self.orientation.left(),
            ..self
        }
    }

    pub fn advance(self) -> Self {
        Robot {
            location: Point {
                x: match self.orientation {
                    East => self.location.x + 1,
                    West => self.location.x - 1,
                    _ => self.location.x,
                },
                y: match self.orientation {
                    North => self.location.y + 1,
                    South => self.location.y - 1,
                    _ => self.location.y,
                },
            },
            ..self
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        // The tests don't expect graceful error handling, so ignore bad inputs.
        instructions
            .chars()
            .fold(self,
                  |robot, instruction| robot.instruction(instruction).unwrap_or(robot))
    }

    fn instruction(self, instruction: char) -> Result<Self, Error> {
        match instruction {
            'A' => Ok(self.advance()),
            'L' => Ok(self.turn_left()),
            'R' => Ok(self.turn_right()),
            _ => Err(Error::UnknownInstruction),
        }
    }

    pub fn position(&self) -> (isize, isize) {
        (self.location.x, self.location.y)
    }

    // FIXME? Why do the tests expect this to return a reference?
    pub fn direction(&self) -> &Direction {
        &(self.orientation)
    }
}
