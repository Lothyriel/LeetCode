use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    get_char_freq(&s) == get_char_freq(&t)
}

fn get_char_freq(s: &str) -> HashMap<u8, u16> {
    s.bytes().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_default() += 1;
        acc
    })
}

