use crate::state::direction::Cardinal;
use crate::state::mars_rover::RoverState;

impl RoverState {
    pub fn create() -> RoverState {
        RoverState {
            x: 0,
            y: 0,
            direction: Cardinal::North,
        }
    }

    pub fn execute(&mut self, commands: &str) -> String {
        for command in commands.chars() {
            match command {
                'M' => self.move_rover(),
                'L' => self.direction = self.direction.rotate_left(),
                'R' => self.direction = self.direction.rotate_right(),
                _ => return String::from("Unknown command/s"),
            }
        }
        return format!("{}:{}:{}", self.x, self.y, self.direction);
    }

    pub fn move_rover(&mut self) {
        match self.direction {
                Cardinal::North => self.y += 1,
                Cardinal::East => self.x += 1,
                Cardinal::South => self.y -= 1,
                Cardinal::West => self.x -= 1,
        }
    }
}
