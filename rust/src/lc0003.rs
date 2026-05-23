pub fn length_of_longest_substring(s: String) -> i32 {
    let bytes = s.as_bytes();

    let mut freq = [0; 127];

    let mut left = 0;
    let mut best = 0;

    for right in 0..bytes.len() {
        let idx = bytes[right] as usize;

        freq[idx] += 1;

        while freq[idx] > 1 {
            let left_idx = bytes[left] as usize;

            freq[left_idx] -= 1;
            left += 1;
        }

        best = best.max(right - left + 1);
    }

    best as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(3, length_of_longest_substring("abcabcbb".to_string()));
        assert_eq!(1, length_of_longest_substring("bbbbb".to_string()));
        assert_eq!(3, length_of_longest_substring("pwwkew".to_string()));
        assert_eq!(1, length_of_longest_substring(" ".to_string()));
    }
}
