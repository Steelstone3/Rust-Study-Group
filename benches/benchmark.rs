use criterion::{black_box, criterion_group, criterion_main, Criterion};
use conways_game_of_life::{Grid, next_generation};
use conways_game_of_life::CellStatus::{ALIVE, DEAD};

pub fn criterion_benchmark(c: &mut Criterion) {
    let grid = Grid {
        cells: vec![
            vec![DEAD, DEAD, DEAD],
            vec![DEAD, ALIVE, ALIVE],
            vec![DEAD, ALIVE, ALIVE],
        ],
        height: 3,
        width: 3,
    };

    c.bench_function("next generation", |b| b.iter(|| next_generation(grid.clone())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
