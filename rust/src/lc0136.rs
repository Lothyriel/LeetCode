pub fn single_number(nums: &[i32]) -> i32 {
    nums.iter().fold(0, |acc, n| acc ^ n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(single_number(&[2, 2, 1]), 1);
    }

    #[test]
    fn example_2() {
        assert_eq!(single_number(&[4, 1, 2, 1, 2]), 4);
    }

    #[test]
    fn example_3() {
        assert_eq!(single_number(&[1]), 1);
    }
}
