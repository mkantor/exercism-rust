#[derive(Debug)]
pub enum Error {
    OutOfBounds,
}

#[derive(Clone, Copy)]
pub struct ChessPosition {
    x: i8,
    y: i8,
}

impl ChessPosition {
    pub fn new(x: i8, y: i8) -> Result<Self, Error> {
        match (x, y) {
            (0...7, 0...7) => Ok(Self { x: x, y: y }),
            _ => Err(Error::OutOfBounds),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Queen {
    position: ChessPosition,
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position: position }
    }

    pub fn can_attack(&self, other_queen: &Self) -> bool {
        let p1 = self.position;
        let p2 = other_queen.position;
        let same_rank_or_file = p1.x == p2.x || p1.y == p2.y;
        let same_diagonal = (p1.x - p2.x).abs() == (p1.y - p2.y).abs();

        same_rank_or_file || same_diagonal
    }
}
