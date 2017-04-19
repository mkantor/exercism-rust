use Chirality::*;
use Shape::*;

#[derive(PartialEq)]
enum Chirality {
    Left,
    Right,
}

#[derive(PartialEq)]
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

pub struct Brackets<'a> {
    // TODO? I could just store a Vec<Bracket> here (and make the From<&str> impl walk through the
    // chars). That would keep this from being tied to the lifetime of the input &str (at the cost
    // of extra allocations during construction).
    //
    // In reality I'd base this decision on usage: Are Brackets instances long-lived? Do callers
    // need to mutate the input &str? Are the input &strs typically large or small?
    //
    // Since I don't have anything to base this on here, go with whatever is more idiomatic.
    string: &'a str,
}
impl<'a> Brackets<'a> {
    pub fn are_balanced(&self) -> bool {
        let mut bracket_scope = vec![];
        for character in self.string.chars() {
            if let Some(bracket) = Bracket::new(character) {
                match bracket {
                    Bracket(Left, shape) => bracket_scope.push(shape),
                    Bracket(Right, shape) => {
                        if bracket_scope.pop() != Some(shape) {
                            // This right bracket didn't match the previous left bracket.
                            return false;
                        }
                    }
                }
            }
        }

        // If there are leftovers on the stack then there were unclosed brackets.
        bracket_scope.len() == 0
    }
}
impl<'a> From<&'a str> for Brackets<'a> {
    fn from(string: &'a str) -> Self {
        Brackets { string: string }
    }
}
