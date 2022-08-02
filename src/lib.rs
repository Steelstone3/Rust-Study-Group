#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: u16,
    score: u16,
    max_rolls: u16,
    active_frame: Option<Frame>,
}

pub struct Frame {
    roll_one_pins: u16,
    roll_two_pins: Option<u16>,
}

impl Frame {
    pub fn is_spare(&self) -> bool {
        if let Some(roll_two_pins) = self.roll_two_pins {
            self.roll_one_pins + roll_two_pins == 10
        } else {
            false
        }
    }

    fn is_second_roll_in_frame(&self) -> bool {
        self.roll_two_pins.is_none()
    }
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            rolls: 0,
            max_rolls: 20,
            score: 0,
            active_frame: None,
        }
    }

    pub fn reset_active_frame(&mut self, pins: u16) {
        self.active_frame = Some(Frame {
            roll_one_pins: pins,
            roll_two_pins: None,
        });
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.is_game_complete() {
            return Err(Error::GameComplete);
        }

        self.rolls += 1;
        self.score += pins;

        if let Some(ref mut frame) = self.active_frame {
            if frame.is_spare() {
                if !(self.is_game_complete()) {
                    self.score += pins;
                }
                self.reset_active_frame(pins);
            } else if frame.is_second_roll_in_frame() {
                frame.roll_two_pins = Some(pins);
                if self.is_spare_last_frame() {
                    self.max_rolls += 1;
                }
            } else {
                self.reset_active_frame(pins);
            }
        } else {
            self.reset_active_frame(pins);
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !(self.is_game_complete()) {
            return None;
        }

        Some(self.score)
    }

    fn is_spare_last_frame(&self) -> bool {
        self.active_frame.as_ref().unwrap().is_spare() && self.rolls == self.max_rolls
    }

    fn is_game_complete(&self) -> bool {
        self.rolls == self.max_rolls
    }
}
