use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

pub fn top_k_frequent(nums: &[i32], k: usize) -> Vec<i32> {
    let freq = nums
        .iter()
        .fold(HashMap::with_capacity(nums.len()), |mut acc, n| {
            *acc.entry(n).or_insert(0) += 1;
            acc
        });

    let mut heap = BinaryHeap::new();
    for (&&num, &count) in freq.iter() {
        heap.push(Reverse((count, num)));

        if heap.len() > k {
            heap.pop();
        }
    }

    heap.into_iter().map(|Reverse((_, n))| n).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(top_k_frequent(&[1, 1, 1, 2, 2, 3], 2), vec![2, 1]);
        assert_eq!(top_k_frequent(&[1], 1), vec![1]);
    }
}
