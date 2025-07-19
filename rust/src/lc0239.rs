use std::collections::VecDeque;

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut r = Vec::new();
    let mut q = VecDeque::new();

    for (i, &c) in nums.iter().enumerate() {
        if let Some(&p) = q.front() {
            if p as i32 <= i as i32 - k {
                q.pop_front();
            }
        }

        while let Some(&p) = q.back() {
            match nums[p] < c {
                true => {
                    q.pop_back();
                }
                false => break,
            }
        }

        q.push_back(i);

        if i >= k as usize - 1 {
            r.push(nums[*q.front().unwrap()]);
        }
    }

    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(
            max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
        assert_eq!(max_sliding_window(vec![1], 1), vec![1]);

        assert_eq!(max_sliding_window(vec![7, 2, 4], 2), vec![7, 4]);
    }
}
