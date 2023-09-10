pub fn maximum_time(time: String) -> String {
    let time_b = time.as_bytes();
    let indexes = time_b.iter().enumerate().filter(|(_, &c)| c == b'?');

    let substitutions = indexes
        .filter_map(|x| match x {
            (4, _) => Some((4, b'9')),
            (3, _) => Some((3, b'5')),
            (1, _) if time_b[0] == b'2' => Some((1, b'3')),
            (1, _) => Some((1, b'9')),
            (0, _) => Some((0, b'2')),
            _ => None,
        })
        .fold(time_b, |mut t, (i, c)| {
            t[i] = c;
            t
        });

    todo!()
}

#[cfg(test)]
mod tests {
    use crate::lc1736::maximum_time;

    #[test]
    fn tests() {
        assert_eq!(maximum_time("2?:?0".to_owned()), "23:50".to_owned());
        assert_eq!(maximum_time("0?:3?".to_owned()), "09:39".to_owned());
        assert_eq!(maximum_time("1?:22".to_owned()), "19:22".to_owned());
    }
}
