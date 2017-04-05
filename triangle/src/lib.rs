use std::ops::Add;

#[derive(Debug)]
pub enum Error {
    SideTooShort,
    TriangleInequality,
}

pub struct Triangle<T> {
    sides: [T; 3],
}

// I wanted to try satisfying the optional tests without using any crates or creating separate
// impls for each numeric type, so this is a bit wonky.
//
// Assumptions that are made:
//  - Any type T that implements some necessary traits can be sides of a Triangle, even when T is
//    not something that would traditionally be considered "numeric". This means you can do funky
//    things like create Triangles whose sides are std::time::Duration (time triangles!).
//  - The Default value for type T is equivalent to zero.
impl<T: Copy + Default + PartialOrd + Add<Output = T>> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Result<Triangle<T>, Error> {
        let zero: T = Default::default();
        if sides.contains(&zero) {
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
