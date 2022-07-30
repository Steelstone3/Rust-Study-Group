use crate::controller::state_controller::StateController;
use crate::DowsedState;

pub struct BurningState {}

impl StateController for BurningState {
    fn start_state() {
        println!("I am burning");
        Self::next_state();
    }

    fn next_state() {
        DowsedState::start_state();
    }
}

#[cfg(test)]
mod burning_state_should {
    use super::*;

    #[test]
    fn start_the_burning_state() {}
}