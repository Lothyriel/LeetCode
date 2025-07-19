use std::collections::BinaryHeap;

pub fn get_cost(quality: &[i32], wage: &[i32], k: usize) -> f64 {
    let mut workers: Vec<_> = quality
        .iter()
        .zip(wage)
        .map(|(q, w)| (q, *w as f64 / *q as f64))
        .collect();

    workers.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let mut k_workers = BinaryHeap::new();
    let mut total_quality = 0;
    let mut min_cost = f64::INFINITY;

    for (&q, r) in &workers {
        k_workers.push(q);
        total_quality += q;

        if k_workers.len() > k {
            total_quality -= k_workers.pop().unwrap();
        }

        if k_workers.len() == k {
            let cost = total_quality as f64 * r;
            min_cost = min_cost.min(cost);
        }
    }

    min_cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(get_cost(&[10, 20, 5], &[70, 50, 30], 2), 105.0);
    }

    #[test]
    fn example_2() {
        let result = get_cost(&[3, 1, 10, 10, 1], &[4, 8, 2, 2, 7], 3);
        let expected = 30.66667;
        let diff = result - expected;

        assert!(diff.abs() < 1e-5);
    }
}
