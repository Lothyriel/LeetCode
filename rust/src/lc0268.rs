pub fn missing_number(nums: Vec<i32>) -> i32 {
    let mut acc = 0;

    for n in &nums {
        acc ^= n;
    }

    for n in 0..=nums.len() as i32 {
        acc ^= n;
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(missing_number(vec![3, 0, 1]), 2);
        assert_eq!(missing_number(vec![0, 1]), 2);
        assert_eq!(missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }
}
