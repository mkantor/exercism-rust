use Chirality::*;
use Shape::*;

#[derive(Clone, Copy, PartialEq)]
enum Chirality {
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq)]
enum Shape {
    Round,
    Square,
    Curly,
}

struct Bracket(Chirality, Shape);
impl Bracket {
    fn new(character: char) -> Option<Self> {
        match character {
            '(' => Some(Bracket(Left, Round)),
            '[' => Some(Bracket(Left, Square)),
            '{' => Some(Bracket(Left, Curly)),
            ')' => Some(Bracket(Right, Round)),
            ']' => Some(Bracket(Right, Square)),
            '}' => Some(Bracket(Right, Curly)),
            _ => None,
        }
    }
}

pub struct Brackets(Vec<Bracket>);
impl Brackets {
    pub fn are_balanced(&self) -> bool {
        let mut bracket_scope = vec![];
        for bracket in &self.0 {
            match *bracket {
                Bracket(Left, shape) => bracket_scope.push(shape),
                Bracket(Right, shape) => {
                    if bracket_scope.pop() != Some(shape) {
                        // This right bracket didn't match the previous left bracket.
                        return false;
                    }
                }
            }
        }

        // If there are leftovers on the stack then there were unclosed brackets.
        bracket_scope.is_empty()
    }
}
impl<'a> From<&'a str> for Brackets {
    fn from(string: &'a str) -> Self {
        Brackets(string.chars().filter_map(Bracket::new).collect())
    }
}
