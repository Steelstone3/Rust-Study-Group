use std::fmt::{Display, Formatter};
use crate::state::direction::Cardinal;

impl Cardinal {
    pub fn rotate_right(&self) -> Cardinal {
        match self {
            Cardinal::North => Cardinal::East,
            Cardinal::East => Cardinal::South,
            Cardinal::South => Cardinal::West,
            Cardinal::West => Cardinal::North,
        }
    }

    pub fn rotate_left(&self) -> Cardinal {
        match self {
            Cardinal::North => Cardinal::West,
            Cardinal::East => Cardinal::North,
            Cardinal::South => Cardinal::East,
            Cardinal::West => Cardinal::South,
        }
    }
}

impl Display for Cardinal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cardinal::North => {
                write!(f, "N")
            }
            Cardinal::East => {
                write!(f, "E")
            }
            Cardinal::South => {
                write!(f, "S")
            }
            Cardinal::West => {
                write!(f, "W")
            }
        }
    }
}
