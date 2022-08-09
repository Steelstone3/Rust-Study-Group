#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: u16,
    score: u16,
    max_rolls: u16,
    active_frame: Frame,
    last_frame_was_spare: bool
}

pub struct Frame {
    roll_one_pins: Option<u16>,
    roll_two_pins: Option<u16>,
}

impl Frame {
    pub fn is_spare(&self) -> bool {
        if let Some(roll_two_pins) = self.roll_two_pins {
            self.roll_one_pins.unwrap() + roll_two_pins == 10
        } else {
            false
        }
    }

    fn is_first_roll_in_frame(&self) -> bool {
        self.roll_one_pins.is_none()
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
            active_frame: Frame {
                roll_one_pins: None,
                roll_two_pins: None,
            },
            last_frame_was_spare: false
        }
    }

    pub fn reset_active_frame(&mut self) {
        self.active_frame = Frame {
            roll_one_pins: None,
            roll_two_pins: None,
        };
    }

    pub fn reset_active_frame_with_first_roll(&mut self, pins: u16) {
        self.active_frame = Frame {
            roll_one_pins: Some(pins),
            roll_two_pins: None,
        };
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

        if self.active_frame.is_first_roll_in_frame() {
            self.calculate_bonus_score(pins);
            self.record_first_roll(pins);
        }
        else if self.active_frame.is_second_roll_in_frame() {
            self.record_second_roll(pins);
            self.check_for_spares();
            self.reset_active_frame()
        }

        Ok(())
    }

    fn record_second_roll(&mut self, pins: u16) {
        self.active_frame.roll_two_pins = Some(pins);
    }

    fn record_first_roll(&mut self, pins: u16) {
        self.active_frame.roll_one_pins = Some(pins);
        if pins == 10 {
            self.rolls += 1;
        }
        self.last_frame_was_spare = false;
    }

    fn calculate_bonus_score(&mut self, pins: u16) {
        if self.last_frame_was_spare {
            if !(self.is_game_complete()) {
                self.score += pins;
            }
        }
    }

    fn check_for_spares(&mut self) {
        if self.active_frame.is_spare() {
            self.last_frame_was_spare = true;
            if self.is_last_frame() {
                self.max_rolls += 1;
            }
        }
    }

    pub fn score(&self) -> Option<u16> {
        if !(self.is_game_complete()) {
            return None;
        }

        Some(self.score)
    }

    fn is_last_frame(&self) -> bool {
        self.rolls == self.max_rolls
    }

    fn is_game_complete(&self) -> bool {
        self.rolls == self.max_rolls
    }
}
