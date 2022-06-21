use std::fmt::{Display, Formatter, Write};

struct Grid {
    cells: Vec<bool>,
    size: usize,
}

impl Grid {
    fn new(size_of_grid: usize) -> Grid {
        Grid {
            cells: vec![],
            size: size_of_grid,
        }
    }

    fn iterate(&mut self) {}
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "     \n     \n     \n     \n     ")
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_an_empty_grid() {
        let grid = Grid::new(5);
        let result = format!("{}", grid);
        let expected = "     \n     \n     \n     \n     ";

        assert_eq!(expected, result);
    }
}
