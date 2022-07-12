use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
use conways_game_of_life::{CellStatus, Grid, next_generation};
use conways_game_of_life::CellStatus::{ALIVE, DEAD};

pub fn criterion_benchmark(c: &mut Criterion) {
    let grid = Grid {
        cells: generate_test_data(1000),
        height: 3,
        width: 3,
    };

    c.bench_function("next generation", |b| b.iter(|| next_generation(grid.clone())));
}

fn generate_test_data(size: usize) -> Vec<Vec<CellStatus>> {
    let mut result = vec![];
    let mut rng = rand::thread_rng();

    for i in 0..=size {
        let mut inner = vec![];
        for i in 0..=size {
            let rand_value = rand::thread_rng().gen();
            inner.push(if rand_value { DEAD } else { ALIVE })
        }
        result.push(inner);
    }

    result
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
