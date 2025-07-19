use criterion::*;

mod lc0242;
mod lc0857;

criterion_group!(benches, lc0242::bench, lc0857::bench);
criterion_main!(benches);
