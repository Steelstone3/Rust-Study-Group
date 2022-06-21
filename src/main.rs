fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Debug, Clone)]
pub enum CellStatus {
    ALIVE,
    DEAD,
}

pub struct Grid {
    height: u8,
    width: u8,
    cells: Vec<CellStatus>,
}

pub struct Position {
    x: u8,
    y: u8
}

// x = 1
// y = 2
// pos = 7 = y * width + x
// D D D
// D D D
// D A D
impl Grid {
    fn with_alives(alive_cell_positions: Vec<(u8, u8)>, height: u8, width: u8) -> Self {
        let mut cells = vec![CellStatus::DEAD; (height * width) as usize];

        for (x, y) in alive_cell_positions {
            cells[(y * width + x) as usize] = CellStatus::ALIVE
        }

        Grid {
            height,
            width,
            cells
        }
    }
}

pub fn next_generation(previous_generation: Grid) -> Grid {
    for y in 0..previous_generation.height {
        for x in 0..previous_generation.width {
            // count neighbours
                // look top left | top | top right
                // look     left |     |     right
                // look bot left | bot | bot right
        }
    }
    Grid::with_alives(vec![], 3, 3)
}

#[cfg(test)]
mod game_of_life_should {
    use super::*;
    use CellStatus::{ALIVE,DEAD};

    #[test]
    fn let_cell_die_because_it_has_no_friends() {
        let previous_generation = Grid::with_alives(vec![(0, 0)], 3, 3);

        assert_eq!(next_generation(previous_generation).cells, vec![
            DEAD, DEAD, DEAD,
            DEAD, DEAD, DEAD,
            DEAD, DEAD, DEAD
        ]);
    }

    #[test]
    fn let_cell_live_because_the_cell_has_2_friendly_neighbourhood_cell() {
        let previous_generation = Grid::with_alives(vec![(0, 0), (1, 0), (2, 0)], 3, 3);

        assert_eq!(
            next_generation(previous_generation).cells,
            vec![
                DEAD, ALIVE, DEAD,
                DEAD, DEAD,  DEAD,
                DEAD, DEAD,  DEAD
            ]
        );
    }
}
