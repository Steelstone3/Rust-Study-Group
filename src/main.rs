use crate::CellStatus::{ALIVE, DEAD};

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
    cells: Vec<Vec<CellStatus>>,
}

// x = 1
// y = 2
// pos = 7 = y * width + x
// D D D
// D D D
// D A D
impl Grid {
    fn count_neighbours_at(&self, x: u8, y: u8) -> u8 {
        let mut neighbours = 0;

        if self.is_alive_at(x + 1, y) {
            neighbours += 1;
        }
        if self.is_alive_at(x, y + 1) {
            neighbours += 1;
        }
        if self.is_alive_at(x + 1, y + 1) {
            neighbours += 1;
        }

        if x > 0 && self.is_alive_at(x - 1, y + 1) {
            neighbours += 1;
        }
        if y > 0 && self.is_alive_at(x + 1, y - 1) {
            neighbours += 1;
        }

        if x > 0 && self.is_alive_at(x - 1, y) {
            neighbours += 1;
        }
        if y > 0 && self.is_alive_at(x, y - 1) {
            neighbours += 1;
        }
        if x > 0 && y > 0 && self.is_alive_at(x - 1, y - 1) {
            neighbours += 1;
        }

        neighbours
    }

    fn is_alive_at(&self, x: u8, y: u8) -> bool {
        match self.cell_at(x, y) {
            Some(ALIVE) => true,
            _ => false,
        }
    }

    fn cell_at(&self, x: u8, y: u8) -> Option<&CellStatus> {
        let line = self.cells.get(x as usize);
        match line {
            Some(cells) => cells.get(y as usize),
            _ => None
        }
    }
}

pub fn next_generation(previous_generation: Grid) -> Grid {
    let mut next_generation: Vec<Vec<CellStatus>> = Vec::new();

    for x in 0..previous_generation.height {
        let mut line: Vec<CellStatus> = Vec::new();
        for y in 0..previous_generation.width {
            let neighbours = previous_generation.count_neighbours_at(x, y);

            match (neighbours, previous_generation.cell_at(x, y)) {
                (2 | 3, Some(&ALIVE)) => {
                    line.push(ALIVE)
                },
                (3, Some(&DEAD)) => {
                    line.push(ALIVE)
                },
                _ => line.push(DEAD)
            }
        }
        next_generation.push(line);
    }

    Grid {
        cells: next_generation,
        width: previous_generation.width,
        height: previous_generation.height
    }
}

#[cfg(test)]
mod game_of_life_should {
    use super::*;
    use CellStatus::{ALIVE, DEAD};

    #[test]
    fn let_cell_die_because_it_has_no_friends() {
        let previous_generation = Grid {
            cells: vec![
                vec![ALIVE, DEAD, DEAD],
                vec![DEAD, DEAD, DEAD],
                vec![DEAD, DEAD, DEAD],
            ],
            height: 3,
            width: 3,
        };

        assert_eq!(next_generation(previous_generation).cells, vec![
            vec![DEAD, DEAD, DEAD],
            vec![DEAD, DEAD, DEAD],
            vec![DEAD, DEAD, DEAD],
        ]);
    }

    #[test]
    fn let_cell_live_because_the_cell_has_2_friendly_neighbourhood_cell() {
        let previous_generation = Grid {
            cells: vec![
                vec![ALIVE, ALIVE, ALIVE],
                vec![DEAD, DEAD, DEAD],
                vec![DEAD, DEAD, DEAD],
            ],
            height: 3,
            width: 3,
        };

        assert_eq!(
            next_generation(previous_generation).cells,
            vec![
                vec![DEAD, ALIVE, DEAD],
                vec![DEAD, ALIVE, DEAD],
                vec![DEAD, DEAD, DEAD],
            ]
        );
    }

    #[test]
    fn let_cell_die_because_it_has_no_friends_plus_some_alive_cells() {
        let previous_generation = Grid {
            cells: vec![
                vec![ALIVE, DEAD, DEAD],
                vec![DEAD, DEAD, ALIVE],
                vec![DEAD, ALIVE, ALIVE],
            ],
            height: 3,
            width: 3,
        };

        assert_eq!(next_generation(previous_generation).cells, vec![
            vec![DEAD, DEAD, DEAD],
            vec![DEAD, DEAD, ALIVE],
            vec![DEAD, ALIVE, ALIVE],
        ]);
    }

    #[test]
    fn dead_cell_with_three_neighbours_is_alive() {
        let previous_generation = Grid {
            cells: vec![
                vec![DEAD, DEAD, DEAD],
                vec![DEAD, DEAD, ALIVE],
                vec![DEAD, ALIVE, ALIVE],
            ],
            height: 3,
            width: 3,
        };

        assert_eq!(next_generation(previous_generation).cells, vec![
            vec![DEAD, DEAD, DEAD],
            vec![DEAD, ALIVE, ALIVE],
            vec![DEAD, ALIVE, ALIVE],
        ]);
    }

    #[test]
    fn alive_cell_with_three_neighbours_stays_alive() {
        let previous_generation = Grid {
            cells: vec![
                vec![DEAD, DEAD, DEAD],
                vec![DEAD, ALIVE, ALIVE],
                vec![DEAD, ALIVE, ALIVE],
            ],
            height: 3,
            width: 3,
        };

        assert_eq!(next_generation(previous_generation).cells, vec![
            vec![DEAD, DEAD, DEAD],
            vec![DEAD, ALIVE, ALIVE],
            vec![DEAD, ALIVE, ALIVE],
        ]);
    }
}
