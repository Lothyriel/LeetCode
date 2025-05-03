use std::collections::HashMap;

pub fn is_anagram_map(s: String, t: String) -> bool {
    get_char_freq(&s) == get_char_freq(&t)
}

pub fn is_anagram_array(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut freq = [0; 26];

    for (a, b) in s.bytes().zip(t.bytes()) {
        freq[(a - b'a') as usize] += 1;
        freq[(b - b'a') as usize] -= 1;
    }

    freq.iter().all(|&x| x == 0)
}

fn get_char_freq(s: &str) -> HashMap<u8, u16> {
    s.bytes().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_default() += 1;
        acc
    })
}
