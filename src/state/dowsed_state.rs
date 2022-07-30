use crate::NormalState;
use crate::controller::state_controller::StateController;

pub struct DowsedState {}

impl StateController for DowsedState {
    fn start_state() {
        println!("I am being put out");
        Self::next_state();
    }

    fn next_state() {
        NormalState::start_state();
    }
}

#[cfg(test)]
mod dowsed_state_should {
    use super::*;

    #[test]
    fn start_the_dowsed_state() {}
}