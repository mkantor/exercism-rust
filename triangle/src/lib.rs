#[derive(Debug)]
pub enum Error {
    SideTooShort,
    TriangleInequality,
}

pub struct Triangle {
    sides: [usize; 3],
}
impl Triangle {
    pub fn build(sides: [usize; 3]) -> Result<Triangle, Error> {
        if sides.iter().any(|&side| side <= 0) {
            Err(Error::SideTooShort)
        } else if sides[0] + sides[1] < sides[2] || sides[1] + sides[2] < sides[0] ||
                  sides[2] + sides[0] < sides[1] {
            Err(Error::TriangleInequality)
        } else {
            Ok(Triangle { sides: sides })
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1] && self.sides[1] == self.sides[2]
    }

    pub fn is_isosceles(&self) -> bool {
        !self.is_equilateral() && !self.is_scalene()
    }

    pub fn is_scalene(&self) -> bool {
        self.sides[0] != self.sides[1] && self.sides[1] != self.sides[2] &&
        self.sides[2] != self.sides[0]
    }
}
