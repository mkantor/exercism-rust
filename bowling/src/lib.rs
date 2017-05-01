use throw::Throw;
mod throw {
    #[derive(Debug)]
    pub enum Error {
        ScoreOutOfBounds,
    }

    #[derive(Clone, Copy)]
    pub struct Throw {
        pins_down: u8,
    }

    impl Throw {
        pub fn new(pins_down: u8) -> Result<Throw, Error> {
            match pins_down {
                0...10 => Ok(Throw { pins_down: pins_down }),
                _ => Err(Error::ScoreOutOfBounds),
            }
        }

        pub fn pins_down(&self) -> u8 {
            self.pins_down
        }
    }
}


use frame::Frame;
use frame::Frame::{IncompleteFrame, Strike, Spare, OpenFrame, TenthFrame};
mod frame {
    use throw::Throw;
    use self::Frame::{IncompleteFrame, Strike, Spare, OpenFrame, TenthFrame};
    use self::IncompleteFrameThrows::{ZeroThrows, OneThrow, TwoThrows};

    #[derive(Debug)]
    pub enum Error {
        ExtraThrow,
        TooManyPins,
    }

    // Rust wishlist item: anonymous enum/variant syntax so I could do this:
    // enum Frame {
    //     IncompleteFrame(enum { (), (Throw), (Throw, Throw) }),
    //     Strike(Throw),
    //     Spare(Throw, Throw),
    //     OpenFrame(Throw, Throw),
    //     TenthFrame(enum { (Throw, Throw), (Throw, Throw, Throw) }),
    // }

    #[derive(Clone, Copy)]
    pub enum Frame {
        IncompleteFrame(IncompleteFrameThrows),
        Strike(Throw),
        Spare(Throw, Throw),
        OpenFrame(Throw, Throw),
        TenthFrame(Throw, Throw, Option<Throw>),
    }

    #[derive(Clone, Copy)]
    pub enum IncompleteFrameThrows {
        ZeroThrows,
        OneThrow(Throw),
        TwoThrows(Throw, Throw),
    }

    impl Default for Frame {
        fn default() -> Self {
            IncompleteFrame(ZeroThrows)
        }
    }

    impl Frame {
        pub fn throw(&self, new_throw: Throw) -> Result<Self, Error> {
            match *self {
                IncompleteFrame(ZeroThrows) => {
                    if new_throw.pins_down() != 10 {
                        Ok(IncompleteFrame(OneThrow(new_throw)))
                    } else {
                        Ok(Strike(new_throw))
                    }
                }
                IncompleteFrame(OneThrow(first_throw)) => {
                    if first_throw.pins_down() + new_throw.pins_down() > 10 {
                        Err(Error::TooManyPins)
                    } else if first_throw.pins_down() + new_throw.pins_down() == 10 {
                        Ok(Spare(first_throw, new_throw))
                    } else {
                        Ok(OpenFrame(first_throw, new_throw))
                    }
                }
                IncompleteFrame(TwoThrows(..)) |
                Strike(_) |
                Spare(..) |
                OpenFrame(..) |
                TenthFrame(..) => Err(Error::ExtraThrow),
            }
        }

        pub fn tenth_frame_throw(&self, new_throw: Throw) -> Result<Self, Error> {
            // The tenth frame in the game is a special case. If someone throws a strike or a
            // spare then they get a fill ball. Fill balls exist to calculate the total of the
            // 10th frame. Scoring a strike or spare on the fill ball does not give the player
            // more fill balls.
            match *self {
                IncompleteFrame(ZeroThrows) => Ok(IncompleteFrame(OneThrow(new_throw))),
                IncompleteFrame(OneThrow(first_throw)) => {
                    if first_throw.pins_down() != 10 &&
                       first_throw.pins_down() + new_throw.pins_down() > 10 {
                        Err(Error::TooManyPins)
                    } else if first_throw.pins_down() == 10 ||
                              first_throw.pins_down() + new_throw.pins_down() == 10 {
                        // The player will need a fill ball, so the tenth frame isn't over yet.
                        Ok(IncompleteFrame(TwoThrows(first_throw, new_throw)))
                    } else {
                        Ok(TenthFrame(first_throw, new_throw, None))
                    }
                }
                IncompleteFrame(TwoThrows(first_throw, second_throw)) => {
                    if first_throw.pins_down() < 10 &&
                       first_throw.pins_down() + second_throw.pins_down() < 10 {
                        // There should not have been a fill ball.
                        Err(Error::ExtraThrow)
                    } else if (first_throw.pins_down() < 10 && second_throw.pins_down() < 10 &&
                               first_throw.pins_down() + second_throw.pins_down() > 10) ||
                              (first_throw.pins_down() == 10 && second_throw.pins_down() < 10 &&
                               second_throw.pins_down() + new_throw.pins_down() > 10) {
                        // On the tenth frame, pins are only replaced when there was a strike on
                        // the first or second throw, or a spare on the first two throws.
                        Err(Error::TooManyPins)
                    } else {
                        Ok(TenthFrame(first_throw, second_throw, Some(new_throw)))
                    }
                }
                Strike(_) | Spare(..) | OpenFrame(..) | TenthFrame(..) => Err(Error::ExtraThrow),
            }
        }

        pub fn is_complete(&self) -> bool {
            match *self {
                IncompleteFrame(..) => false,
                _ => true,
            }
        }

        pub fn pins_down(&self) -> u8 {
            match *self {
                IncompleteFrame(ZeroThrows) => 0,
                IncompleteFrame(OneThrow(throw1)) => throw1.pins_down(),
                IncompleteFrame(TwoThrows(throw1, throw2)) => {
                    throw1.pins_down() + throw2.pins_down()
                }
                Strike(_) | Spare(..) => 10,
                OpenFrame(throw1, throw2) |
                TenthFrame(throw1, throw2, None) => throw1.pins_down() + throw2.pins_down(),
                TenthFrame(throw1, throw2, Some(throw3)) => {
                    throw1.pins_down() + throw2.pins_down() + throw3.pins_down()
                }
            }
        }
    }
}


#[derive(Debug)]
pub enum Error {
    TooManyPins,
    ThrowScoreOutOfBounds,
    CannotRollOnCompleteGame,
    CannotScoreIncompleteGame,
}

pub struct SuccessfulRoll;

pub struct BowlingGame {
    frames: [Frame; 10],
    completed_frame_count: usize,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frames: [Frame::default(); 10],
            completed_frame_count: 0,
        }
    }

    fn frame(&mut self, frame: Frame) {
        self.frames[self.completed_frame_count] = frame;
        if frame.is_complete() {
            self.completed_frame_count += 1;
        }
    }

    fn current_frame(&self) -> Frame {
        self.frames[self.completed_frame_count]
    }

    pub fn roll(&mut self, throw_score: u8) -> Result<SuccessfulRoll, Error> {
        if self.completed_frame_count == 10 {
            Err(Error::CannotRollOnCompleteGame)
        } else {
            let new_throw = Throw::new(throw_score)
                .map_err(|error| match error {
                             throw::Error::ScoreOutOfBounds => Error::ThrowScoreOutOfBounds,
                         })?;

            let new_frame = (if self.completed_frame_count == 9 {
                self.current_frame().tenth_frame_throw(new_throw)
            } else {
                self.current_frame().throw(new_throw)
            }).map_err(|error| match error {
                frame::Error::TooManyPins => Error::TooManyPins,
                frame::Error::ExtraThrow => {
                    unreachable!("Number of throws in frame was incorrect")
                }
            })?;

            self.frame(new_frame);
            Ok(SuccessfulRoll)
        }
    }

    pub fn score(&self) -> Result<u16, Error> {
        let mut score = 0;
        for (i, &frame) in self.frames.iter().enumerate() {
            score += match frame {
                IncompleteFrame(..) => {
                    return Err(Error::CannotScoreIncompleteGame);
                }
                strike @ Strike(_) => {
                    // A strike is where all ten pins are knocked down after the first throw. The
                    // total value of a strike is 10 plus the number of pins knocked down in their
                    // next two throws. If a strike is immediately followed by a second strike,
                    // then we can not total the value of first strike until they throw the ball
                    // one more time.
                    let bonus_points = match (self.frames.get(i + 1), self.frames.get(i + 2)) {
                        (Some(&IncompleteFrame(..)), _) |
                        (Some(_), Some(&IncompleteFrame(..))) => {
                            return Err(Error::CannotScoreIncompleteGame);
                        }
                        (Some(&Strike(next_throw)), Some(&Strike(next_next_throw))) |
                        (Some(&Strike(next_throw)), Some(&Spare(next_next_throw, _))) |
                        (Some(&Strike(next_throw)), Some(&OpenFrame(next_next_throw, _))) |
                        (Some(&Strike(next_throw)), Some(&TenthFrame(next_next_throw, ..))) |
                        (Some(&Spare(next_throw, next_next_throw)), Some(_)) |
                        (Some(&OpenFrame(next_throw, next_next_throw)), Some(_)) |
                        (Some(&TenthFrame(next_throw, next_next_throw, _)), None) => {
                            next_throw.pins_down() + next_next_throw.pins_down()
                        }
                        (None, _) |
                        (Some(&Strike(_)), None) |
                        (Some(&Spare(..)), None) |
                        (Some(&OpenFrame(..)), None) => unreachable!("Final frame was not a TenthFrame"),
                        (Some(&TenthFrame(..)), Some(_)) => unreachable!("TenthFrame was not final frame"),
                    };
                    (strike.pins_down() + bonus_points) as u16
                }
                spare @ Spare(..) => {
                    // A spare is where all ten pins are knocked down after the second throw. The
                    // total value of a spare is 10 plus the number of pins knocked down in their
                    // next throw.
                    let bonus_points = match self.frames.get(i + 1) {
                        Some(&IncompleteFrame(..)) => {
                            return Err(Error::CannotScoreIncompleteGame);
                        }
                        Some(&Strike(next_throw)) |
                        Some(&Spare(next_throw, _)) |
                        Some(&OpenFrame(next_throw, _)) |
                        Some(&TenthFrame(next_throw, ..)) => next_throw.pins_down(),
                        None => unreachable!("Final frame was not a TenthFrame"),
                    };
                    (spare.pins_down() + bonus_points) as u16
                }
                open_frame @ OpenFrame(..) => {
                    // An open frame is where a score of less than 10 is recorded for the frame.
                    // In this case the score for the frame is the number of pins knocked down.
                    open_frame.pins_down() as u16
                }
                tenth_frame @ TenthFrame(..) => {
                    // The total value of the 10th frame is the total number of pins knocked down.
                    tenth_frame.pins_down() as u16
                }
            }
        }
        Ok(score)
    }
}
