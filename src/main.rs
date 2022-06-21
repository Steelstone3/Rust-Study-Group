
struct Grid {
    cells: Vec<bool>,
    size: usize
}

impl Grid {
    fn new(size_of_grid: usize) -> Grid {
        Grid {
            cells : vec![],
            size: size_of_grid
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
}
