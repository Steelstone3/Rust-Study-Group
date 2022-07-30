use crate::controller::state_controller::StateController;

pub struct NormalState {}

impl StateController for NormalState {
    fn start_state() {
        println!("I am not burning");
        Self::next_state();
    }

    fn next_state() {
        println!("Fin!")
    }
}

#[cfg(test)]
mod normal_state_should {
    use super::*;

    #[test]
    fn start_the_normal_state() {}
}