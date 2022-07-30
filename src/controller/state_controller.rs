pub trait StateController {
    fn start_state() {
        Self::next_state()
    }
    fn next_state();
}