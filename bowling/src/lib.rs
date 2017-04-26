// TODO:
//  - Make the order for handling different cases consistent:
//      - Probably always handle error cases first.
//      - Establish a common order for handling strike/spare/open/final frames.
//  - Fix all remaining .unwraps() and unreachable macros. Ideally have explicit handling with
//    error values, or at least provide messages for unreachable.
//      - What's the actual difference between unreachable and panic? If I want to abort on errors
//        which indicate internal inconsistency, which should I use?

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
use frame::Frame::{Strike, Spare, OpenFrame, TenthFrame};
mod frame {
    use throw::Throw;
    use Frame::{Strike, Spare, OpenFrame, TenthFrame};

    #[derive(Debug)]
    pub enum Error {
        MissingThrow,
        ExtraThrow,
        TooManyPins,
    }

    #[derive(Clone, Copy)]
    pub enum Frame {
        Strike(Throw),
        Spare(Throw, Throw),
        OpenFrame(Throw, Throw),
        TenthFrame(Throw, Throw, Option<Throw>),
    }

    impl Frame {
        pub fn new(throw1: Throw, throw2: Option<Throw>) -> Result<Self, Error> {
            match (throw1, throw2) {
                (throw1, None) => {
                    if throw1.pins_down() != 10 {
                        // If the player didn't get a strike there should have been another throw.
                        Err(Error::MissingThrow)
                    } else {
                        Ok(Strike(throw1))
                    }
                }
                (throw1, Some(throw2)) => {
                    if throw1.pins_down() == 10 {
                        // The player got a strike on the first throw, so there shouldn't have
                        // been a second one.
                        Err(Error::ExtraThrow)
                    } else if throw1.pins_down() + throw2.pins_down() > 10 {
                        Err(Error::TooManyPins)
                    } else if throw1.pins_down() + throw2.pins_down() == 10 {
                        Ok(Spare(throw1, throw2))
                    } else {
                        Ok(OpenFrame(throw1, throw2))
                    }
                }
            }
        }

        pub fn new_tenth(throw1: Throw,
                         throw2: Throw,
                         fill_ball: Option<Throw>)
                         -> Result<Self, Error> {
            match (throw1, throw2, fill_ball) {
                (throw1, throw2, None) => {
                    if throw1.pins_down() == 10 || throw1.pins_down() + throw2.pins_down() == 10 {
                        // There should have been a fill ball.
                        Err(Error::MissingThrow)
                    } else if throw1.pins_down() + throw2.pins_down() > 10 {
                        Err(Error::TooManyPins)
                    } else {
                        Ok(TenthFrame(throw1, throw2, None))
                    }
                }
                (throw1, throw2, Some(fill_ball)) => {
                    if throw1.pins_down() < 10 && throw1.pins_down() + throw2.pins_down() < 10 {
                        // There should not have been a fill ball.
                        Err(Error::ExtraThrow)
                    } else if (throw1.pins_down() < 10 && throw2.pins_down() < 10 &&
                               throw1.pins_down() + throw2.pins_down() > 10) ||
                              (throw1.pins_down() == 10 && throw2.pins_down() < 10 &&
                               throw2.pins_down() + fill_ball.pins_down() > 10) {
                        // On the tenth frame, pins are only replaced when there was a strike on
                        // the first or second throw, or a spare on the first two throws.
                        Err(Error::TooManyPins)
                    } else {
                        Ok(TenthFrame(throw1, throw2, Some(fill_ball)))
                    }
                }
            }
        }

        pub fn pins_down(&self) -> u8 {
            match *self {
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


const GAME_LENGTH: usize = 10;

#[derive(Debug)]
pub enum Error {
    TooManyPins,
    ThrowScoreOutOfBounds,
    CannotRollOnCompleteGame,
    CannotScoreIncompleteGame,
}

pub struct SuccessfulRoll;

pub struct BowlingGame {
    frames: [Option<Frame>; GAME_LENGTH],
    completed_frame_count: usize,
    incomplete_frame_throw: Option<Throw>,
    final_frame_extra_throw: Option<Throw>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frames: [None; GAME_LENGTH],
            completed_frame_count: 0,
            incomplete_frame_throw: None,
            final_frame_extra_throw: None,
        }
    }

    fn add_frame(&mut self, frame: Frame) {
        self.frames[self.completed_frame_count] = Some(frame);
        self.completed_frame_count += 1;
        self.incomplete_frame_throw = None;
        self.final_frame_extra_throw = None;
    }

    pub fn roll(&mut self, throw_score: u8) -> Result<SuccessfulRoll, Error> {
        let new_throw = Throw::new(throw_score)
            .map_err(|throw_err| match throw_err {
                         throw::Error::ScoreOutOfBounds => Error::ThrowScoreOutOfBounds,
                     })?;

        let game_is_over = self.completed_frame_count == GAME_LENGTH;
        let game_is_on_tenth_frame = self.completed_frame_count == GAME_LENGTH - 1;

        if game_is_over {
            Err(Error::CannotRollOnCompleteGame)
        } else if game_is_on_tenth_frame {
            // The tenth frame in the game is a special case. If someone throws a strike or a
            // spare then they get a fill ball. Fill balls exist to calculate the total of the
            // 10th frame. Scoring a strike or spare on the fill ball does not give the player
            // more fill balls.
            match (self.incomplete_frame_throw, self.final_frame_extra_throw) {
                (None, None) => {
                    // First throw in final frame.
                    self.incomplete_frame_throw = Some(new_throw);
                    Ok(SuccessfulRoll)
                }
                (Some(first_throw), None) => {
                    // Second throw in final frame.
                    match Frame::new_tenth(first_throw, new_throw, None) {
                        Ok(final_frame) => {
                            self.add_frame(final_frame);
                            Ok(SuccessfulRoll)
                        }
                        Err(frame::Error::MissingThrow) => {
                            self.final_frame_extra_throw = Some(new_throw);
                            Ok(SuccessfulRoll)
                        }
                        Err(frame::Error::TooManyPins) => Err(Error::TooManyPins),
                        Err(frame::Error::ExtraThrow) => unreachable!(),
                    }
                }
                (Some(first_throw), Some(second_throw)) => {
                    // Third throw (fill ball) in final frame.
                    match Frame::new_tenth(first_throw, second_throw, Some(new_throw)) {
                        Ok(final_frame) => {
                            self.add_frame(final_frame);
                            Ok(SuccessfulRoll)
                        }
                        Err(frame::Error::TooManyPins) => Err(Error::TooManyPins),
                        Err(frame::Error::ExtraThrow) => Err(Error::CannotRollOnCompleteGame),
                        Err(frame::Error::MissingThrow) => unreachable!(),
                    }
                }
                (None, Some(_)) => unreachable!(),
            }
        } else {
            // "Normal" (non-tenth) frames.
            match self.incomplete_frame_throw {
                None => {
                    // First throw in a frame.
                    self.incomplete_frame_throw = Some(new_throw);
                    if let Ok(new_frame @ Strike(..)) = Frame::new(new_throw, None) {
                        self.add_frame(new_frame);
                    };
                }
                Some(first_throw) => {
                    // Second throw in a frame.
                    let new_frame = Frame::new(first_throw, Some(new_throw))
                        .map_err(|frame_err| match frame_err {
                                     frame::Error::TooManyPins => Error::TooManyPins,
                                     frame::Error::ExtraThrow | frame::Error::MissingThrow => {
                                         unreachable!("Number of throws in a frame was incorrect")
                                     }
                        })?;
                    self.add_frame(new_frame);
                }
            };

            // Note that there are some early returns above (via `?`) for error conditions.
            Ok(SuccessfulRoll)
        }
    }

    pub fn score(&self) -> Result<u16, Error> {
        let mut score = 0;
        for (i, &frame) in self.frames.iter().enumerate() {
            // These are double-wrapped Options since self.frames is a [Option<Frame>] and get()
            // returns an Option (to handle out of bounds indexes). Flatten them here to keep
            // things simple.
            let next_frame_1 = self.frames.get(i + 1).and_then(|&x| x);
            let next_frame_2 = self.frames.get(i + 2).and_then(|&x| x);

            match frame {
                Some(strike @ Strike(_)) => {
                    // A strike is where all ten pins are knocked down after the first throw. The
                    // total value of a strike is 10 plus the number of pins knocked down in their
                    // next two throws. If a strike is immediately followed by a second strike,
                    // then we can not total the value of first strike until they throw the ball
                    // one more time.
                    let bonus_points = match (next_frame_1, next_frame_2) {
                        (Some(TenthFrame(throw_1, throw_2, _)), _) |
                        (Some(Strike(throw_1)), Some(TenthFrame(throw_2, ..))) |
                        (Some(Strike(throw_1)), Some(Strike(throw_2))) |
                        (Some(Strike(throw_1)), Some(Spare(throw_2, _))) |
                        (Some(Strike(throw_1)), Some(OpenFrame(throw_2, _))) |
                        (Some(Spare(throw_1, throw_2)), Some(_)) |
                        (Some(OpenFrame(throw_1, throw_2)), Some(_)) => {
                            throw_1.pins_down() + throw_2.pins_down()
                        }
                        (None, Some(_)) |
                        (None, None) |
                        (Some(Strike(_)), None) |
                        (Some(Spare(..)), None) |
                        (Some(OpenFrame(..)), None) => {
                            return Err(Error::CannotScoreIncompleteGame);
                        }
                    };
                    score += (strike.pins_down() + bonus_points) as u16;
                }
                Some(spare @ Spare(..)) => {
                    // A spare is where all ten pins are knocked down after the second throw. The
                    // total value of a spare is 10 plus the number of pins knocked down in their
                    // next throw.
                    let bonus_points = match next_frame_1 {
                        Some(TenthFrame(throw, ..)) |
                        Some(Strike(throw)) |
                        Some(Spare(throw, _)) |
                        Some(OpenFrame(throw, _)) => throw.pins_down(),
                        None => {
                            return Err(Error::CannotScoreIncompleteGame);
                        }
                    };
                    score += (spare.pins_down() + bonus_points) as u16;
                }
                Some(open_frame @ OpenFrame(..)) => {
                    // An open frame is where a score of less than 10 is recorded for the frame.
                    // In this case the score for the frame is the number of pins knocked down.
                    score += open_frame.pins_down() as u16;
                }
                Some(tenth_frame @ TenthFrame(..)) => {
                    // The total value of the 10th frame is the total number of pins knocked down.
                    score += tenth_frame.pins_down() as u16;
                }
                None => {
                    return Err(Error::CannotScoreIncompleteGame);
                }
            }
        }
        Ok(score)
    }
}
