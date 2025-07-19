pub fn maximum_time(time: String) -> String {
    let time = time.as_bytes();

    let substitutions = time.iter().enumerate().map(|x| match x {
        (0, b'?') if time[1] > b'3' && b'9' >= time[1] => b'1',
        (0, b'?') => b'2',
        (1, b'?') if time[0] == b'2' || time[0] == b'?' => b'3',
        (1, b'?') => b'9',
        (3, b'?') => b'5',
        (4, b'?') => b'9',
        (_, c) => *c,
    });

    String::from_utf8(substitutions.collect()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(maximum_time("2?:?0".to_owned()), "23:50".to_owned());
        assert_eq!(maximum_time("0?:3?".to_owned()), "09:39".to_owned());
        assert_eq!(maximum_time("1?:22".to_owned()), "19:22".to_owned());
    }
}
