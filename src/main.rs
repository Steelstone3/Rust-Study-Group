use std::fmt::{Display, Formatter};

struct Grid {
    cells: Vec<bool>,
    size: usize,
}

impl Grid {
    fn new(size_of_grid: usize, alive_cells: Vec<Coordinate>) -> Grid {
        let alive_cells: Vec<usize> = alive_cells.iter()
            .map(|c| c.1 * size_of_grid + c.0)
            .collect();

        let new_cells: Vec<bool> = (0..size_of_grid * size_of_grid)
            .map(|f| alive_cells.contains(&f))
            .collect();

        Grid {
            cells: new_cells,
            size: size_of_grid,
        }
    }

    fn iterate(&mut self) {}
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for (i, cell) in self.cells.iter().enumerate() {
            if i % self.size == 0 && i > 0 {
                result.push('\n');
            }

            if *cell {
                result.push('*');
            } else {
                result.push(' ');
            }
        }

        write!(f, "{}", result)
    }
}

struct Coordinate(usize, usize);

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_an_empty_grid() {
        let grid = Grid::new(5, vec![]);
        let result = format!("{}", grid);
        let expected = "     \n     \n     \n     \n     ";

        assert_eq!(expected, result);
    }

    #[test]
    fn render_one_cell() {
        let grid = Grid::new(5, vec![Coordinate(0, 0)]);
        let result = format!("{}", grid);
        let expected = "*    \n     \n     \n     \n     ";

        assert_eq!(expected, result);
    }
}
