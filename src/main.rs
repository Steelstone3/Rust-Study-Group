use crate::controller::state_controller::StateController;
use crate::state::normal_state::NormalState;
use crate::state::dowsed_state::DowsedState;
use crate::state::burning_state::BurningState;

mod state;
mod controller;

fn main() {
    BurningState::start_state()
}