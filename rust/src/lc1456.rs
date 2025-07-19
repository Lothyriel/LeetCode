pub fn max_vowels(s: String, k: i32) -> i32 {
    let s = s.as_bytes();

    s.iter()
        .enumerate()
        .fold((0, 0), |(max, count), (i, &c)| {
            let new_count = count + is_vowel(c);

            let new_count = if i >= k as usize {
                new_count - is_vowel(s[i - k as usize])
            } else {
                new_count
            };

            (max.max(new_count), new_count)
        })
        .0
}

fn is_vowel(char: u8) -> i32 {
    match char {
        b'a' | b'e' | b'i' | b'o' | b'u' => 1,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(max_vowels("abciiidef".to_owned(), 3), 3);
        assert_eq!(max_vowels("aeiou".to_owned(), 2), 2);
        assert_eq!(max_vowels("leetcode".to_owned(), 3), 2);
        assert_eq!(max_vowels("abba".to_owned(), 4), 2);
        assert_eq!(max_vowels("abba".to_owned(), 2), 1);
    }
}
