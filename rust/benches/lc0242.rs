use criterion::Criterion;

use leetcode::lc0242::{is_anagram_array, is_anagram_map};

pub fn bench(c: &mut Criterion) {
    c.bench_function("is_anagram map small", |b| {
        b.iter(|| is_anagram_map("listen".to_string(), "silent".to_string()))
    });

    c.bench_function("is_anagram map large", |b| {
        let s = "abcdefghijklmnopqrstuvwxyz".repeat(1000);

        let t: String = s.chars().rev().collect();

        b.iter(|| is_anagram_map(s.clone(), t.clone()))
    });

    c.bench_function("is_anagram array small", |b| {
        b.iter(|| is_anagram_array("listen".to_string(), "silent".to_string()))
    });

    c.bench_function("is_anagram array large", |b| {
        let s = "abcdefghijklmnopqrstuvwxyz".repeat(1000);

        let t: String = s.chars().rev().collect();

        b.iter(|| is_anagram_array(s.clone(), t.clone()))
    });
}
