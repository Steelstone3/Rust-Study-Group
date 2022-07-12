use crate::state::mars_rover::RoverState;

mod state;
mod controllers;

fn main() {
    let mut rover = RoverState::create();
    println!("{}", rover.execute("MMLLRR"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use state::mars_rover::RoverState;

    #[rstest]
    #[case("", "0:0:N")]
    #[case("X", "Unknown command/s")]
    #[case("\\", "Unknown command/s")]
    #[case("XPYZ", "Unknown command/s")]
    #[case("^£$%^%", "Unknown command/s")]
    #[case("MMXYPZ", "Unknown command/s")]
    #[case("MLR%G^&LR", "Unknown command/s")]
    #[case("R", "0:0:E")]
    #[case("RR", "0:0:S")]
    #[case("RRR", "0:0:W")]
    #[case("RRRR", "0:0:N")]
    #[case("L", "0:0:W")]
    #[case("LL", "0:0:S")]
    #[case("LLL", "0:0:E")]
    #[case("LLLL", "0:0:N")]
    #[case("M", "0:1:N")]
    #[case("MM", "0:2:N")]
    #[case("MMMMM", "0:5:N")]
    #[case("RM", "1:0:E")]
    #[case("RMMM", "3:0:E")]
    #[case("RRM", "0:-1:S")]
    #[case("RRMM", "0:-2:S")]
    #[case("RRRM", "-1:0:W")]
    #[case("RRRMM", "-2:0:W")]
    #[case("MMRMMLM", "2:3:N")]
    fn execute_rover_commands(#[case] commands: &str, #[case] expected_result: &str) {
        let mut rover = RoverState::create();
        assert_eq!(expected_result, rover.execute(commands));
    }
}
