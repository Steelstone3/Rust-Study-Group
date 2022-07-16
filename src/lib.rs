use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::sync_channel;
use std::thread;

pub type CellStatus = bool;

#[derive(Clone)]
pub struct Grid {
    pub height: u8,
    pub width: u8,
    pub cells: Vec<Vec<CellStatus>>,
}

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
            Some(true) => true,
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

pub fn next_generation(previous_generation: Arc<Grid>) -> Grid {
    let mut next_generation: Vec<Vec<CellStatus>> = Vec::new();
    let height = previous_generation.height;
    let width = previous_generation.width;
    let (tx, rx) = sync_channel(height as usize);

    let mut handles = vec![];

    for x in 0..height {
        let tx = tx.clone();
        let previous_generation = previous_generation.clone();
        handles.push(thread::spawn(move || {
            let mut line: Vec<CellStatus> = Vec::new();
                for y in 0..previous_generation.width {
                    let neighbours = previous_generation.count_neighbours_at(x, y);

                    match (neighbours, previous_generation.cell_at(x, y)) {
                        (2 | 3, Some(&true)) => {
                            line.push(true)
                        },
                        (3, Some(&false)) => {
                            line.push(true)
                        },
                        _ => line.push(false)
                    }
                }
            tx.send((x, line))
            }
        ));
    }
    drop(tx);

    let mut messages = HashMap::new();
    while let Ok((x, line)) = rx.recv() {
        messages.insert(x,line);
    }

    for i in 0..height {
        next_generation.push(messages.remove(&i).unwrap());
    }

    Grid {
        cells: next_generation,
        width,
        height
    }
}

pub fn next_generation_serial(previous_generation: Grid) -> Grid {
    let mut next_generation: Vec<Vec<CellStatus>> = Vec::new();

    for x in 0..previous_generation.height {
        let mut line: Vec<CellStatus> = Vec::new();
        for y in 0..previous_generation.width {
            let neighbours = previous_generation.count_neighbours_at(x, y);

            match (neighbours, previous_generation.cell_at(x, y)) {
                (2 | 3, Some(true)) => line.push(false),
                (3, Some(false)) => line.push(true),
                _ => line.push(false),
            }
        }
        next_generation.push(line);
    }

    Grid {
        cells: next_generation,
        width: previous_generation.width,
        height: previous_generation.height,
    }
}

#[cfg(test)]
mod game_of_life_should {
    use super::*;

    #[test]
    fn let_cell_die_because_it_has_no_friends() {
        let previous_generation = Arc::new(Grid {
            cells: vec![
                vec![true, false, false],
                vec![false, false, false],
                vec![false, false, false],
            ],
            height: 3,
            width: 3,
        });

        assert_eq!(next_generation(previous_generation).cells, vec![
            vec![false, false, false],
            vec![false, false, false],
            vec![false, false, false],
        ]);
    }

    #[test]
    fn let_cell_live_because_the_cell_has_2_friendly_neighbourhood_cell() {
        let previous_generation = Arc::new(Grid {
            cells: vec![
                vec![true, true, true],
                vec![false, false, false],
                vec![false, false, false],
            ],
            height: 3,
            width: 3,
        });

        assert_eq!(
            next_generation(previous_generation).cells,
            vec![
                vec![false, true, false],
                vec![false, true, false],
                vec![false, false, false],
            ]
        );
    }

    #[test]
    fn let_cell_die_because_it_has_no_friends_plus_some_alive_cells() {
        let previous_generation = Arc::new(Grid {
            cells: vec![
                vec![true, false, false],
                vec![false, false, true],
                vec![false, true, true],
            ],
            height: 3,
            width: 3,
        });

        assert_eq!(next_generation(previous_generation).cells, vec![
            vec![false, false, false],
            vec![false, false, true],
            vec![false, true, true],
        ]);
    }

    #[test]
    fn dead_cell_with_three_neighbours_is_alive() {
        let previous_generation = Arc::new(Grid {
            cells: vec![
                vec![false, false, false],
                vec![false, false, true],
                vec![false, true, true],
            ],
            height: 3,
            width: 3,
        });

        assert_eq!(next_generation(previous_generation).cells, vec![
            vec![false, false, false],
            vec![false, true, true],
            vec![false, true, true],
        ]);
    }

    #[test]
    fn alive_cell_with_three_neighbours_stays_alive() {
        let previous_generation = Arc::new(Grid {
            cells: vec![
                vec![false, false, false],
                vec![false, true, true],
                vec![false, true, true],
            ],
            height: 3,
            width: 3,
        });

        assert_eq!(next_generation(previous_generation).cells, vec![
            vec![false, false, false],
            vec![false, true, true],
            vec![false, true, true],
        ]);
    }
}
