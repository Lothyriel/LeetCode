use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut remainders = HashMap::with_capacity(nums.len() - 1);

    for (i, number) in nums.iter().enumerate() {
        if let Some(j) = remainders.get(number) {
            return vec![*j as i32, i as i32];
        }

        remainders.insert(target - number, i);
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use crate::lc0001::two_sum;

    #[test]
    fn tests() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
