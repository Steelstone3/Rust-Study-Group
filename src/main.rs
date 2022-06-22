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

    fn count_neighbours_at(&self, x: u8, y: u8) -> u8 {
        let mut neighbours = 0;

        if x > 0 && self.is_alive_at(x - 1, y) {
            neighbours += 1;
        }

        if self.is_alive_at(x + 1, y) {
            neighbours += 1;
        }

        neighbours
    }

    fn is_alive_at(&self, x: u8, y: u8) -> bool {
        match self.cell_at(x, y) {
            Some(CellStatus::ALIVE) => true,
            _ => false,
        }
    }

    fn cell_at(&self, x: u8, y: u8) -> Option<&CellStatus> {
        self.cells.get((y * self.width + x) as usize)
    }
}

pub fn next_generation(previous_generation: Grid) -> Grid {
    let mut next_generation: Vec<(u8, u8)> = vec![];

    for y in 0..previous_generation.height {
        for x in 0..previous_generation.width {
            let neighbours = previous_generation.count_neighbours_at(x, y);

            if neighbours == 2 {
                if previous_generation.is_alive_at(x, y) {
                    next_generation.push((x, y))
                }
            }
        }
    }

    Grid::with_alives(next_generation, previous_generation.width, previous_generation.height)
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
