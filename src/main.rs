fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Debug)]
pub enum CellStatus {
    ALIVE,
    DEAD,
}

pub struct Cell {
    x: u8,
    y: u8,
    status: CellStatus,
}

pub struct Grid {
    height: u8,
    width: u8,
    cells: Vec<Cell>,
}

pub fn next_generation(previous_generation: Vec<CellStatus>) -> Vec<CellStatus> {
    if previous_generation.len() <= 1 {
        return vec![];
    } else {
        return previous_generation;
    }
}

#[cfg(test)]
mod game_of_life_should {
    use super::*;

    #[test]
    fn let_cell_die_because_it_has_no_friends() {
        let previous_generation = vec![
            CellStatus::ALIVE,
            CellStatus::DEAD,
            CellStatus::DEAD,
            CellStatus::DEAD,
            CellStatus::DEAD,
            CellStatus::DEAD,
            CellStatus::DEAD,
            CellStatus::DEAD,
            CellStatus::DEAD,
        ];

        assert_eq!(next_generation(previous_generation), vec![])
    }

    #[test]
    fn let_cell_live_because_the_cell_has_2_friendly_neighbourhood_cell() {
        let previous_generation = vec![
            CellStatus::ALIVE,
            CellStatus::ALIVE,
            CellStatus::DEAD,
            CellStatus::ALIVE,
            CellStatus::DEAD,
            CellStatus::DEAD,
            CellStatus::DEAD,
            CellStatus::DEAD,
            CellStatus::DEAD,
        ];

        assert_eq!(
            next_generation(previous_generation),
            vec![CellStatus::ALIVE, CellStatus::ALIVE]
        )
    }
}
