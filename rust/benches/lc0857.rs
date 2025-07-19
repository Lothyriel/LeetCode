use criterion::Criterion;
use leetcode::lc0857::get_cost;

pub fn bench(c: &mut Criterion) {
    c.bench_function("get_cost e1", |b| {
        b.iter(|| get_cost(&[10, 20, 5], &[70, 50, 30], 2))
    });

    c.bench_function("get_cost e2", |b| {
        b.iter(|| get_cost(&[3, 1, 10, 10, 1], &[4, 8, 2, 2, 7], 3))
    });
}
